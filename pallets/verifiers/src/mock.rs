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
#![cfg(test)]

use frame_support::{derive_impl, weights::Weight};
use sp_runtime::{traits::IdentityLookup, BuildStorage};

use hp_verifiers::{Verifier, VerifyError, WeightInfo};

/// A on_proof_verifier fake pallet
pub mod on_proof_verified {
    pub use pallet::*;

    #[frame_support::pallet]
    #[allow(unused_imports)]
    mod pallet {
        use frame_support::pallet_prelude::*;
        use sp_core::H256;

        use hp_attestation::OnProofVerified;

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

pub mod fake_pallet {
    use super::*;

    /// - Accept Proof iff proof == pubs and vk != 0.
    /// - If vk == 0 the vk is invalid and raise InvalidVerificationKey
    /// - If proof == 0 the proof is invalid and raise InvalidProofData
    /// - If pubs == 0 pubs are invalid raise InvalidInput
    /// - Otherwise
    ///     - proof != pubs the proof raise a VerifyError
    ///
    #[crate::verifier]
    pub struct FakeVerifier;

    impl FakeVerifier {
        pub fn malformed_vk() -> Box<<Self as Verifier>::Vk> {
            Box::new(0)
        }

        pub fn malformed_proof() -> Box<<Self as Verifier>::Proof> {
            Box::new(0)
        }

        pub fn malformed_pubs() -> Box<<Self as Verifier>::Pubs> {
            Box::new(0)
        }
    }

    impl Verifier for FakeVerifier {
        type Proof = u64;

        type Pubs = u64;

        type Vk = u64;

        fn hash_context_data() -> &'static [u8] {
            b"fake"
        }

        fn verify_proof(
            vk: &Self::Vk,
            proof: &Self::Proof,
            pubs: &Self::Pubs,
        ) -> Result<(), VerifyError> {
            match (*vk, *proof, *pubs) {
                (0, _, _) => Err(VerifyError::InvalidVerificationKey),
                (_, 0, _) => Err(VerifyError::InvalidProofData),
                (_, _, 0) => Err(VerifyError::InvalidInput),
                (_vk, proof, pubs) if proof == pubs => Ok(()),
                _ => Err(VerifyError::VerifyError),
            }
        }

        fn validate_vk(vk: &Self::Vk) -> Result<(), VerifyError> {
            if *vk == 0 {
                Err(VerifyError::InvalidVerificationKey)
            } else {
                Ok(())
            }
        }
        fn pubs_bytes(pubs: &Self::Pubs) -> sp_std::borrow::Cow<[u8]> {
            sp_std::borrow::Cow::Owned(pubs.to_be_bytes().into())
        }
    }
}

pub use fake_pallet::FakeVerifier;
pub struct MockWeightInfo;
impl WeightInfo<FakeVerifier> for MockWeightInfo {
    fn submit_proof(_proof: &u64, _pubs: &u64) -> Weight {
        Weight::from_parts(1, 2)
    }

    fn submit_proof_with_vk_hash(_proof: &u64, _pubs: &u64) -> Weight {
        Weight::from_parts(3, 4)
    }

    fn register_vk(_vk: &u64) -> Weight {
        Weight::from_parts(5, 6)
    }
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        FakeVerifierPallet: fake_pallet,
        OnProofVerifiedMock: on_proof_verified,
    }
);

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type Block = frame_system::mocking::MockBlockU32<Test>;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
}

impl crate::Config<FakeVerifier> for Test {
    type RuntimeEvent = RuntimeEvent;
    type OnProofVerified = OnProofVerifiedMock;
    type WeightInfo = MockWeightInfo;
}

impl on_proof_verified::Config for Test {
    type RuntimeEvent = RuntimeEvent;
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
