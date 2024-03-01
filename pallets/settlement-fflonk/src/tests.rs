use sp_core::H256;

use crate::mock;
use crate::mock::*;
use crate::Proof;

pub static VALID_PROOF_HEX: &str = "17f87a599ca7d3a86ffd7de8cf12adcfd418136c14aec5ced91f4a49b2975c2c\
                                    1287c8ed2b2009c6fe9031e272439442d8ccda251de9c8737c2e5af3689a1767\
                                    1b74f0d660e9e88f0f8f87c6e32be65cb71204e4fd385c29fa93f3aa043c26ba\
                                    2581eb0d9e2b5942ec8ffc9d61650e05d049c8d35b986f1224b6876d12b6194b\
                                    1da25c0ab8021a9b52681e5510be5f2e38bc5daf6ade3d58a0d54711aa33c534\
                                    1fed05884b416a93d551d27a6fdf683972568ff0d2c9a26c8425d0604c3b77a9\
                                    2ca037535c6e9d94a8cf15511dd38a5a43377816242ce93846d8f882306f39a3\
                                    17daf2a44ced35aa8ac02921c5f8c0557f30290d5940f52e2d1fa4608ea5b1db\
                                    0954ff268194b6e09677a8e930a1cf8e38b5315807ed8b393954b626263896d9\
                                    184f910581d502641cd8cff4512b1d4e382932b55dc8d816484b9de0c9c43630\
                                    1a345a58b9a9f87ac671a3f7bb17032c41a75a537f9101a5aeb83009feeef401\
                                    0c74addd2dbe0ee47fcfc2b1cf5cec3c5e86692ef48f1c0235fad1d7a01c668e\
                                    03372f5c6df30567156e9a2788f8a404033b4cc12591084918018425b36c85e1\
                                    20120c2975a7dfb730fdae333a771049473e4c13eb3ccd85911d8a6e1a8ec19f\
                                    00e2b3945fa3224f8f395791ed78709d153044397bf0a48cc41a2007b5228086\
                                    012b96cd44c4f4ea2fdc8beb2414e0bb5b3c9de9df1a938044e522e1c6fff631\
                                    25008ebe0c16aac088bc38cbb5f487b5601673421aa31462869c8c992e4ca321\
                                    181f1c35924e14d4b3aa39a55331f016e7a1bda6b0562f227493c38f2bcd94aa\
                                    1ea83ce07e30d84945c0a665d1f9e0e93fd2db9f3a61fd9c05f33e753715dbec\
                                    1deed29feb3a59387ea9b087fc0c6b36b2a69124da7ced65b852d1535a385b64\
                                    1a950c68fe0cd92b6f4e83890b62a8e115f126ba0399084b6def365ed80fe360\
                                    27887a2f0b8a87c873b171d74db622cd77e67291bee1c59a9fa7f00ca0b87e95\
                                    09c6dfcc7db43ceee36998f660efa5e1c485a083a43c497b8e1061ab2b9bc0c2\
                                    1948698c7b7f3b4c2b6f8ca07f6ca519c27dc72e87e67bbe4675a92a92371897\
                                    2a2e8a3d4897c9ef79f20daf88ce801f240a3bfb97b4e8e6faf831fbd9f26706";
pub static VALID_HASH: &str = "ec9d6d72bf6acbcd85aeb4bfc65f4cbd83b15a9035e66c9b392b711ac098f000";

#[test]
fn valid_proof_passes_verification_and_is_notified() {
    new_test_ext().execute_with(|| {
        let valid_proof = hex::decode(VALID_PROOF_HEX)
            .expect("Decoding failed")
            .try_into()
            .expect("Wrong size");

        // Dispatch a signed extrinsic.
        assert!(
            SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), valid_proof).is_ok()
        );

        let expected_hash = hex::decode(VALID_HASH)
            .expect("Decoding failed")
            .try_into()
            .expect("Wrong size");

        let events = mock::System::events();
        assert_eq!(events.len(), 1);

        mock::System::assert_last_event(
            crate::mock::on_proof_verified::pallet::Event::NewProof {
                value: H256(expected_hash),
            }
            .into(),
        );
    });
}

#[test]
fn malformed_proof_fails_verification_and_is_not_notified() {
    new_test_ext().execute_with(|| {
        let mut malformed_proof: Proof = hex::decode(VALID_PROOF_HEX)
            .expect("Decoding failed")
            .try_into()
            .expect("Wrong size");
        // first byte changed from '0x17' to '0x07' (raw proof data)
        malformed_proof[0] = 0x07;

        // Dispatch a signed extrinsic.
        assert!(
            SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), malformed_proof)
                .is_err()
        );

        let events = mock::System::events();
        assert_eq!(events.len(), 0);
    });
}

#[test]
fn invalid_proof_fails_verification_and_is_not_notified() {
    new_test_ext().execute_with(|| {
        let mut invalid_proof: Proof = hex::decode(VALID_PROOF_HEX)
            .expect("Decoding failed")
            .try_into()
            .expect("Wrong size");
        // last byte changed from '0x06' to '0x00' (public inputs)
        invalid_proof[invalid_proof.len() - 1] = 0x00;

        // Dispatch a signed extrinsic.
        assert!(
            SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), invalid_proof).is_err()
        );

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
            let valid_proof = hex::decode(VALID_PROOF_HEX)
                .expect("Decoding failed")
                .try_into()
                .expect("Wrong size");

            // Dispatch a signed extrinsic.
            let _ = SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), valid_proof);
        });
    }

    #[test]
    fn malformed_proof_fails_verification_and_is_not_notified_another() {
        new_test_ext().execute_with(|| {
            let mut malformed_proof: Proof = hex::decode(VALID_PROOF_HEX)
                .expect("Decoding failed")
                .try_into()
                .expect("Wrong size");
            // first byte changed from '0x17' to '0x07' (raw proof data)
            malformed_proof[0] = 0x07;

            // Dispatch a signed extrinsic.
            let _ = SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), malformed_proof);

            // should not panic
        });
    }

    #[test]
    fn invalid_proof_fails_verification_and_is_not_notified_another() {
        new_test_ext().execute_with(|| {
            let mut invalid_proof: Proof = hex::decode(VALID_PROOF_HEX)
                .expect("Decoding failed")
                .try_into()
                .expect("Wrong size");
            // last byte changed from '0x06' to '0x00' (public inputs)
            invalid_proof[invalid_proof.len() - 1] = 0x00;

            // Dispatch a signed extrinsic.
            let _ = SettlementFFlonkPallet::submit_proof(RuntimeOrigin::signed(1), invalid_proof);

            // should not panic
        });
    }
}
