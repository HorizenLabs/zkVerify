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

use super::*;
use crate::mock::*;
use frame_support::dispatch::{GetDispatchInfo, Pays};
use frame_support::{assert_err, assert_err_ignore_postinfo};
use frame_support::{assert_noop, assert_ok};
use hp_verifiers::{Verifier, WeightInfo};
use rstest::{fixture, rstest};
use sp_core::H256;
use sp_runtime::{BuildStorage, DispatchError};

type Vk = <FakeVerifier as Verifier>::Vk;
type RError = Error<Test, FakeVerifier>;
type VkOrHash = super::VkOrHash<Vk>;
type DisableStorage = Disabled<Test, FakeVerifier>;

pub const USER_1: AccountId = 42;
pub const USER_2: AccountId = 24;
pub static USERS: [(AccountId, Balance); 2] = [(USER_1, 42_000_000_000), (USER_2, 24_000_000_000)];

#[fixture]
pub fn test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: USERS.to_vec(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::from(t);

    ext.execute_with(|| {
        System::set_block_number(1);
    });
    ext
}

pub mod registered_vk {
    use hex_literal::hex;

    use super::*;

    pub const REGISTERED_VK: Vk = 4325;
    pub const REGISTERED_VK_HASH: H256 = H256(hex!(
        "7aeb79b96627dd87eac158bec5612ddb7f350513a179d9ab0daf4ab5788c3262"
    ));
    pub const VALID_HASH_REGISTERED_VK: H256 = H256(hex!(
        "a65dc57cd8f1e436aaa8a8a473005040a4594f5411e0d9c7c5d7f20630217b79"
    ));

    /// Provide an environment with a registered vk
    #[fixture]
    pub fn def_vk(mut test_ext: sp_io::TestExternalities) -> sp_io::TestExternalities {
        test_ext.execute_with(|| {
            FakeVerifierPallet::register_vk(RuntimeOrigin::signed(USER_1), Box::new(REGISTERED_VK))
                .unwrap();
            System::reset_events();
        });
        test_ext
    }
}

mod register_should {
    use hex_literal::hex;
    use registered_vk::*;

    use super::*;

    #[rstest]
    #[case(42, H256(hex!("ee55bf17be166383be3ca3ff9d91bc5f3400bb658843fe52e62f5ceb16b5f101")))]
    #[case(24, H256(hex!("0e570c1367b641384abf443b67b3de101c1f6ed3b7d41113772866dfc15f38f9")))]
    fn accept_valid_vk(
        mut test_ext: sp_io::TestExternalities,
        #[case] vk: Vk,
        #[case] expected_hash: H256,
    ) {
        test_ext.execute_with(|| {
            assert_ok!(FakeVerifierPallet::register_vk(
                RuntimeOrigin::signed(USER_1),
                Box::new(vk)
            ));

            System::assert_last_event(
                Event::VkRegistered {
                    hash: expected_hash,
                }
                .into(),
            );
        });
    }

    #[rstest]
    fn reject_invalid_vk(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            // Dispatch a signed extrinsic.
            assert_noop!(
                FakeVerifierPallet::register_vk(
                    RuntimeOrigin::signed(1),
                    FakeVerifier::malformed_vk()
                ),
                RError::InvalidVerificationKey
            );
        });
    }

    #[rstest]
    fn reject_valid_vk_if_disabled(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            DisableStorage::set(Some(true));
            assert!(
                FakeVerifierPallet::register_vk(RuntimeOrigin::signed(1), Box::new(42),).is_err(),
            );
        });
    }

    #[test]
    fn use_the_configured_weights() {
        let info = Call::<Test, FakeVerifier>::register_vk { vk: Box::new(42) }.get_dispatch_info();

        assert_eq!(info.pays_fee, Pays::Yes);
        assert_eq!(info.weight, MockWeightInfo::register_vk(&43));
    }

    #[rstest]
    fn hold_a_deposit(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            let initial_reserved_balance = Balances::reserved_balance(USER_1);
            assert_ok!(FakeVerifierPallet::register_vk(
                RuntimeOrigin::signed(USER_1),
                Box::new(42)
            ));
            assert!(Balances::reserved_balance(USER_1) > initial_reserved_balance);
        })
    }

    #[rstest]
    fn fail_if_insufficient_free_balance(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            assert_noop!(
                FakeVerifierPallet::register_vk(RuntimeOrigin::signed(1), Box::new(42)),
                DispatchError::Token(sp_runtime::TokenError::FundsUnavailable)
            );
        })
    }

    #[rstest]
    fn not_be_allowed_for_root(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            assert_noop!(
                FakeVerifierPallet::register_vk(RuntimeOrigin::root(), Box::new(42)),
                DispatchError::BadOrigin
            );
        })
    }

    #[rstest]
    fn fail_if_vk_is_registered_twice(mut def_vk: sp_io::TestExternalities) {
        def_vk.execute_with(|| {
            assert_noop!(
                FakeVerifierPallet::register_vk(
                    RuntimeOrigin::signed(USER_1),
                    Box::new(REGISTERED_VK)
                ),
                RError::VerificationKeyAlreadyRegistered
            );
        })
    }
}

mod unregister_should {
    use super::*;
    use registered_vk::*;

    #[rstest]
    fn unregister_a_previously_registered_vk(mut def_vk: sp_io::TestExternalities) {
        def_vk.execute_with(|| {
            assert_ok!(FakeVerifierPallet::unregister_vk(
                RuntimeOrigin::signed(USER_1),
                REGISTERED_VK_HASH
            ));
        })
    }

    #[rstest]
    fn release_deposit(mut def_vk: sp_io::TestExternalities) {
        def_vk.execute_with(|| {
            assert!(Balances::reserved_balance(USER_1) > 0);
            FakeVerifierPallet::unregister_vk(RuntimeOrigin::signed(USER_1), REGISTERED_VK_HASH)
                .unwrap();
            assert_eq!(Balances::reserved_balance(USER_1), 0)
        })
    }

    #[rstest]
    fn emit_event(mut def_vk: sp_io::TestExternalities) {
        def_vk.execute_with(|| {
            assert_ok!(FakeVerifierPallet::unregister_vk(
                RuntimeOrigin::signed(USER_1),
                REGISTERED_VK_HASH
            ));
            mock::System::assert_last_event(
                Event::VkUnregistered {
                    hash: REGISTERED_VK_HASH,
                }
                .into(),
            );
        })
    }

    mod fail {
        use super::*;
        use frame_support::assert_noop;

        #[rstest]
        fn on_root_origin(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                assert_noop!(
                    FakeVerifierPallet::unregister_vk(RuntimeOrigin::root(), REGISTERED_VK_HASH),
                    DispatchError::BadOrigin
                );
            })
        }

        #[rstest]
        fn on_wrong_origin(mut def_vk: sp_io::TestExternalities) {
            def_vk.execute_with(|| {
                assert_noop!(
                    FakeVerifierPallet::unregister_vk(
                        RuntimeOrigin::signed(USER_2),
                        REGISTERED_VK_HASH
                    ),
                    DispatchError::BadOrigin
                );
            })
        }

        #[rstest]
        fn on_nonexistent_vk(mut def_vk: sp_io::TestExternalities) {
            def_vk.execute_with(|| {
                assert_noop!(
                    FakeVerifierPallet::unregister_vk(
                        RuntimeOrigin::signed(USER_1),
                        H256::from_low_u64_be(42)
                    ),
                    RError::VerificationKeyNotFound
                );
            })
        }
    }
}

mod submit_proof_should {
    use super::*;
    use hex_literal::hex;
    use registered_vk::*;

    #[rstest]
    #[case::vk(VkOrHash::Vk(Box::new(REGISTERED_VK)), VALID_HASH_REGISTERED_VK)]
    #[case::use_registered_vk(VkOrHash::Hash(REGISTERED_VK_HASH), VALID_HASH_REGISTERED_VK)]
    fn validate_proof_and_notify_execution_when(
        mut def_vk: sp_io::TestExternalities,
        #[case] vk_or_hash: VkOrHash,
        #[case] expected_hash: H256,
    ) {
        use on_proof_verified::new_proof_event;

        def_vk.execute_with(|| {
            // Dispatch a signed extrinsic.
            assert_ok!(FakeVerifierPallet::submit_proof(
                RuntimeOrigin::signed(1),
                vk_or_hash,
                Box::new(42),
                Box::new(42),
            ));

            assert_eq!(System::events().len(), 1);

            System::assert_last_event(new_proof_event(expected_hash).into());
        });
    }

    #[rstest]
    #[case::submit_proof(VkOrHash::from_vk(24), MockWeightInfo::submit_proof(&5, &6))]
    #[case::submit_proof_with_vk_hash(
        VkOrHash::from_hash(REGISTERED_VK_HASH),
        MockWeightInfo::submit_proof_with_vk_hash(&12, &75)
    )]
    fn use_the_configured_weights(
        #[case] vk_or_hash: VkOrHash,
        #[case] weight: frame_support::weights::Weight,
    ) {
        let info = Call::<Test, FakeVerifier>::submit_proof {
            vk_or_hash,
            proof: Box::new(42),
            pubs: Box::new(24),
        }
        .get_dispatch_info();

        assert_eq!(info.pays_fee, Pays::Yes);
        assert_eq!(info.weight, weight);
    }

    mod reject {
        use super::*;

        #[rstest]
        fn valid_proof_if_disabled(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                DisableStorage::set(Some(true));
                // Dispatch a signed valid proof.
                assert!(FakeVerifierPallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    VkOrHash::from_vk(32),
                    Box::new(42),
                    Box::new(42),
                )
                .is_err());
            });
        }

        #[rstest]
        fn invalid_proof(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                // Dispatch a signed extrinsic.
                assert_noop!(
                    FakeVerifierPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        VkOrHash::from_vk(32),
                        Box::new(42),
                        Box::new(24),
                    ),
                    RError::VerifyError
                );
            });
        }

        #[rstest]
        fn proof_if_request_to_use_an_unregisterd_vk(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                assert_noop!(
                    FakeVerifierPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        VkOrHash::Hash(H256(hex!(
                            "ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000"
                        ))),
                        Box::new(42),
                        Box::new(42),
                    ),
                    RError::VerificationKeyNotFound
                );
            });
        }

        #[rstest]
        fn malformed_proof(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                assert_noop!(
                    FakeVerifierPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        VkOrHash::from_vk(32),
                        FakeVerifier::malformed_proof(),
                        Box::new(42),
                    ),
                    RError::InvalidProofData
                );
            });
        }

        #[rstest]
        fn malformed_vk(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                assert_noop!(
                    FakeVerifierPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        VkOrHash::from_vk(*FakeVerifier::malformed_vk()),
                        Box::new(42),
                        Box::new(42),
                    ),
                    RError::InvalidVerificationKey
                );
            });
        }

        #[rstest]
        fn malformed_pubs(mut test_ext: sp_io::TestExternalities) {
            test_ext.execute_with(|| {
                assert_noop!(
                    FakeVerifierPallet::submit_proof(
                        RuntimeOrigin::signed(1),
                        VkOrHash::from_vk(42),
                        Box::new(42),
                        FakeVerifier::malformed_pubs(),
                    ),
                    RError::InvalidInput
                );
            });
        }
    }
}

#[cfg(test)]
mod disable_should {
    use common::WeightInfo;

    use super::*;

    #[rstest]
    fn set_the_correct_state(
        mut test_ext: sp_io::TestExternalities,
        #[values(true, false)] value: bool,
    ) {
        test_ext.execute_with(|| {
            assert_eq!(FakeVerifierPallet::disabled(), None);

            FakeVerifierPallet::disable(RuntimeOrigin::root(), value).unwrap();
            assert_eq!(FakeVerifierPallet::disabled(), Some(value));
        });
    }

    #[rstest]
    fn disable_execution(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            assert_eq!(FakeVerifierPallet::disabled(), None);

            FakeVerifierPallet::disable(RuntimeOrigin::root(), true).unwrap();
            // Dispatch a signed valid proof.
            assert_err_ignore_postinfo!(
                FakeVerifierPallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    VkOrHash::from_vk(32),
                    Box::new(42),
                    Box::new(42),
                ),
                RError::DisabledVerifier
            );
        });
    }

    #[rstest]
    fn disable_register_vk(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            assert_eq!(FakeVerifierPallet::disabled(), None);

            FakeVerifierPallet::disable(RuntimeOrigin::root(), true).unwrap();

            assert_err_ignore_postinfo!(
                FakeVerifierPallet::register_vk(RuntimeOrigin::signed(1), 42.into(),),
                RError::DisabledVerifier
            );
        });
    }

    #[rstest]
    fn disable_execution_pay_the_correct_weight(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            assert_eq!(FakeVerifierPallet::disabled(), None);

            FakeVerifierPallet::disable(RuntimeOrigin::root(), true).unwrap();

            // I cannot use `assert_err_with_weight` here because it doesn't work with
            // try-runtime feature,
            assert_err!(
                FakeVerifierPallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    VkOrHash::from_vk(32),
                    Box::new(42),
                    Box::new(42),
                ),
                on_disable_error::<Test, FakeVerifier>(),
            );
            assert_err!(
                FakeVerifierPallet::register_vk(RuntimeOrigin::signed(1), 42.into(),),
                on_disable_error::<Test, FakeVerifier>(),
            );
            assert_eq!(
                on_disable_error::<Test, FakeVerifier>()
                    .post_info
                    .actual_weight,
                Some(MockCommonWeightInfo::on_verify_disabled_verifier())
            );
        });
    }

    #[rstest]
    fn enable_a_disabled_execution(mut test_ext: sp_io::TestExternalities) {
        test_ext.execute_with(|| {
            DisableStorage::set(Some(true));

            FakeVerifierPallet::disable(RuntimeOrigin::root(), false).unwrap();
            // Dispatch a signed valid proof.
            assert_ok!(FakeVerifierPallet::submit_proof(
                RuntimeOrigin::signed(1),
                VkOrHash::from_vk(32),
                Box::new(42),
                Box::new(42),
            ));
            assert_ok!(FakeVerifierPallet::register_vk(
                RuntimeOrigin::signed(USER_1),
                42.into(),
            ));
        });
    }

    #[rstest]
    fn be_rejected_if_no_root(
        mut test_ext: sp_io::TestExternalities,
        #[values(true, false)] value: bool,
    ) {
        test_ext.execute_with(|| {
            assert_noop!(
                FakeVerifierPallet::disable(RuntimeOrigin::signed(1), value),
                sp_runtime::traits::BadOrigin
            );
        });
    }
}
