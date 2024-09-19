#![cfg(feature = "runtime-benchmarks")]

use super::ProofOfSql;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use hp_verifiers::Verifier;
use pallet_verifiers::{VkOrHash, Vks};

pub struct Pallet<T: Config>(crate::Pallet<T>);

pub trait Config: crate::Config {}
impl<T: crate::Config> Config for T {}
pub type Call<T> = pallet_verifiers::Call<T, ProofOfSql<T>>;

#[benchmarks(where T: pallet_verifiers::Config<ProofOfSql<T>>)]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn submit_proof() {
        // setup code
        let caller = whitelisted_caller();
        let vk = include_bytes!("resources_benchmarking/VALID_VK.bin").to_vec();
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF.bin").to_vec();
        let pubs = include_bytes!("resources_benchmarking/VALID_PUBS.bin").to_vec();

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_vk(vk.into()),
            proof.into(),
            pubs.into(),
        );
    }

    #[benchmark]
    fn submit_proof_with_vk_hash() {
        // setup code
        let caller = whitelisted_caller();
        let vk_hash = sp_core::H256::repeat_byte(2);
        let vk: crate::Vk<T> = include_bytes!("resources_benchmarking/VALID_VK.bin")
            .to_vec()
            .into();
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF.bin").to_vec();
        let pubs = include_bytes!("resources_benchmarking/VALID_PUBS.bin").to_vec();
        Vks::<T, ProofOfSql<T>>::insert(vk_hash, vk);

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_hash(vk_hash),
            proof.into(),
            pubs.into(),
        );
    }

    #[benchmark]
    fn register_vk() {
        // setup code
        let caller = whitelisted_caller();
        let vk: crate::Vk<T> = include_bytes!("resources_benchmarking/VALID_VK.bin")
            .to_vec()
            .into();

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk.clone().into());

        // Verify
        assert!(Vks::<T, ProofOfSql<T>>::get(ProofOfSql::<T>::vk_hash(&vk)).is_some());
    }

    impl_benchmark_test_suite!(Pallet, super::mock::test_ext(), super::mock::Test);
}
