//! Here we write the integration tests that just check pallets weighs are correctly linked.

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
fn pallet_utility() {
    use pallet_utility::WeightInfo;

    assert_eq!(
        <Runtime as pallet_utility::Config>::WeightInfo::dispatch_as(),
        crate::weights::pallet_utility::ZKVWeight::<Runtime>::dispatch_as()
    );
}

#[test]
fn pallet_vesting() {
    use pallet_vesting::WeightInfo;

    assert_eq!(
        <Runtime as pallet_vesting::Config>::WeightInfo::force_vested_transfer(1, 2),
        crate::weights::pallet_vesting::ZKVWeight::<Runtime>::force_vested_transfer(1, 2)
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

#[test]
fn pallet_bags_list() {
    use pallet_bags_list::WeightInfo;

    assert_eq!(
        <Runtime as pallet_bags_list::Config<pallet_bags_list::Instance1>>::WeightInfo::put_in_front_of(),
        crate::weights::pallet_bags_list::ZKVWeight::<Runtime>::put_in_front_of()
    );
}