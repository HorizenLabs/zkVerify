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

#[fixture]
fn def_vk(mut test_ext: sp_io::TestExternalities) -> sp_io::TestExternalities {
    test_ext.execute_with(|| {
        SettlementFFlonkPallet::register_vk(
            RuntimeOrigin::signed(1),
            fflonk_verifier::VerificationKey::default().into(),
        )
        .unwrap();
    });
    test_ext
}

#[rstest]
fn valid_proof_passes_verification_and_is_notified(mut test_ext: sp_io::TestExternalities) {
    test_ext.execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(1),
            VALID_PROOF.into(),
            None
        ));

        assert_eq!(System::events().len(), 1);

        System::assert_last_event(new_proof_event(VALID_HASH).into());
    });
}

#[rstest]
#[case::default_vk(fflonk_verifier::VerificationKey::default(), DEFAULT_VK_HASH)]
#[case::other_vk(other_vk().0, other_vk().1)]
fn should_register_valid_vk(
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
fn should_reject_invalid_vk(mut test_ext: sp_io::TestExternalities) {
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

#[rstest]
fn should_use_the_registered_vk(mut def_vk: sp_io::TestExternalities) {
    def_vk.execute_with(|| {
        assert_ok!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(1),
            VALID_PROOF.into(),
            Some(DEFAULT_VK_HASH)
        ));

        System::assert_last_event(new_proof_event(VALID_HASH_WITH_VK).into());
    });
}

#[rstest]
fn should_return_error_if_request_an_unregisterd_vk(mut test_ext: sp_io::TestExternalities) {
    test_ext.execute_with(|| {
        assert_noop!(
            SettlementFFlonkPallet::submit_proof(
                RuntimeOrigin::signed(1),
                VALID_PROOF.into(),
                Some(DEFAULT_VK_HASH)
            ),
            Error::<Test>::VerificationKeyNotFound
        );
    });
}

#[rstest]
fn malformed_proof_fails_verification_and_is_not_notified(mut test_ext: sp_io::TestExternalities) {
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
fn invalid_proof_fails_verification_and_is_not_notified(mut test_ext: sp_io::TestExternalities) {
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
fn should_fail_if_try_to_verify_a_proof_providing_a_not_related_vk(
    mut test_ext: sp_io::TestExternalities,
) {
    let (vk, h) = other_vk();
    test_ext.execute_with(|| {
        SettlementFFlonkPallet::register_vk(RuntimeOrigin::signed(1), vk.into()).unwrap();
    });

    test_ext.execute_with(|| {
        assert_noop!(
            SettlementFFlonkPallet::submit_proof(
                RuntimeOrigin::signed(1),
                VALID_PROOF.into(),
                Some(h)
            ),
            Error::<Test>::VerifyError
        );
    });
}

#[rstest]
#[case::submit_proof_default_vk(Call::<Test>::submit_proof {
        raw_proof: Box::new(VALID_PROOF),
        vk_hash: None,
    }, MockWeightInfo::submit_proof_default(), Pays::Yes)]
#[case::submit_proof_with_vk(Call::<Test>::submit_proof {
        raw_proof: Box::new(VALID_PROOF),
        vk_hash: Some(DEFAULT_VK_HASH),
    }, MockWeightInfo::submit_proof_with_vk(), Pays::Yes)]
#[case::register_vk(Call::<Test>::register_vk {
        vk: fflonk_verifier::VerificationKey::default().into(),
    }, MockWeightInfo::register_vk(), Pays::Yes)]
fn should_use_the_configured_weights(
    #[case] call: Call<Test>,
    #[case] weight: frame_support::weights::Weight,
    #[case] pay: Pays,
) {
    let info = call.get_dispatch_info();

    assert_eq!(info.pays_fee, pay);
    assert_eq!(info.weight, weight);
}
