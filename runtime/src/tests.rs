// Copyright 2024, Horizen Labs, Inc.

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
use frame_support::dispatch::GetDispatchInfo;
use frame_support::traits::schedule::DispatchTime;
use frame_support::traits::StorePreimage;
use frame_support::{
    assert_ok,
    traits::{
        fungible::Inspect, Currency, EstimateNextNewSession, EstimateNextSessionRotation,
        ExistenceRequirement, OnInitialize, WithdrawReasons,
    },
};
use frame_system::{EventRecord, Phase};
use pallet_conviction_voting::{AccountVote, Vote};
use pallet_verifiers::VkOrHash;
use sp_consensus_babe::{Slot, BABE_ENGINE_ID};
use sp_core::crypto::VrfSecret;
use sp_core::{Pair, Public, H256};
use sp_runtime::traits::Hash;
use sp_runtime::{AccountId32, Digest, DigestItem, MultiAddress};
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
            .iter()
            .cloned()
            .map(|user| (user.raw_account.into(), user.starting_balance))
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_babe::GenesisConfig::<super::Runtime> {
        authorities: vec![],
        epoch_config: TEST_BABE_GENESIS_EPOCH_CONFIG,
        ..Default::default()
    }
    .assimilate_storage(&mut t)
    .unwrap();

    // Add authorities
    pallet_session::GenesisConfig::<super::Runtime> {
        keys: testsfixtures::SAMPLE_USERS
            .iter()
            .cloned()
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
            .iter()
            .cloned()
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

    pallet_sudo::GenesisConfig::<super::Runtime> {
        key: Some(testsfixtures::SAMPLE_USERS[0].raw_account.into()),
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
        let dummy_proof: pallet_fflonk_verifier::Proof = [0; pallet_fflonk_verifier::PROOF_SIZE];
        let dummy_pubs: pallet_fflonk_verifier::Pubs = [0; pallet_fflonk_verifier::PUBS_SIZE];
        assert!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            VkOrHash::from_hash(H256::zero()),
            dummy_proof.into(),
            dummy_pubs.into(),
        )
        .is_err());
        // just checking code builds, hence the pallet is available to the runtime
    });
}

#[test]
fn pallet_multisig_availability() {
    new_test_ext().execute_with(|| {
        let issuer: AccountId32 = testsfixtures::SAMPLE_USERS[0].raw_account.into();
        let account_ids: Vec<_> = testsfixtures::SAMPLE_USERS
            .iter()
            .skip(1)
            .map(|u| u.raw_account.into())
            .collect();
        let call = Box::new(RuntimeCall::Balances(BalancesCall::transfer_allow_death {
            dest: MultiAddress::Id(issuer.clone()),
            value: 5000 * currency::ACME,
        }));
        assert_ok!(Multisig::as_multi(
            RuntimeOrigin::signed(issuer),
            2,
            account_ids,
            None,
            call,
            Weight::zero()
        ));
    })
}
#[test]
fn pallet_preimage_availability() {
    new_test_ext().execute_with(|| {
        assert_ok!(Preimage::note_preimage(
            RuntimeOrigin::root(),
            vec![0xCA, 0xFE, 0xBA, 0xBE]
        ));
    });
}

#[test]
fn pallet_scheduler_availability() {
    new_test_ext().execute_with(|| {
        let call = Box::new(RuntimeCall::Balances(BalancesCall::transfer_allow_death {
            dest: MultiAddress::Id(testsfixtures::SAMPLE_USERS[2].raw_account.into()),
            value: 5000 * currency::ACME,
        }));

        assert_ok!(Scheduler::schedule(
            RuntimeOrigin::root(),
            100,
            None,
            0,
            call
        ));
    });
}

fn aye(amount: Balance, conviction: u8) -> AccountVote<Balance> {
    let vote = Vote {
        aye: true,
        conviction: conviction.try_into().unwrap(),
    };
    AccountVote::Standard {
        vote,
        balance: amount,
    }
}
#[test]
fn pallet_referenda_and_conviction_voting_availability() {
    new_test_ext().execute_with(|| {
        let call = RuntimeCall::Balances(BalancesCall::transfer_allow_death {
            dest: MultiAddress::Id(testsfixtures::SAMPLE_USERS[1].raw_account.into()),
            value: 5000 * currency::ACME,
        });
        let proposal = <Preimage as StorePreimage>::bound(call).unwrap();

        let origin = RuntimeOrigin::signed(testsfixtures::SAMPLE_USERS[1].raw_account.into());
        let proposal_origin = Box::new(frame_system::RawOrigin::Root.into());
        let enactment_moment = DispatchTime::At(10);

        assert_ok!(Referenda::submit(
            origin,
            proposal_origin,
            proposal,
            enactment_moment
        ));

        let origin = RuntimeOrigin::signed(testsfixtures::SAMPLE_USERS[1].raw_account.into());
        assert_ok!(ConvictionVoting::vote(origin, 0, aye(10_u128, 0)));
    });
}

#[test]
fn pallet_whitelist_availability() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::root();
        let call = RuntimeCall::Balances(BalancesCall::transfer_allow_death {
            dest: MultiAddress::Id(testsfixtures::SAMPLE_USERS[1].raw_account.into()),
            value: 5000 * currency::ACME,
        });

        let encoded_call = call.encode();
        let call_hash = <Runtime as frame_system::Config>::Hashing::hash_of(&encoded_call);

        assert_ok!(Whitelist::whitelist_call(origin, call_hash));
    });
}

#[test]
fn pallet_treasury_availability() {
    new_test_ext().execute_with(|| {
        let asset_kind = Box::new(());
        let amount = 1000 * ACME;
        let beneficiary = Box::new(testsfixtures::SAMPLE_USERS[2].raw_account.into());
        let valid_from = None;

        let treasury_account = Treasury::account_id();
        let _ = Balances::make_free_balance_be(&treasury_account, 10000 * ACME);

        assert_ok!(Treasury::spend(
            RuntimeOrigin::root(),
            asset_kind,
            amount,
            beneficiary,
            valid_from
        ));
    });
}

#[test]
fn pallet_bounties_availability() {
    new_test_ext().execute_with(|| {
        let proposer = testsfixtures::SAMPLE_USERS[2].raw_account.into();
        let origin = RuntimeOrigin::signed(proposer);

        let value = 1000 * ACME;
        let description = vec![0; 100];

        assert_ok!(Bounties::propose_bounty(origin, value, description.clone()));
    });
}

#[test]
fn pallet_zksync_availability() {
    new_test_ext().execute_with(|| {
        let dummy_origin = AccountId32::new([0; 32]);
        let dummy_proof = [0; pallet_zksync_verifier::PROOF_SIZE];
        let dummy_pubs = [0; pallet_zksync_verifier::PUBS_SIZE];
        assert!(SettlementZksyncPallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            VkOrHash::from_hash(H256::zero()),
            dummy_proof.into(),
            dummy_pubs.into(),
        )
        .is_err());
        // just checking code builds, hence the pallet is available to the runtime
    });
}

#[test]
fn pallet_groth16_availability() {
    new_test_ext().execute_with(|| {
        let dummy_origin = AccountId32::new([0; 32]);
        assert!(SettlementGroth16Pallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            VkOrHash::from_hash(H256::zero()),
            pallet_groth16_verifier::Proof::default().into(),
            Box::new(Vec::new()),
        )
        .is_err());
        // just checking code builds, hence the pallet is available to the runtime
    });
}

#[test]
fn pallet_risc0_availability() {
    new_test_ext().execute_with(|| {
        let dummy_origin = AccountId32::new([0; 32]);

        let dummy_vk = H256::default();
        let dummy_proof = vec![];
        let dummy_pubs = vec![];

        assert!(SettlementRisc0Pallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            VkOrHash::Vk(dummy_vk.into()),
            dummy_proof.into(),
            dummy_pubs.into()
        )
        .is_err());
        // just checking code builds, hence the pallet is available to the runtime
    });
}

#[test]
fn pallet_ultraplonk_availability() {
    new_test_ext().execute_with(|| {
        let dummy_origin = AccountId32::new([0; 32]);

        let dummy_vk = [0; pallet_ultraplonk_verifier::VK_SIZE];
        let dummy_proof = vec![0; pallet_ultraplonk_verifier::PROOF_SIZE];
        let dummy_pubs = Vec::new();

        assert!(SettlementUltraplonkPallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            VkOrHash::Vk(dummy_vk.into()),
            dummy_proof.into(),
            dummy_pubs.into()
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

mod use_correct_weights {
    use crate::{OnChainSeqPhragmen, Runtime};

    #[test]
    fn frame_system() {
        use frame_system::WeightInfo;

        assert_eq!(
            <Runtime as frame_system::Config>::SystemWeightInfo::set_heap_pages(),
            crate::weights::frame_system::ZKVWeight::<Runtime>::set_heap_pages()
        );
    }

    #[test]
    fn db() {
        assert_eq!(
            <Runtime as frame_system::Config>::DbWeight::get(),
            crate::weights::db::constants::RocksDbWeight::get()
        );
    }

    #[test]
    fn pallet_babe() {
        use pallet_babe::WeightInfo;

        assert_eq!(
            <Runtime as pallet_babe::Config>::WeightInfo::report_equivocation(42, 42),
            crate::weights::pallet_babe::ZKVWeight::<Runtime>::report_equivocation(42, 42)
        );
    }

    #[test]
    fn pallet_grandpa() {
        use pallet_grandpa::WeightInfo;

        assert_eq!(
            <Runtime as pallet_grandpa::Config>::WeightInfo::report_equivocation(42, 42),
            crate::weights::pallet_grandpa::ZKVWeight::<Runtime>::report_equivocation(42, 42)
        );
    }

    #[test]
    fn pallet_balances() {
        use pallet_balances::WeightInfo;

        assert_eq!(
            <Runtime as pallet_balances::Config>::WeightInfo::transfer_allow_death(),
            crate::weights::pallet_balances::ZKVWeight::<Runtime>::transfer_allow_death()
        );
    }

    #[test]
    fn pallet_session() {
        use pallet_session::WeightInfo;

        assert_eq!(
            <Runtime as pallet_session::Config>::WeightInfo::set_keys(),
            crate::weights::pallet_session::ZKVWeight::<Runtime>::set_keys()
        );
    }

    #[test]
    fn frame_election_provider_support() {
        use frame_election_provider_support::WeightInfo;

        assert_eq!(
            <OnChainSeqPhragmen as frame_election_provider_support::onchain::Config>::WeightInfo::phragmen(42, 42, 42),
            crate::weights::frame_election_provider_support::ZKVWeight::<Runtime>::phragmen(42, 42, 42)
        );
    }

    #[test]
    fn pallet_sudo() {
        use pallet_sudo::WeightInfo;

        assert_eq!(
            <Runtime as pallet_sudo::Config>::WeightInfo::sudo(),
            crate::weights::pallet_sudo::ZKVWeight::<Runtime>::sudo()
        );
    }

    #[test]
    fn pallet_multisig() {
        use pallet_multisig::WeightInfo;

        assert_eq!(
            <Runtime as pallet_multisig::Config>::WeightInfo::as_multi_approve(3, 100),
            crate::weights::pallet_multisig::ZKVWeight::<Runtime>::as_multi_approve(3, 100)
        );
    }

    #[test]
    fn pallet_preimage() {
        use pallet_preimage::WeightInfo;

        assert_eq!(
            <Runtime as pallet_preimage::Config>::WeightInfo::note_preimage(100),
            crate::weights::pallet_preimage::ZKVWeight::<Runtime>::note_preimage(100)
        );
    }

    #[test]
    fn pallet_scheduler() {
        use pallet_scheduler::WeightInfo;

        assert_eq!(
            <Runtime as pallet_scheduler::Config>::WeightInfo::schedule(10),
            crate::weights::pallet_scheduler::ZKVWeight::<Runtime>::schedule(10)
        );
    }

    #[test]
    fn pallet_referenda() {
        use pallet_referenda::WeightInfo;

        assert_eq!(
            <Runtime as pallet_referenda::Config>::WeightInfo::submit(),
            crate::weights::pallet_referenda::ZKVWeight::<Runtime>::submit()
        );
    }

    #[test]
    fn pallet_whitelist() {
        use pallet_whitelist::WeightInfo;

        assert_eq!(
            <Runtime as pallet_whitelist::Config>::WeightInfo::whitelist_call(),
            crate::weights::pallet_whitelist::ZKVWeight::<Runtime>::whitelist_call()
        );
    }

    #[test]
    fn pallet_conviction_voting() {
        use pallet_conviction_voting::WeightInfo;

        assert_eq!(
            <Runtime as pallet_conviction_voting::Config>::WeightInfo::vote_new(),
            crate::weights::pallet_conviction_voting::ZKVWeight::<Runtime>::vote_new()
        );
    }

    #[test]
    fn pallet_treasury() {
        use pallet_treasury::WeightInfo;

        assert_eq!(
            <Runtime as pallet_treasury::Config>::WeightInfo::check_status(),
            crate::weights::pallet_treasury::ZKVWeight::<Runtime>::check_status()
        );
    }

    #[test]
    fn pallet_bounties() {
        use pallet_bounties::WeightInfo;

        assert_eq!(
            <Runtime as pallet_bounties::Config>::WeightInfo::propose_curator(),
            crate::weights::pallet_bounties::ZKVWeight::<Runtime>::propose_curator()
        );
    }

    #[test]
    fn pallet_timestamp() {
        use pallet_timestamp::WeightInfo;

        assert_eq!(
            <Runtime as pallet_timestamp::Config>::WeightInfo::set(),
            crate::weights::pallet_timestamp::ZKVWeight::<Runtime>::set()
        );
    }

    #[test]
    fn pallet_im_online() {
        use pallet_im_online::WeightInfo;

        assert_eq!(
            <Runtime as pallet_im_online::Config>::WeightInfo::validate_unsigned_and_then_heartbeat(42),
            crate::weights::pallet_im_online::ZKVWeight::<Runtime>::validate_unsigned_and_then_heartbeat(42)
        );
    }

    #[test]
    fn pallet_fflonk_verifier() {
        use pallet_fflonk_verifier::Fflonk;
        let dummy_proof = [0; pallet_fflonk_verifier::PROOF_SIZE];
        let dummy_pubs = [0; pallet_fflonk_verifier::PUBS_SIZE];
        use pallet_fflonk_verifier::WeightInfo;

        assert_eq!(
            <<Runtime as pallet_verifiers::Config<Fflonk>>::WeightInfo as pallet_verifiers::WeightInfo<Fflonk>>::submit_proof(
                &dummy_proof,
                &dummy_pubs
            ),
            crate::weights::pallet_fflonk_verifier::ZKVWeight::<Runtime>::submit_proof()
        );
    }

    #[test]
    fn pallet_zksync_verifier() {
        use pallet_zksync_verifier::Zksync;
        let dummy_proof = [0; pallet_zksync_verifier::PROOF_SIZE];
        let dummy_pubs = [0; pallet_zksync_verifier::PUBS_SIZE];
        use pallet_zksync_verifier::WeightInfo;

        assert_eq!(
            <<Runtime as pallet_verifiers::Config<Zksync>>::WeightInfo as pallet_verifiers::WeightInfo<Zksync>>::submit_proof(
                &dummy_proof,
                &dummy_pubs
            ),
            crate::weights::pallet_zksync_verifier::ZKVWeight::<Runtime>::submit_proof()
        );
    }

    #[test]
    fn pallet_groth16_verifier() {
        use pallet_groth16_verifier::Groth16;
        use pallet_groth16_verifier::WeightInfo;

        assert_eq!(
            <<Runtime as pallet_verifiers::Config<Groth16<Runtime>>>::WeightInfo as
                pallet_verifiers::WeightInfo<Groth16<Runtime>>>
                ::submit_proof(
                &pallet_groth16_verifier::Proof::default(),
                &Vec::new()
            ),
            crate::weights::pallet_groth16_verifier::ZKVWeight::<Runtime>::submit_proof_bn254(0)
        );
    }

    #[test]
    fn pallet_settlement_risc0() {
        use pallet_risc0_verifier::Risc0;
        use pallet_risc0_verifier::WeightInfo;

        assert_eq!(
            <<Runtime as pallet_verifiers::Config<Risc0<Runtime>>>::WeightInfo as
                pallet_verifiers::WeightInfo<Risc0<Runtime>>>
                ::submit_proof(
                &Vec::new(),
                &Vec::new()
            ),
            crate::weights::pallet_risc0_verifier::ZKVWeight::<Runtime>::submit_proof_cycle_2_pow_13()
        );
    }

    #[test]
    fn pallet_settlement_ultraplonk() {
        use pallet_ultraplonk_verifier::{Ultraplonk, WeightInfo};

        assert_eq!(
            <<Runtime as pallet_verifiers::Config<Ultraplonk<Runtime>>>::WeightInfo as
                pallet_verifiers::WeightInfo<Ultraplonk<Runtime>>>
                ::submit_proof(
                &vec![0; pallet_ultraplonk_verifier::PROOF_SIZE],
                &Vec::new()
            ),
            crate::weights::pallet_ultraplonk_verifier::ZKVWeight::<Runtime>::submit_proof_32()
        );
    }

    #[test]
    fn pallet_poe() {
        use pallet_poe::WeightInfo;

        assert_eq!(
            <Runtime as pallet_poe::Config>::WeightInfo::publish_attestation(),
            crate::weights::pallet_poe::ZKVWeight::<Runtime>::publish_attestation()
        );
    }
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
        ex.execute_with(initialize);
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
                        testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].raw_account
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
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].raw_account,
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
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].raw_account,
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
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].raw_account,
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
                    let signed = offender.sign(&payload);
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
                    testsfixtures::SAMPLE_USERS[BABE_AUTHOR_ID as usize].raw_account,
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

    mod sudo {
        use super::*;

        #[test]
        fn and_multisig() {
            new_test_ext().execute_with(|| {
                // Extract the account IDs from SAMPLE_USERS
                let account_ids: Vec<AccountId32> = testsfixtures::SAMPLE_USERS
                    .iter()
                    .map(|user| AccountId32::from(user.raw_account))
                    .collect();
                let multi = Multisig::multi_account_id(&account_ids[..3], 2);

                assert_ok!(Balances::transfer_allow_death(
                    RuntimeOrigin::signed(account_ids[0].clone()),
                    MultiAddress::Id(multi.clone()),
                    1 * currency::ACME
                ));

                // Setting the multisig account as the new sudo account
                assert_ok!(Sudo::set_key(
                    RuntimeOrigin::signed(account_ids[0].clone()),
                    MultiAddress::Id(multi.clone())
                ));

                // Prepare a sudo call to change the sudo key again to a new account (account_ids[1])
                let sudo_call = pallet_sudo::Call::set_key {
                    new: MultiAddress::Id(account_ids[1].clone()),
                };

                let sudo_call_weight = sudo_call.get_dispatch_info().weight;

                // First part of multisig approval (propose the sudo call)
                assert_ok!(Multisig::as_multi(
                    RuntimeOrigin::signed(account_ids[0].clone()),
                    2,
                    vec![account_ids[1].clone(), account_ids[2].clone()],
                    None,
                    Box::new(RuntimeCall::Sudo(sudo_call.clone())),
                    Weight::zero()
                ));
                // Second part of multisig approval (approve the sudo call)
                assert_ok!(Multisig::as_multi(
                    RuntimeOrigin::signed(account_ids[1].clone()),
                    2,
                    vec![account_ids[0].clone(), account_ids[2].clone()],
                    Some(Multisig::timepoint()),
                    Box::new(RuntimeCall::Sudo(sudo_call.clone())),
                    sudo_call_weight
                ));

                // Ensure the sudo key has been updated correctly to account_ids[1]
                assert_ok!(Sudo::remove_key(RuntimeOrigin::signed(
                    account_ids[1].clone()
                )));
            });
        }
    }

    mod scheduler {
        use super::*;
        use frame_support::traits::QueryPreimage;
        use sp_runtime::traits::Hash;

        #[test]
        fn uses_preimage() {
            new_test_ext().execute_with(|| {
                // We need a call bigger than 128 bytes to trigger preimage usage
                let call = Box::new(
                    RuntimeCall::SettlementFFlonkPallet(pallet_verifiers::Call::<
                        Runtime,
                        pallet_fflonk_verifier::Fflonk,
                    >::new_call_variant_submit_proof(
                        VkOrHash::from_hash(H256::zero()),
                        [0; pallet_fflonk_verifier::PROOF_SIZE].into(),
                        [0; pallet_fflonk_verifier::PUBS_SIZE].into(),
                    )),
                );
                let call_hash = <Runtime as frame_system::Config>::Hashing::hash_of(&call);

                assert_ok!(Scheduler::schedule(
                    RuntimeOrigin::root(),
                    100,
                    None,
                    0,
                    call
                ));

                assert!(Preimage::len(&call_hash).is_some());
            });
        }
    }
}

/// This module tests the correct computation of rewards for validators.
mod payout {
    use pallet_staking::EraPayout;

    use crate::{Balance, Runtime};

    use super::new_test_ext;

    #[test]
    fn check_era_rewards() {
        new_test_ext().execute_with(|| {
            const ERA_DURATION_MILLIS: u64 = 6 * 60 * 60 * 1000;
            const TOTAL_STAKED: Balance = 900000000;
            const TOTAL_ISSUANCE: Balance = 1000000000;

            // Check the reward for an empty era.
            assert_eq!(
                <Runtime as pallet_staking::Config>::EraPayout::era_payout(
                    0,
                    0,
                    ERA_DURATION_MILLIS
                ),
                (0u128, 0)
            );

            // Check the reward for a normal era
            assert_eq!(
                <Runtime as pallet_staking::Config>::EraPayout::era_payout(
                    TOTAL_STAKED,
                    TOTAL_ISSUANCE,
                    ERA_DURATION_MILLIS
                ),
                (17313, 51133)
            );
        });
    }
}
