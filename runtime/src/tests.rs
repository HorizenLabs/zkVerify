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
    traits::{fungible::Inspect, Currency, ExistenceRequirement, OnInitialize, WithdrawReasons},
};
use frame_system::{EventRecord, Phase};
use pallet_settlement_fflonk::{Proof, FULL_PROOF_SIZE};
use sp_consensus_aura::{Slot, AURA_ENGINE_ID};
use sp_core::{Pair, Public};
use sp_runtime::{AccountId32, Digest, DigestItem};
use sp_staking::{offence, offence::ReportOffence, Exposure, SessionIndex};

mod testsfixtures;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: u8) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//test_seed{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

pub const SLOT_ID: u64 = 87; // Any random value should do
pub const NUM_VALIDATORS: u32 = 2;
pub const AURA_AUTHOR_ID: u32 = (SLOT_ID as u32) % NUM_VALIDATORS;

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
                        aura: get_from_seed::<AuraId>(user.session_key_seed),
                        grandpa: get_from_seed::<GrandpaId>(user.session_key_seed),
                        im_online: get_from_seed::<ImOnlineId>(user.session_key_seed),
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

    #[test]
    fn session_notifies_staking() {
        new_test_ext().execute_with(|| {
            let pre_staking_session = Staking::current_planned_session();
            Session::rotate_session();
            let post_staking_session = Staking::current_planned_session();
            assert_eq!(pre_staking_session + 1, post_staking_session);
        });
    }

    mod authorship {
        use super::*;
        const BLOCK_NUMBER: BlockNumber = 1;

        fn initialize() {
            let slot = Slot::from(SLOT_ID);
            let pre_digest = Digest {
                logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
            };
            System::reset_events();
            System::initialize(&BLOCK_NUMBER, &Default::default(), &pre_digest);
        }

        #[test]
        fn authorship_is_configured_with_aura() {
            new_test_ext().execute_with(|| {
                initialize();
                assert_eq!(
                    Authorship::author(),
                    Some(AccountId32::new(
                        testsfixtures::SAMPLE_USERS[AURA_AUTHOR_ID as usize]
                            .raw_account
                            .into()
                    ))
                );
            });
        }

        // Check that Authorship calls back on ImOnline
        #[test]
        fn authorship_notifies_imonline() {
            new_test_ext().execute_with(|| {
                initialize();
                assert!(!ImOnline::is_online(AURA_AUTHOR_ID));
                Authorship::on_initialize(BLOCK_NUMBER);
                assert!(ImOnline::is_online(AURA_AUTHOR_ID));
            });
        }

        #[test]
        fn authorship_notifies_staking() {
            new_test_ext().execute_with(|| {
                initialize();
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

    fn is_offender(session: SessionIndex, offender_account: &AccountId) -> bool {
        pallet_offences::ConcurrentReportsIndex::<Runtime>::get(
            b"im-online:offlin",
            session.encode(),
        )
        .into_iter()
        .any(|offender| {
            pallet_offences::Reports::<Runtime>::get(offender)
                .expect("Offence not found")
                .offender
                .0
                == *offender_account
        })
    }

    #[test]
    fn imonline_notifies_offences() {
        new_test_ext().execute_with(|| {
            let session = Session::current_index();
            let offender_account = AccountId32::new(
                testsfixtures::SAMPLE_USERS[AURA_AUTHOR_ID as usize]
                    .raw_account
                    .into(),
            );

            // Check that no previous offences were reported
            assert!(!is_offender(session, &offender_account));

            // AURA_AUTHOR_ID is considered offline
            assert!(!ImOnline::is_online(AURA_AUTHOR_ID));

            // Advance to next session
            System::set_block_number(System::block_number() + 1);
            Session::rotate_session();

            // Check that the offline offence for the last session was received by pallet_offences
            assert!(is_offender(session, &offender_account));
        });
    }

    pub const TEST_SLASH_FRACTION: Perbill = Perbill::one();
    struct TestOffence {
        offender_account: AccountId32,
    }
    impl offence::Offence<(AccountId32, Exposure<AccountId32, u128>)> for TestOffence {
        const ID: offence::Kind = *b"testoffencenooop";
        type TimeSlot = u128;

        fn offenders(&self) -> Vec<(AccountId32, Exposure<AccountId32, u128>)> {
            let exposure =
                pallet_staking::EraInfo::<Runtime>::get_full_exposure(0, &self.offender_account);

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
    fn offences_notifies_staking() {
        new_test_ext().execute_with(|| {
            let offender_account = sp_runtime::AccountId32::new(
                testsfixtures::SAMPLE_USERS[AURA_AUTHOR_ID as usize]
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
    fn payout_is_same_as_pow_coinbase() {
        new_test_ext().execute_with(|| {
            const POW_BLOCK_TIME_MILLIS: u64 = 150 * 1000;
            const POW_BLOCK_COINBASE: Balance = 625 * CENTS;
            assert_eq!(
                <Runtime as pallet_staking::Config>::EraPayout::era_payout(
                    0,
                    0,
                    POW_BLOCK_TIME_MILLIS
                ),
                (POW_BLOCK_COINBASE, 0)
            );
        });
    }
}
