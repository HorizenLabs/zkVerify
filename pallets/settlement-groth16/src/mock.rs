use frame_support::{derive_impl, weights::Weight};
use frame_system;
use sp_runtime::{traits::IdentityLookup, BuildStorage};

pub mod on_proof_verified {
    pub use pallet::*;

    #[frame_support::pallet]
    pub mod pallet {
        use frame_support::pallet_prelude::*;
        use sp_core::H256;

        use hp_poe::OnProofVerified;

        #[pallet::pallet]
        pub struct Pallet<T>(_);

        #[pallet::config]
        pub trait Config: frame_system::Config {
            type RuntimeEvent: From<Event<Self>>
                + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        }

        #[pallet::event]
        #[pallet::generate_deposit(pub(super) fn deposit_event)]
        pub enum Event<T: Config> {
            NewProof { value: H256 },
        }

        impl<T: Config> OnProofVerified for Pallet<T> {
            fn on_proof_verified(value: H256) {
                Self::deposit_event(Event::NewProof { value });
            }
        }
    }
}

pub struct MockWeightInfo;

impl MockWeightInfo {
    pub const REF_TIME: u64 = 42;
    pub const PROOF_SIZE: u64 = 24;
    pub const REF_INPUT_TIME: u64 = 3;
}

impl crate::weight::WeightInfo for MockWeightInfo {
    fn submit_proof_bn254(n: u32) -> Weight {
        Weight::from_parts(Self::REF_TIME, Self::PROOF_SIZE)
            .saturating_add(Weight::from_parts(Self::REF_INPUT_TIME, 0).saturating_mul(n.into()))
    }

    fn submit_proof_bls12_381(n: u32) -> Weight {
        Weight::from_parts(Self::REF_TIME, Self::PROOF_SIZE)
            .saturating_add(Weight::from_parts(Self::REF_INPUT_TIME, 0).saturating_mul(n.into()))
    }
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        SettlementGroth16Pallet: crate,
        OnProofVerifiedMock: on_proof_verified,
    }
);

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type Block = frame_system::mocking::MockBlockU32<Test>;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
}

impl crate::Config for Test {
    type OnProofVerified = OnProofVerifiedMock;
    type WeightInfo = MockWeightInfo;
    const MAX_NUM_INPUTS: usize = 16;
}

impl on_proof_verified::pallet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut ext = sp_io::TestExternalities::from(
        frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap(),
    );
    ext.execute_with(|| System::set_block_number(1));
    ext
}
