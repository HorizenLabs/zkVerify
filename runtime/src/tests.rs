// Copyright 2024, The Horizen Foundation

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::*;

use codec::Encode;
use frame_support::{
    assert_ok,
    traits::{
        fungible::Inspect, Currency, EstimateNextNewSession, EstimateNextSessionRotation,
        ExistenceRequirement, OnInitialize, WithdrawReasons,
    },
};
use frame_system::{EventRecord, Phase};
use sp_consensus_babe::{Slot, BABE_ENGINE_ID};
use sp_core::crypto::VrfSecret;
use sp_core::{Pair, Public, H256};
use sp_runtime::{AccountId32, Digest, DigestItem};
use sp_staking::{offence, offence::ReportOffence, Exposure, SessionIndex};

mod testsfixtures;

fn get_from_seed<TPublic: Public>(seed: u8) -> TPublic::Pair {
    TPublic::Pair::from_string(&format!("//test_seed{}", seed), None)
        .expect("static values are valid; qed")
}

const NUM_VALIDATORS: u32 = 2;

/// The BABE epoch configuration at genesis.
const TEST_BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
    sp_consensus_babe::BabeEpochConfiguration {
        c: crate::PRIMARY_PROBABILITY,
        allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
    };

// Function used for creating the environment for the test.
// It must return a sp_io::TestExternalities, and the actual test will execute this one before running.
fn new_test_ext() -> sp_io::TestExternalities {
    // This builds the initial genesis storage for this test
    let mut t = frame_system::GenesisConfig::<super::Runtime>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<super::Runtime> {
        balances: testsfixtures::SAMPLE_USERS
            .to_vec()
            .into_iter()
            .map(|user| (user.raw_account.into(), user.starting_balance))
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_babe::GenesisConfig::<super::Runtime> {
        authorities: vec![],
        epoch_config: Some(TEST_BABE_GENESIS_EPOCH_CONFIG),
        ..Default::default()
    }
    .assimilate_storage(&mut t)
    .unwrap();

    // Add authorities
    pallet_session::GenesisConfig::<super::Runtime> {
        keys: testsfixtures::SAMPLE_USERS
            .to_vec()
            .into_iter()
            .map(|user| {
                (
                    user.raw_account.into(),
                    user.raw_account.into(),
                    SessionKeys {
                        babe: get_from_seed::<BabeId>(user.session_key_seed).public(),
                        grandpa: get_from_seed::<GrandpaId>(user.session_key_seed).public(),
                        im_online: get_from_seed::<ImOnlineId>(user.session_key_seed).public(),
                    },
                )
            })
            .take(NUM_VALIDATORS as usize)
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_staking::GenesisConfig::<super::Runtime> {
        stakers: testsfixtures::SAMPLE_USERS
            .to_vec()
            .into_iter()
            .map(|user| {
                (
                    user.raw_account.into(),
                    user.raw_account.into(),
                    testsfixtures::STASH_DEPOSIT,
                    sp_staking::StakerStatus::Validator::<AccountId>,
                )
            })
            .take(NUM_VALIDATORS as usize)
            .collect(),
        minimum_validator_count: NUM_VALIDATORS,
        validator_count: NUM_VALIDATORS,
        canceled_payout: 0,
        force_era: pallet_staking::Forcing::ForceNone,
        invulnerables: [].to_vec(),
        max_nominator_count: None,
        max_validator_count: None,
        min_nominator_bond: 1,
        min_validator_bond: testsfixtures::STASH_DEPOSIT,
        slash_reward_fraction: Perbill::zero(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::from(t);

    ext.execute_with(|| System::set_block_number(1));

    // Return the test externalities
    ext
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn check_starting_balances_and_existential_limit() {
    new_test_ext().execute_with(|| {
        // This creates a few public keys used to be converted to AccountId

        for sample_user in &testsfixtures::SAMPLE_USERS {
            assert_eq!(
                Balances::balance(&sample_user.raw_account.into()),
                sample_user.starting_balance
            );
        }

        // Now perform a withdraw on the fourth account, leaving its balance under the EXISTENTIAL_DEPOSIT limit
        // This should kill the account, when executed with the ExistenceRequirement::AllowDeath option
        let _id_3_withdraw = Balances::withdraw(
            &testsfixtures::SAMPLE_USERS[3].raw_account.into(),
            testsfixtures::EXISTENTIAL_DEPOSIT_REMAINDER, // Withdrawing more th
            WithdrawReasons::TIP,
            ExistenceRequirement::AllowDeath,
        );

        // Verify that the fourth account balance is now 0
        assert_eq!(
            Balances::balance(&testsfixtures::SAMPLE_USERS[3].raw_account.into()),
            0
        );
    });
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn pallet_fflonk_availability() {
    new_test_ext().execute_with(|| {
        let dummy_origin = AccountId32::new([0; 32]);
        let dummy_raw_proof: Proof = [0; FULL_PROOF_SIZE];
        assert!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            dummy_raw_proof.into()
        )
        .is_err());
        // just checking code builds, hence the pallet is available to the runtime
    });
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn pallet_poe_availability() {
    new_test_ext().execute_with(|| {
        assert_ok!(Poe::publish_attestation(RuntimeOrigin::root()));
        // just checking code builds, hence the pallet is available to the runtime
    });
}

mod pallets_interact {
    use super::*;

    // Any random values for these constants should do
    const BLOCK_NUMBER: BlockNumber = 1;
    const SLOT_ID: u64 = 87;
    const BABE_AUTHOR_ID: u32 = 1;

    // Initialize block #BLOCK_NUMBER, authored at slot SLOT_ID by BABE_AUTHOR_ID using Babe
    fn initialize() {
        let slot = Slot::from(SLOT_ID);
        let authority_index = BABE_AUTHOR_ID;
        let transcript = sp_consensus_babe::VrfTranscript::new(b"test", &[]);
        let pair: &sp_consensus_babe::AuthorityPair = &get_from_seed::<BabeId>(
            testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].session_key_seed,
        );
        let vrf_signature = pair.as_ref().vrf_sign(&transcript.into());
        let digest_data = sp_consensus_babe::digests::PreDigest::Primary(
            sp_consensus_babe::digests::PrimaryPreDigest {
                authority_index,
                slot,
                vrf_signature,
            },
        );
        let pre_digest = Digest {
            logs: vec![DigestItem::PreRuntime(BABE_ENGINE_ID, digest_data.encode())],
        };
        System::reset_events();
        System::initialize(&BLOCK_NUMBER, &Default::default(), &pre_digest);
        Babe::on_initialize(BLOCK_NUMBER);
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let mut ex = super::new_test_ext();
        ex.execute_with(|| initialize());
        ex
    }

    mod session {
        use super::*;

        #[test]
        fn uses_babe_session_length() {
            new_test_ext().execute_with(|| {
                assert_eq!(
                    Session::average_session_length(),
                    Babe::average_session_length()
                );
            });
        }

        #[test]
        fn notifies_staking() {
            new_test_ext().execute_with(|| {
                let pre_staking_session = Staking::current_planned_session();
                Session::rotate_session();
                let post_staking_session = Staking::current_planned_session();
                assert_eq!(pre_staking_session + 1, post_staking_session);
            });
        }
    }

    mod authorship {
        use super::*;

        #[test]
        fn is_configured_with_babe() {
            new_test_ext().execute_with(|| {
                assert_eq!(
                    Authorship::author(),
                    Some(AccountId32::new(
                        testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize]
                            .raw_account
                            .into()
                    ))
                );
            });
        }

        // Check that Authorship calls back on ImOnline
        #[test]
        fn notifies_imonline() {
            new_test_ext().execute_with(|| {
                assert!(!ImOnline::is_online(BABE_AUTHOR_ID));
                Authorship::on_initialize(BLOCK_NUMBER);
                assert!(ImOnline::is_online(BABE_AUTHOR_ID));
            });
        }

        #[test]
        fn notifies_staking() {
            new_test_ext().execute_with(|| {
                // Before authoring a block, no points have been given in the active era
                assert!(
                    Staking::eras_reward_points(
                        Staking::active_era().expect("No active era").index
                    )
                    .total
                        == 0
                );

                // Pretend we author a block
                Authorship::on_initialize(BLOCK_NUMBER);

                // Authoring a block notifies Staking, which results in a positive points balance
                assert!(
                    Staking::eras_reward_points(
                        Staking::active_era().expect("No active era").index
                    )
                    .total
                        > 0
                );
            });
        }
    }

    mod offences {
        use super::*;
        use sp_consensus_babe::digests::CompatibleDigestItem;
        use sp_runtime::generic::Header;
        use sp_runtime::traits::Header as _;

        type OffencesOpaqueTimeSlot = Vec<u8>;

        fn is_offender(
            time_slot: OffencesOpaqueTimeSlot,
            offender_account: &AccountId,
            offence: &[u8; 16],
        ) -> bool {
            pallet_offences::ConcurrentReportsIndex::<Runtime>::get(offence, time_slot)
                .into_iter()
                .any(|offender| {
                    pallet_offences::Reports::<Runtime>::get(offender)
                        .expect("Offence not found")
                        .offender
                        .0
                        == *offender_account
                })
        }

        const TEST_SLASH_FRACTION: Perbill = Perbill::one();
        struct TestOffence {
            offender_account: AccountId32,
        }
        impl offence::Offence<(AccountId32, Exposure<AccountId32, u128>)> for TestOffence {
            const ID: offence::Kind = *b"testoffencenooop";
            type TimeSlot = u128;

            fn offenders(&self) -> Vec<(AccountId32, Exposure<AccountId32, u128>)> {
                let exposure = pallet_staking::EraInfo::<Runtime>::get_full_exposure(
                    0,
                    &self.offender_account,
                );

                vec![(self.offender_account.clone(), exposure)]
            }
            fn validator_set_count(&self) -> u32 {
                NUM_VALIDATORS
            }
            fn time_slot(&self) -> Self::TimeSlot {
                0
            }
            fn session_index(&self) -> SessionIndex {
                0
            }
            fn slash_fraction(&self, _offenders_count: u32) -> Perbill {
                TEST_SLASH_FRACTION
            }
        }

        #[test]
        fn notifies_staking() {
            new_test_ext().execute_with(|| {
                let offender_account = sp_runtime::AccountId32::new(
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize]
                        .raw_account
                        .into(),
                );

                let expected_slashing_event = EventRecord {
                    phase: Phase::Initialization,
                    event: RuntimeEvent::Staking(pallet_staking::Event::SlashReported {
                        validator: offender_account.clone(),
                        fraction: TEST_SLASH_FRACTION,
                        slash_era: 0,
                    }),
                    topics: vec![],
                };

                // Make sure that no slash events for offender_account is published
                assert!(!System::events().contains(&expected_slashing_event));

                // Make pallet_offences report an offence
                let offence = TestOffence {
                    offender_account: offender_account.clone(),
                };
                assert_ok!(Offences::report_offence(vec![], offence));

                // Check that pallet_staking generates the related event (i.e. it has been notified of
                // the offence)
                assert!(System::events().contains(&expected_slashing_event));
            });
        }

        #[test]
        fn notified_by_imonline() {
            new_test_ext().execute_with(|| {
                let session = Session::current_index();
                let offender_account = AccountId32::new(
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize]
                        .raw_account
                        .into(),
                );

                const EQUIVOCATION_KIND: &offence::Kind = b"im-online:offlin";
                // Check that no previous offences were reported
                assert!(!is_offender(
                    session.encode(),
                    &offender_account,
                    EQUIVOCATION_KIND
                ));

                // BABE_AUTHOR_ID is considered offline
                assert!(!ImOnline::is_online(BABE_AUTHOR_ID));

                // Advance to next session w/o offender being online
                System::set_block_number(System::block_number() + 1);
                Session::rotate_session();

                // Check that the offline offence for the last session was received by pallet_offences
                assert!(is_offender(
                    session.encode(),
                    &offender_account,
                    EQUIVOCATION_KIND
                ));
            });
        }

        #[test]
        fn notified_by_grandpa() {
            new_test_ext().execute_with(|| {
                let offender_account = AccountId32::new(
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize]
                        .raw_account
                        .into(),
                );
                let offender = get_from_seed::<GrandpaId>(
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].session_key_seed,
                );

                const EQUIVOCATION_KIND: &offence::Kind = b"grandpa:equivoca";

                let round = 0;
                let set_id = Grandpa::current_set_id();
                let time_slot = pallet_grandpa::TimeSlot { set_id, round };

                // Make sure no previous reports for this offence/offender pair
                assert!(!is_offender(
                    time_slot.encode(),
                    &offender_account,
                    EQUIVOCATION_KIND
                ));

                // Make Grandpa report an offence for a double vote on different hashes for the
                // same target block in the same Grandpa round
                let target_number = BLOCK_NUMBER;
                let create_signed_prevote = |target_hash| {
                    let prevote = finality_grandpa::Prevote {
                        target_hash,
                        target_number,
                    };
                    let prevote_msg = finality_grandpa::Message::Prevote(prevote.clone());
                    let payload =
                        sp_consensus_grandpa::localized_payload(round, set_id, &prevote_msg);
                    let signed = offender.sign(&payload).into();
                    (prevote, signed)
                };
                let first_vote = create_signed_prevote(H256::random());
                let second_vote = create_signed_prevote(H256::random());
                let equivocation_proof = sp_consensus_grandpa::EquivocationProof::<H256, u32>::new(
                    set_id,
                    sp_consensus_grandpa::Equivocation::Prevote(finality_grandpa::Equivocation {
                        round_number: round,
                        identity: offender.public(),
                        first: first_vote,
                        second: second_vote,
                    }),
                );
                let key = (sp_consensus_grandpa::KEY_TYPE, &offender.public());
                let key_owner_proof = Historical::prove(key).unwrap();

                assert_ok!(Grandpa::report_equivocation_unsigned(
                    RuntimeOrigin::none(),
                    Box::new(equivocation_proof),
                    key_owner_proof,
                ));

                // Check report for this offence/offender pair has been received by pallet_offences
                assert!(is_offender(
                    time_slot.encode(),
                    &offender_account,
                    EQUIVOCATION_KIND
                ));
            });
        }

        #[test]
        fn notified_by_babe() {
            new_test_ext().execute_with(|| {
                let offender_account = AccountId32::new(
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize]
                        .raw_account
                        .into(),
                );
                let offender = get_from_seed::<BabeId>(
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].session_key_seed,
                );

                let seal_header = |mut header: Header<u32, BlakeTwo256>| {
                    let pre_hash = header.hash();
                    let seal = <DigestItem as CompatibleDigestItem>::babe_seal(
                        offender.sign(pre_hash.as_ref()),
                    );
                    header.digest_mut().push(seal);
                    header
                };

                // Produce two different block headers for the same height
                let h1 = seal_header(System::finalize());
                // Need to initialize again
                initialize();
                let h2 = seal_header(System::finalize());

                let slot = Slot::from(SLOT_ID);
                const EQUIVOCATION_KIND: &offence::Kind = b"babe:equivocatio";

                // Make sure no previous reports for this offence/offender pair
                assert!(!is_offender(
                    slot.encode(),
                    &offender_account,
                    EQUIVOCATION_KIND
                ));

                // Make Babe report the offence for authoring two different blocks for the same
                // target height
                let equivocation_proof = sp_consensus_babe::EquivocationProof {
                    slot,
                    offender: offender.public(),
                    first_header: h1,
                    second_header: h2,
                };
                let key = (sp_consensus_babe::KEY_TYPE, &offender.public());
                let key_owner_proof = Historical::prove(key).unwrap();

                assert_ok!(Babe::report_equivocation_unsigned(
                    RuntimeOrigin::none(),
                    Box::new(equivocation_proof),
                    key_owner_proof
                ));

                // Check report for this offence/offender pair has been received by pallet_offences
                assert!(is_offender(
                    slot.encode(),
                    &offender_account,
                    EQUIVOCATION_KIND
                ));
            });
        }
    }
}

/// This module tests the correct computation of rewards for validators.
mod payout {
    use pallet_staking::EraPayout;

    use crate::{Balance, Runtime, CENTS};

    use super::new_test_ext;

    /// Test that validators receive a cumulative reward that mimics the current emission of
    /// $ZEN in the PoW Horizen blockchain for miners, which is a coinbase of 6.25 Zen for
    /// each block every 2.5 minutes.
    #[test]
    fn is_same_as_pow_coinbase() {
        new_test_ext().execute_with(|| {
            const POW_BLOCK_TIME_MILLIS: u64 = 150 * 1000;
            const POW_BLOCK_COINBASE: Balance = 625 * CENTS;

            // Check the reward for an era lasting the target time.
            assert_eq!(
                <Runtime as pallet_staking::Config>::EraPayout::era_payout(
                    0,
                    0,
                    POW_BLOCK_TIME_MILLIS
                ),
                (POW_BLOCK_COINBASE, 0)
            );

            // Check the reward also for a smaller era (it should be proportional).
            assert_eq!(
                <Runtime as pallet_staking::Config>::EraPayout::era_payout(
                    0,
                    0,
                    POW_BLOCK_TIME_MILLIS / 10
                ),
                (POW_BLOCK_COINBASE / 10, 0)
            );

            // Check the reward also for an empty era.
            assert_eq!(
                <Runtime as pallet_staking::Config>::EraPayout::era_payout(0, 0, 0),
                (0, 0)
            );
        });
    }
}
