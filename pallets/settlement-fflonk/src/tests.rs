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

use hex_literal::hex;

use sp_core::H256;

use super::Proof;
use crate::mock;
use crate::mock::*;

include!("proof.rs");
pub static VALID_HASH: [u8; 32] =
    hex!("337d23faf65147cd3a2cc495aac5cfbe44fe55b17c83990f43f3e5663b0ff248");

#[test]
fn valid_proof_passes_verification_and_is_notified() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert!(
            SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), VALID_PROOF.into())
                .is_ok()
        );

        let events = mock::System::events();
        assert_eq!(events.len(), 1);

        mock::System::assert_last_event(
            crate::mock::on_proof_verified::pallet::Event::NewProof {
                value: H256(VALID_HASH),
            }
            .into(),
        );
    });
}

#[test]
fn malformed_proof_fails_verification_and_is_not_notified() {
    new_test_ext().execute_with(|| {
        let mut malformed_proof: Proof = VALID_PROOF;
        // first byte changed from '0x17' to '0x07' (raw proof data)
        malformed_proof[0] = 0x07;

        // Dispatch a signed extrinsic.
        assert!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(1),
            malformed_proof.into()
        )
        .is_err());

        let events = mock::System::events();
        assert_eq!(events.len(), 0);
    });
}

#[test]
fn invalid_proof_fails_verification_and_is_not_notified() {
    new_test_ext().execute_with(|| {
        let mut invalid_proof: Proof = VALID_PROOF;
        // last byte changed from '0x06' to '0x00' (public inputs)
        invalid_proof[invalid_proof.len() - 1] = 0x00;

        // Dispatch a signed extrinsic.
        assert!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(1),
            invalid_proof.into()
        )
        .is_err());

        let events = mock::System::events();
        assert_eq!(events.len(), 0);
    });
}

mod another_way_of_testing {
    use hp_poe::OnProofVerified;

    use super::*;
    use frame_support::derive_impl;
    use frame_system;
    use sp_runtime::{traits::IdentityLookup, BuildStorage};

    pub struct Crash {}

    // Configure a mock runtime to test the pallet.
    frame_support::construct_runtime!(
        pub enum Test
        {
            System: frame_system,
            SettlementFFlonkPallet: crate,
        }
    );

    #[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
    impl frame_system::Config for Test {
        type Block = frame_system::mocking::MockBlockU32<Test>;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
    }

    impl crate::Config for Test {
        type OnProofVerified = Crash;
    }

    impl OnProofVerified for Crash {
        fn on_proof_verified(_pubs_hash: H256) {
            panic!("should never be called");
        }
    }

    // Build genesis storage according to the mock runtime.
    pub fn new_test_ext() -> sp_io::TestExternalities {
        frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .into()
    }

    #[test]
    #[should_panic(expected = "should never be called")]
    fn valid_proof_passes_verification_and_is_notified_another() {
        new_test_ext().execute_with(|| {
            // Dispatch a signed extrinsic.
            let _ =
                SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), VALID_PROOF.into());
        });
    }

    #[test]
    fn malformed_proof_fails_verification_and_is_not_notified_another() {
        new_test_ext().execute_with(|| {
            let mut malformed_proof: Proof = VALID_PROOF;
            // first byte changed from '0x17' to '0x07' (raw proof data)
            malformed_proof[0] = 0x07;

            // Dispatch a signed extrinsic.
            let _ = SettlementFFlonkPallet::submit_proof(
                RuntimeOrigin::signed(1),
                malformed_proof.into(),
            );

            // should not panic
        });
    }

    #[test]
    fn invalid_proof_fails_verification_and_is_not_notified_another() {
        new_test_ext().execute_with(|| {
            let mut invalid_proof: Proof = VALID_PROOF;
            // last byte changed from '0x06' to '0x00' (public inputs)
            invalid_proof[invalid_proof.len() - 1] = 0x00;

            // Dispatch a signed extrinsic.
            let _ = SettlementFFlonkPallet::submit_proof(
                RuntimeOrigin::signed(1),
                invalid_proof.into(),
            );

            // should not panic
        });
    }
}
