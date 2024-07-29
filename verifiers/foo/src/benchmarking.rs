#![cfg(feature = "runtime-benchmarks")]

use super::Foo;
use frame_benchmarking::v2::*;
pub use frame_support::parameter_types;
use frame_system::RawOrigin;
use hp_verifiers::Verifier;
use pallet_verifiers::{VkOrHash, Vks};

pub struct Pallet<T: Config>(crate::Pallet<T>);

pub trait Config: crate::Config {}
impl<T: crate::Config> Config for T {}
pub type Call<T> = pallet_verifiers::Call<T, Foo<T>>;

include!("resources.rs");

#[benchmarks(where T: pallet_verifiers::Config<Foo<T>>)]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn submit_proof() {
        // setup code
        let caller = whitelisted_caller();
        let vk = VALID_VK;
        let proof = VALID_PROOF;
        let pubs = VALID_PUBS;

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_vk(vk),
            proof.into(),
            pubs.into(),
        );
    }

    #[benchmark]
    fn submit_proof_with_vk_hash() {
        // setup code
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = VALID_PROOF;
        let pubs = VALID_PUBS;
        Vks::<T, Foo<T>>::insert(VALID_VK, VALID_VK);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof.into(), pubs.into());
    }

    #[benchmark]
    fn register_vk() {
        // setup code
        let caller = whitelisted_caller();
        let vk = VALID_VK;

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk.clone().into());

        // Verify
        assert!(Vks::<T, Foo<T>>::get(Foo::<T>::vk_hash(&vk)).is_some());
    }

    impl_benchmark_test_suite!(Pallet, super::mock::test_ext(), super::mock::Test);
}

#[cfg(test)]
mod mock {
    use frame_support::derive_impl;
    use sp_core::ConstU8;
    use sp_runtime::{traits::IdentityLookup, BuildStorage};

    // Configure a mock runtime to test the pallet.
    frame_support::construct_runtime!(
        pub enum Test
        {
            System: frame_system,
            VerifierPallet: crate,
        }
    );

    pub const SOME_PARAMETER: u8 = 1; // arbitrary value

    impl crate::Config for Test {
        type SomeParameter = ConstU8<SOME_PARAMETER>; // arbitrary value
    }

    #[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
    impl frame_system::Config for Test {
        type Block = frame_system::mocking::MockBlockU32<Test>;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
    }

    impl pallet_verifiers::Config<crate::Foo<Test>> for Test {
        type RuntimeEvent = RuntimeEvent;
        type OnProofVerified = ();
        type WeightInfo = crate::FooWeight<()>;
    }

    /// Build genesis storage according to the mock runtime.
    pub fn test_ext() -> sp_io::TestExternalities {
        let mut ext = sp_io::TestExternalities::from(
            frame_system::GenesisConfig::<Test>::default()
                .build_storage()
                .unwrap(),
        );
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
