use crate::mock;
use crate::mock::RuntimeEvent as TestEvent;
use crate::mock::*;
use frame_support::inherent::ProvideInherent;
use frame_support::pallet_prelude::InherentData;
use frame_system::{EventRecord, Phase};
use hex_literal::hex;
use hp_poe::OnProofVerified;
use hp_poe::INHERENT_IDENTIFIER;
use sp_core::H256;

fn assert_attestation_evt(id: u64, value: H256) {
    assert!(mock::System::events().contains(&EventRecord {
        phase: Phase::Initialization,
        event: TestEvent::Poe(crate::Event::NewAttestation {
            id: id,
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
            value: value,
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
        assert!(Poe::publish_attestation(RuntimeOrigin::root()).is_ok());
        assert_attestation_evt(0, H256::default());
    })
}

#[test]
fn root_publish_two_attestations() {
    new_test_ext().execute_with(|| {
        assert!(Poe::publish_attestation(RuntimeOrigin::root()).is_ok());
        assert!(Poe::publish_attestation(RuntimeOrigin::root()).is_ok());
        assert_attestation_evt(0, H256::default());
        assert_attestation_evt(1, H256::default());
    })
}

#[test]
fn user_cannot_publish_attestation() {
    new_test_ext().execute_with(|| {
        assert!(Poe::publish_attestation(RuntimeOrigin::signed(1)).is_err());
    })
}

#[test]
fn one_tree_per_block() {
    // Test that even if we have more than MIN_PROOFS_FOR_ROOT_PUBLISHING proofs
    // they still end up in the same single merkle tree
    new_test_ext().execute_with(|| {
        for _ in 0..crate::mock::MIN_PROOFS_FOR_ROOT_PUBLISHING * 2 {
            let pid = H256::random();
            Poe::on_proof_verified(pid);
            assert_element_evt(0, pid);
        }
        assert!(Poe::publish_attestation(RuntimeOrigin::root()).is_ok());
    })
}

#[test]
fn proof_added() {
    new_test_ext().execute_with(|| {
        let pid = H256(HASHES[0].into());
        Poe::on_proof_verified(pid);
        assert_element_evt(0, pid);
    })
}

#[test]
fn correct_root() {
    new_test_ext().execute_with(|| {
        for h in HASHES {
            let pid = H256(h.into());
            Poe::on_proof_verified(pid);
            assert_element_evt(0, pid);
        }

        assert!(Poe::publish_attestation(RuntimeOrigin::root()).is_ok());
        let res =
            H256(hex!("138b734ecc0edcb6a36504258a5907f92734afb254b488156db374cee1d78f54").into());
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
                for p in 0..MIN_PROOFS_FOR_ROOT_PUBLISHING as usize {
                    Poe::on_proof_verified(H256(HASHES[p].into()));
                }
                assert_eq!(
                    Some(Call::publish_attestation {}),
                    Poe::create_inherent(&inherent_data())
                );
            })
        }

        #[test]
        fn if_timeout_expired() {
            new_test_ext().execute_with(|| {
                Poe::on_proof_verified(H256(HASHES[0].into()));
                // Move timestamp forward and check that root would be published
                Timestamp::set_timestamp(Timestamp::now() + MILLISECS_PER_PROOF_ROOT_PUBLISHING);
                assert_eq!(
                    Some(Call::publish_attestation {}),
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
                for p in 0..(MIN_PROOFS_FOR_ROOT_PUBLISHING - 1) as usize {
                    Poe::on_proof_verified(H256(HASHES[p].into()));
                }
                // Check that without enough elements nothing would be published
                assert_eq!(None, Poe::create_inherent(&inherent_data()));
            })
        }

        #[test]
        fn if_not_enough_unique_leaves() {
            new_test_ext().execute_with(|| {
                for _ in 0..(MIN_PROOFS_FOR_ROOT_PUBLISHING) as usize {
                    // Keep inserting the same element
                    Poe::on_proof_verified(H256(HASHES[0].into()));
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
