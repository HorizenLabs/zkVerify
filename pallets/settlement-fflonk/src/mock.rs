use frame_support::derive_impl;
use frame_system;
use sp_runtime::{traits::IdentityLookup, BuildStorage};

pub mod on_proof_verified {
    pub use pallet::*;

    #[frame_support::pallet]
    pub mod pallet {
        use frame_support::pallet_prelude::*;
        use sp_core::H256;

        use crate::OnProofVerified;

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

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        SettlementFFlonkPallet: crate,
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
