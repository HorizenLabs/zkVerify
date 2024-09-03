// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::mock;
use crate::mock::RuntimeEvent as TestEvent;
use crate::mock::*;
use crate::AttestationPathRequestError;
use frame_support::dispatch::GetDispatchInfo;
use frame_support::dispatch::Pays;
use frame_support::inherent::ProvideInherent;
use frame_support::pallet_prelude::InherentData;
use frame_system::{EventRecord, Phase};
use hex_literal::hex;
use hp_attestation::OnProofVerified;
use hp_attestation::INHERENT_IDENTIFIER;
use sp_core::H256;
use sp_runtime::traits::Keccak256;

fn assert_attestation_evt(id: u64, value: H256) {
    assert!(mock::System::events().contains(&EventRecord {
        phase: Phase::Initialization,
        event: TestEvent::Attestation(crate::Event::NewAttestation {
            id,
            attestation: value,
        }),
        topics: vec![],
    }))
}

fn assert_element_evt(id: u64, value: H256) {
    assert!(mock::System::events().contains(&EventRecord {
        phase: Phase::Initialization,
        event: TestEvent::Attestation(crate::Event::NewProof {
            attestation_id: id,
            value,
        }),
        topics: vec![],
    }))
}

pub static HASHES: [[u8; 32]; 5] = [
    hex!("bbfe8badbe1f542b860d9d7858edae06df34464cb086a47d4b62ba66e0608f72"),
    hex!("e88c51ed3460ac2b7ae83b7a6f79039983836e80fa2846bef39a0231244122ca"),
    hex!("cad1697e7a6a93e6533712bca47d77d77a5b4486d0db136d37a2c150211af6a7"),
    hex!("f9df33d6e172fd137eb9820db11a3e241f152c8cf8cd7ce118abfac2f534a750"),
    hex!("86de95769384558243ad17a2da0f305d2ea9888dcb7b6f933b6492f8dea56d7f"),
];

#[test]
fn root_publish_attestation() {
    new_test_ext().execute_with(|| {
        assert!(Attestation::publish_attestation(RuntimeOrigin::root()).is_ok());
        assert_attestation_evt(0, H256::default());
    })
}

#[test]
fn root_publish_two_attestations() {
    new_test_ext().execute_with(|| {
        assert!(Attestation::publish_attestation(RuntimeOrigin::root()).is_ok());
        assert!(Attestation::publish_attestation(RuntimeOrigin::root()).is_ok());
        assert_attestation_evt(0, H256::default());
        assert_attestation_evt(1, H256::default());
    })
}

#[test]
fn user_cannot_publish_attestation() {
    new_test_ext().execute_with(|| {
        assert!(Attestation::publish_attestation(RuntimeOrigin::signed(1)).is_err());
    })
}

#[test]
fn one_tree_per_block() {
    // Test that even if we have more than MIN_PROOFS_FOR_ROOT_PUBLISHING proofs
    // they still end up in the same single merkle tree
    new_test_ext().execute_with(|| {
        for _ in 0..crate::mock::MIN_PROOFS_FOR_ROOT_PUBLISHING * 2 {
            let pid = H256::random();
            Attestation::on_proof_verified(pid);
            assert_element_evt(0, pid);
        }
        assert!(Attestation::publish_attestation(RuntimeOrigin::root()).is_ok());
    })
}

#[test]
fn proof_added() {
    new_test_ext().execute_with(|| {
        let pid = H256(HASHES[0]);
        Attestation::on_proof_verified(pid);
        assert_element_evt(0, pid);
    })
}

#[test]
fn correct_root() {
    new_test_ext().execute_with(|| {
        for h in HASHES {
            let pid = H256(h);
            Attestation::on_proof_verified(pid);
            assert_element_evt(0, pid);
        }

        assert!(Attestation::publish_attestation(RuntimeOrigin::root()).is_ok());
        let res = H256(hex!(
            "138b734ecc0edcb6a36504258a5907f92734afb254b488156db374cee1d78f54"
        ));
        assert_attestation_evt(0, res);
    })
}

mod should_inherent_call {
    use super::*;
    use crate::Call;

    fn inherent_data() -> InherentData {
        let mut data = InherentData::default();
        data.put_data(INHERENT_IDENTIFIER, &()).unwrap();
        data
    }

    mod publish_attestation {
        use super::*;

        #[test]
        fn if_enough_leaves() {
            new_test_ext().execute_with(|| {
                // Add enough elements and check that root would be published
                for h in HASHES
                    .into_iter()
                    .take(MIN_PROOFS_FOR_ROOT_PUBLISHING as usize)
                {
                    Attestation::on_proof_verified(H256(h));
                }
                assert_eq!(
                    Some(Call::publish_attestation {}),
                    Attestation::create_inherent(&inherent_data())
                );
            })
        }

        #[test]
        fn if_timeout_expired() {
            new_test_ext().execute_with(|| {
                Attestation::on_proof_verified(H256(HASHES[0]));
                // Move timestamp forward and check that root would be published
                Timestamp::set_timestamp(Timestamp::now() + MILLISECS_PER_PROOF_ROOT_PUBLISHING);
                assert_eq!(
                    Some(Call::publish_attestation {}),
                    Attestation::create_inherent(&inherent_data())
                );
            })
        }
    }

    mod not_publish_attestation {
        use super::*;

        #[test]
        fn if_not_enough_leaves() {
            new_test_ext().execute_with(|| {
                for h in HASHES
                    .into_iter()
                    .take((MIN_PROOFS_FOR_ROOT_PUBLISHING - 1) as usize)
                {
                    Attestation::on_proof_verified(H256(h));
                }
                // Check that without enough elements nothing would be published
                assert_eq!(None, Attestation::create_inherent(&inherent_data()));
            })
        }

        #[test]
        fn if_not_enough_unique_leaves() {
            new_test_ext().execute_with(|| {
                for _ in 0..(MIN_PROOFS_FOR_ROOT_PUBLISHING) as usize {
                    // Keep inserting the same element
                    Attestation::on_proof_verified(H256(HASHES[0]));
                }
                // Check that without enough unique elements nothing would be published
                assert_eq!(None, Attestation::create_inherent(&inherent_data()));
            })
        }

        #[test]
        fn if_timeout_expired_but_no_leaves() {
            new_test_ext().execute_with(|| {
                // Move timestamp forward so that timeout expires
                Timestamp::set_timestamp(Timestamp::now() + MILLISECS_PER_PROOF_ROOT_PUBLISHING);
                // Check that nothing would be published
                assert_eq!(None, Attestation::create_inherent(&inherent_data()));
            })
        }
    }
}

/// Tests for the `get_proof_path_from_pallet` function
#[test]
fn get_proof_from_pallet_proof_not_found() {
    new_test_ext().execute_with(|| {
        for h in HASHES
            .into_iter()
            .take(MIN_PROOFS_FOR_ROOT_PUBLISHING as usize)
        {
            Attestation::on_proof_verified(H256(h));
        }
        Attestation::publish_attestation(RuntimeOrigin::root()).unwrap();
        let attestation_id = 0;
        let proof_hash = H256(hex!(
            "0badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbad"
        ));

        // Query for a proof that does not exist
        assert_eq!(
            Attestation::get_proof_path_from_pallet(attestation_id, proof_hash),
            Err(AttestationPathRequestError::ProofNotFound(
                attestation_id,
                proof_hash,
            ))
        );
    })
}

#[test]
fn get_proof_from_pallet_invalid_att_id() {
    new_test_ext().execute_with(|| {
        for h in HASHES
            .into_iter()
            .take(MIN_PROOFS_FOR_ROOT_PUBLISHING as usize)
        {
            Attestation::on_proof_verified(H256(h));
        }
        Attestation::publish_attestation(RuntimeOrigin::root()).unwrap();
        let attestation_id = 10;
        let proof_hash = H256(HASHES[0]);

        // Query for an existing proof with an invalid attestation id
        assert!(Attestation::get_proof_path_from_pallet(attestation_id, proof_hash).is_err());
    })
}

#[test]
fn get_proof_from_pallet_valid_att_id_and_valid_proof() {
    new_test_ext().execute_with(|| {
        for h in HASHES
            .into_iter()
            .take(MIN_PROOFS_FOR_ROOT_PUBLISHING as usize)
        {
            Attestation::on_proof_verified(H256(h));
        }
        Attestation::publish_attestation(RuntimeOrigin::root()).unwrap();
        let attestation_id = 0;
        let proof_hash = H256(HASHES[0]);

        let proof = Attestation::get_proof_path_from_pallet(attestation_id, proof_hash).unwrap();

        assert!(binary_merkle_tree::verify_proof::<Keccak256, _, _>(
            &proof.root,
            proof.proof,
            proof.number_of_leaves,
            proof.leaf_index,
            &proof_hash
        ));
    })
}

#[test]
fn should_use_the_configured_weights() {
    use crate::weight::WeightInfo;
    let info = crate::pallet::Call::<Test>::publish_attestation {}.get_dispatch_info();

    assert_eq!(info.pays_fee, Pays::Yes);
    assert_eq!(info.weight, MockWeightInfo::publish_attestation());
}
