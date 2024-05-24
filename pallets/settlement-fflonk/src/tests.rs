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

use frame_support::dispatch::{GetDispatchInfo, Pays};
use frame_support::{assert_noop, assert_ok};

use mock::on_proof_verified::new_proof_event;
use sp_core::H256;

use super::*;
use crate::mock::*;
use crate::vk::Vk;
use crate::weight::WeightInfo;
use rstest::{fixture, rstest};

include!("resources.rs");

mod register_should {
    use super::*;

    #[rstest]
    #[case::default_vk(fflonk_verifier::VerificationKey::default(), DEFAULT_VK_HASH)]
    #[case::other_vk(other_vk().0, other_vk().1)]
    fn accept_valid_vk(
        mut test_ext: sp_io::TestExternalities,
        #[case] vk: fflonk_verifier::VerificationKey,
        #[case] expected_hash: H256,
    ) {
        test_ext.execute_with(|| {
            let vk: Vk = vk.into();
            assert_ok!(SettlementFFlonkPallet::register_vk(
                RuntimeOrigin::signed(1),
                vk
            ));

            mock::System::assert_last_event(
                Event::VkRegistered {
                    hash: expected_hash,
                }
                .into(),
            );
        });
    }

    #[rstest]
    fn reject_invalid_vk(mut test_ext: sp_io::TestExternalities) {
        let mut vk: Vk = fflonk_verifier::VerificationKey::default().into();
        *vk.mut_c0_x() = sp_core::U256::zero();

        test_ext.execute_with(|| {
            // Dispatch a signed extrinsic.
            assert_noop!(
                SettlementFFlonkPallet::register_vk(RuntimeOrigin::signed(1), vk),
                Error::<Test>::InvalidVerificationKey
            );
        });
    }

    #[test]
    fn use_the_configured_weights() {
        let info = Call::<Test>::register_vk {
            vk: fflonk_verifier::VerificationKey::default().into(),
        }
        .get_dispatch_info();

        assert_eq!(info.pays_fee, Pays::Yes);
        assert_eq!(info.weight, MockWeightInfo::register_vk());
    }
}

mod submit_proof_should {
    use super::*;

    /// Provide an environment with a registered vk (use used the default vk)
    #[fixture]
    fn def_vk(mut test_ext: sp_io::TestExternalities) -> sp_io::TestExternalities {
        test_ext.execute_with(|| {
            SettlementFFlonkPallet::register_vk(
                RuntimeOrigin::signed(1),
                fflonk_verifier::VerificationKey::default().into(),
            )
            .unwrap();
            System::reset_events();
        });
        test_ext
    }

    #[rstest]
    #[case::no_given_vk(None, VALID_HASH)]
    #[case::provide_vk(Some(VkOrHash::Vk(fflonk_verifier::VerificationKey::default().into())), VALID_HASH_WITH_VK)]
    #[case::use_registered_vk(Some(VkOrHash::Hash(DEFAULT_VK_HASH)), VALID_HASH_WITH_VK)]
    fn validate_proof_and_notify_execution_when(
        mut def_vk: sp_io::TestExternalities,
        #[case] vk_or_hash: Option<VkOrHash>,
        #[case] expected_hash: H256,
    ) {
        def_vk.execute_with(|| {
            // Dispatch a signed extrinsic.
            assert_ok!(SettlementFFlonkPallet::submit_proof(
                RuntimeOrigin::signed(1),
                VALID_PROOF.into(),
                vk_or_hash
            ));

            assert_eq!(System::events().len(), 1);

            System::assert_last_event(new_proof_event(expected_hash).into());
        });
    }

    #[rstest]
    #[case::submit_proof_default_vk(None, MockWeightInfo::submit_proof_default())]
    #[case::submit_proof_with_vk_hash(
        Some(VkOrHash::Hash(DEFAULT_VK_HASH)),
        MockWeightInfo::submit_proof_with_vk_hash()
    )]
    #[case::submit_proof_with_vk(
        Some(VkOrHash::Vk(fflonk_verifier::VerificationKey::default().into())),
        MockWeightInfo::submit_proof_with_vk()
    )]
    fn use_the_configured_weights(
        #[case] vk_or_hash: Option<VkOrHash>,
        #[case] weight: frame_support::weights::Weight,
    ) {
        let info = Call::<Test>::submit_proof {
            raw_proof: Box::new(VALID_PROOF),
            vk_or_hash,
        }
        .get_dispatch_info();

        assert_eq!(info.pays_fee, Pays::Yes);
        assert_eq!(info.weight, weight);
    }

    mod reject {
        use super::*;

        #[rstest]
        fn proof_if_request_to_use_an_unregisterd_vk(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                assert_noop!(
                    SettlementFFlonkPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        VALID_PROOF.into(),
                        Some(VkOrHash::Hash(DEFAULT_VK_HASH))
                    ),
                    Error::<Test>::VerificationKeyNotFound
                );
            });
        }

        #[rstest]
        fn malformed_proof(mut test_ext: sp_io::TestExternalities) {
            let mut malformed_proof: Proof = VALID_PROOF;
            // first byte changed from '0x17' to '0x07' (raw proof data)
            malformed_proof[0] = 0x07;

            test_ext.execute_with(|| {
                assert_noop!(
                    SettlementFFlonkPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        malformed_proof.into(),
                        None
                    ),
                    Error::<Test>::InvalidProofData
                );
            });
        }

        #[rstest]
        fn invalid_proof(mut test_ext: sp_io::TestExternalities) {
            let mut invalid_proof: Proof = VALID_PROOF;
            // last byte changed from '0x06' to '0x00' (public inputs)

            invalid_proof[invalid_proof.len() - 1] = 0x00;
            test_ext.execute_with(|| {
                // Dispatch a signed extrinsic.
                assert_noop!(
                    SettlementFFlonkPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        invalid_proof.into(),
                        None
                    ),
                    Error::<Test>::VerifyError
                );
            });
        }

        #[rstest]
        #[case::other_hash(VkOrHash::Hash(other_vk().1))]
        #[case::other_vk(VkOrHash::Vk(other_vk().0.into()))]
        fn proof_provided_with_not_related_vk(
            mut test_ext: sp_io::TestExternalities,
            #[case] vk_or_hash: VkOrHash,
        ) {
            let (vk, _h) = other_vk();
            test_ext.execute_with(|| {
                SettlementFFlonkPallet::register_vk(RuntimeOrigin::signed(1), vk.into()).unwrap();
            });

            test_ext.execute_with(|| {
                assert_noop!(
                    SettlementFFlonkPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        VALID_PROOF.into(),
                        Some(vk_or_hash)
                    ),
                    Error::<Test>::VerifyError
                );
            });
        }
    }
}
