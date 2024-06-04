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

use frame_support::{derive_impl, weights::Weight};
use frame_system;
use rstest::fixture;
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

        pub fn new_proof_event<T: Config>(h: H256) -> Event<T> {
            Event::NewProof { value: h }
        }
    }
}

pub struct MockWeightInfo;

impl crate::weight::WeightInfo for MockWeightInfo {
    fn submit_proof_default() -> Weight {
        Weight::from_parts(1, 2)
    }

    fn submit_proof_with_vk() -> Weight {
        Weight::from_parts(3, 4)
    }

    fn submit_proof_with_vk_hash() -> Weight {
        Weight::from_parts(5, 6)
    }

    fn register_vk() -> Weight {
        Weight::from_parts(7, 8)
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
    type RuntimeEvent = RuntimeEvent;
    type OnProofVerified = OnProofVerifiedMock;
    type WeightInfo = MockWeightInfo;
}

impl on_proof_verified::pallet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
}

/// Build genesis storage according to the mock runtime.
#[fixture]
pub fn test_ext() -> sp_io::TestExternalities {
    let mut ext = sp_io::TestExternalities::from(
        frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap(),
    );
    ext.execute_with(|| System::set_block_number(1));
    ext
}
