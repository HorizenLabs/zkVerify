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
use frame_support::assert_ok;
use frame_support::dispatch::GetDispatchInfo;
use frame_support::dispatch::Pays;
use frame_support::inherent::ProvideInherent;
use frame_support::pallet_prelude::InherentData;
use frame_support::traits::{Hooks, Len};
use frame_system::{EventRecord, Phase};
use hex_literal::hex;
use hp_poe::OnProofVerified;
use hp_poe::INHERENT_IDENTIFIER;
use sp_core::H256;
use sp_runtime::traits::Keccak256;

fn count_new_attestation_events() -> u32 {
    mock::System::events()
        .iter()
        .filter(|event_record| {
            matches!(
                event_record.event,
                TestEvent::Poe(crate::Event::NewAttestation { .. })
            )
        })
        .count() as u32
}

fn assert_attestation_evt(id: u64, value: H256) {
    if let Some(first_event) = System::events().last() {
        println!("First event phase: {:?}", first_event.phase);
        println!("First event: {:?}", first_event.event);
        println!("First event topics: {:?}", first_event.topics);
    }

    assert!(mock::System::events().contains(&EventRecord {
        phase: Phase::Initialization,
        event: TestEvent::Poe(crate::Event::NewAttestation {
            id,
            attestation: value,
        }),
        topics: vec![],
    }))
}

fn assert_element_evt(id: u64, value: H256) {
    assert!(mock::System::events().contains(&EventRecord {
        phase: Phase::Initialization,
        event: TestEvent::Poe(crate::Event::NewElement {
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

pub static DEFAULT_EMPTY_ATT: [u8; 32] =
    hex!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563");

pub static FIRST_TREE_HASH: [u8; 32] =
    hex!("0dbeb4955732181494864950fb47e5ec70089d1d03065c0eb5101797734b1ace");

pub static SECOND_TREE_HASH: [u8; 32] =
    hex!("5e9d0e6da80e5a0ea225b926fec6266a8d27320effb8c3b5aa51d900559de1d6");

#[test]
fn root_publish_attestation() {
    new_test_ext().execute_with(|| {
        assert_ok!(Poe::publish_attestations(RuntimeOrigin::root()));
        assert_attestation_evt(0, H256(DEFAULT_EMPTY_ATT));
    })
}

#[test]
fn root_publish_two_attestations() {
    new_test_ext().execute_with(|| {
        assert_ok!(Poe::publish_attestations(RuntimeOrigin::root()));
        assert_ok!(Poe::publish_attestations(RuntimeOrigin::root()));
        assert_attestation_evt(0, H256(DEFAULT_EMPTY_ATT));
        assert_attestation_evt(1, H256(DEFAULT_EMPTY_ATT));
    })
}

#[test]
fn user_cannot_publish_attestation() {
    new_test_ext().execute_with(|| {
        assert!(Poe::publish_attestations(RuntimeOrigin::signed(1)).is_err());
    })
}

#[test]
fn proof_added() {
    new_test_ext().execute_with(|| {
        let pid = H256(HASHES[0]);
        Poe::on_proof_verified(pid);
        assert_element_evt(0, pid);
    })
}

#[test]
fn two_attestations_no_remaining() {
    new_test_ext().execute_with(|| {
        for i in 0..4 {
            let hash = HASHES[i];
            Poe::on_proof_verified(H256(hash));
            if i <= 1 {
                assert_element_evt(0, H256(hash));
            } else {
                assert_element_evt(1, H256(hash));
            }
        }

        assert_eq!(Poe::attestations_to_be_cleared().len(), 0);
        assert_ok!(Poe::publish_attestations(RuntimeOrigin::root()));
        assert_attestation_evt(0, H256(FIRST_TREE_HASH));
        assert_attestation_evt(1, H256(SECOND_TREE_HASH));

        assert_eq!(Poe::attestations_to_be_cleared().len(), 2);
        assert!(Poe::attestations_to_be_cleared().contains(&1));
        assert!(Poe::attestations_to_be_cleared().contains(&0));

        assert_eq!(Poe::attestations_with_proofs_to_be_published(1).len(), 2);
        assert!(Poe::attestations_with_proofs_to_be_published(0).contains(&H256(HASHES[0])));
        assert!(Poe::attestations_with_proofs_to_be_published(0).contains(&H256(HASHES[1])));
        assert!(Poe::attestations_with_proofs_to_be_published(1).contains(&H256(HASHES[2])));
        assert!(Poe::attestations_with_proofs_to_be_published(1).contains(&H256(HASHES[3])));

        // Move to the next block
        System::set_block_number(2);
        Poe::on_initialize(2);

        assert_eq!(Poe::attestations_to_be_cleared().len(), 0);
        assert_eq!(Poe::attestations_with_proofs_to_be_published(0).len(), 0);
        assert_eq!(Poe::attestations_with_proofs_to_be_published(1).len(), 0);
    })
}

#[test]
fn two_attestations_one_proof_remaining() {
    new_test_ext().execute_with(|| {
        for i in 0..5 {
            let hash = HASHES[i];
            Poe::on_proof_verified(H256(hash));
            if i <= 1 {
                assert_element_evt(0, H256(hash));
            } else if i > 1 && i <= 3 {
                assert_element_evt(1, H256(hash));
            } else {
                assert_element_evt(2, H256(hash));
            }
        }

        assert_eq!(Poe::attestations_to_be_cleared().len(), 0);
        assert_ok!(Poe::publish_attestations(RuntimeOrigin::root()));
        assert_eq!(count_new_attestation_events(), 2);

        assert_eq!(Poe::attestations_to_be_cleared().len(), 2);
        assert!(Poe::attestations_to_be_cleared().contains(&1));
        assert!(Poe::attestations_to_be_cleared().contains(&0));

        assert_eq!(Poe::attestations_with_proofs_to_be_published(1).len(), 2);
        assert!(Poe::attestations_with_proofs_to_be_published(0).contains(&H256(HASHES[0])));
        assert!(Poe::attestations_with_proofs_to_be_published(0).contains(&H256(HASHES[1])));
        assert!(Poe::attestations_with_proofs_to_be_published(1).contains(&H256(HASHES[2])));
        assert!(Poe::attestations_with_proofs_to_be_published(1).contains(&H256(HASHES[3])));
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
                for h in HASHES.into_iter().take(PROOFS_PER_ATTESTATION as usize) {
                    Poe::on_proof_verified(H256(h));
                }
                assert_eq!(
                    Some(Call::publish_attestations {}),
                    Poe::create_inherent(&inherent_data())
                );
            })
        }

        #[test]
        fn if_timeout_expired() {
            new_test_ext().execute_with(|| {
                Poe::on_proof_verified(H256(HASHES[0]));
                // Move timestamp forward and check that root would be published
                Timestamp::set_timestamp(Timestamp::now() + MILLISECS_PER_PROOF_ROOT_PUBLISHING);
                assert_eq!(
                    Some(Call::publish_attestations {}),
                    Poe::create_inherent(&inherent_data())
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
                    .take((PROOFS_PER_ATTESTATION - 1) as usize)
                {
                    Poe::on_proof_verified(H256(h));
                }
                // Check that without enough elements nothing would be published
                assert_eq!(None, Poe::create_inherent(&inherent_data()));
            })
        }

        #[test]
        fn if_not_enough_unique_leaves() {
            new_test_ext().execute_with(|| {
                for _ in 0..(PROOFS_PER_ATTESTATION) as usize {
                    // Keep inserting the same element
                    Poe::on_proof_verified(H256(HASHES[0]));
                }
                // Check that without enough unique elements nothing would be published
                assert_eq!(None, Poe::create_inherent(&inherent_data()));
            })
        }

        #[test]
        fn if_timeout_expired_but_no_leaves() {
            new_test_ext().execute_with(|| {
                // Move timestamp forward so that timeout expires
                Timestamp::set_timestamp(Timestamp::now() + MILLISECS_PER_PROOF_ROOT_PUBLISHING);
                // Check that nothing would be published
                assert_eq!(None, Poe::create_inherent(&inherent_data()));
            })
        }
    }
}

/// Tests for the `get_proof_path_from_pallet` function
#[test]
fn get_proof_from_pallet_proof_not_found() {
    new_test_ext().execute_with(|| {
        for h in HASHES.into_iter().take(PROOFS_PER_ATTESTATION as usize) {
            Poe::on_proof_verified(H256(h));
        }
        Poe::publish_attestations(RuntimeOrigin::root()).unwrap();
        let attestation_id = 0;
        let proof_hash = H256(hex!(
            "0badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbad"
        ));

        // Query for a proof that does not exist
        assert_eq!(
            Poe::get_proof_path_from_pallet(attestation_id, proof_hash),
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
        for h in HASHES.into_iter().take(PROOFS_PER_ATTESTATION as usize) {
            Poe::on_proof_verified(H256(h));
        }
        Poe::publish_attestations(RuntimeOrigin::root()).unwrap();
        let attestation_id = 10;
        let proof_hash = H256(HASHES[0]);

        // Query for an existing proof with an invalid attestation id
        assert!(Poe::get_proof_path_from_pallet(attestation_id, proof_hash).is_err());
    })
}

#[test]
fn get_proof_from_pallet_valid_att_id_and_valid_proof() {
    new_test_ext().execute_with(|| {
        for h in HASHES.into_iter().take(PROOFS_PER_ATTESTATION as usize) {
            Poe::on_proof_verified(H256(h));
        }
        Poe::publish_attestations(RuntimeOrigin::root()).unwrap();
        let attestation_id = 0;
        let proof_hash = H256(HASHES[0]);

        let proof = Poe::get_proof_path_from_pallet(attestation_id, proof_hash).unwrap();

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
    let info = crate::pallet::Call::<Test>::publish_attestations {}.get_dispatch_info();

    assert_eq!(info.pays_fee, Pays::Yes);
    assert_eq!(info.weight, MockWeightInfo::publish_attestations());
}
