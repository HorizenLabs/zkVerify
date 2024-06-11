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

use super::*;
use crate::mock::*;
use crate::weight::WeightInfo;
use crate::{compute_groth16_hash, groth16::Curve, groth16::Groth16};
use frame_support::dispatch::{GetDispatchInfo, Pays};
use frame_support::{assert_noop, assert_ok};
use rstest::rstest;
use rstest_reuse::{apply, template};

#[template]
#[rstest]
#[case::bn254(Curve::Bn254)]
#[case::bls12_381(Curve::Bls12_381)]
fn curves(#[case] curve: Curve) {}

mod submit_proof_should {
    use super::*;

    #[apply(curves)]
    fn validate_correct_proof_and_notify(#[case] curve: Curve) {
        new_test_ext().execute_with(|| {
            let (proof, vk, inputs) = Groth16::get_instance(10, None, curve);
            let hash = compute_groth16_hash(&vk, &inputs);
            assert_ok!(SettlementGroth16Pallet::submit_proof(
                RuntimeOrigin::signed(1),
                proof,
                vk.into(),
                inputs
            ));

            let events = System::events();
            assert_eq!(events.len(), 1);

            System::assert_last_event(
                on_proof_verified::pallet::Event::NewProof { value: hash }.into(),
            );
        });
    }

    #[rstest]
    #[case::no_inputs(0)]
    #[case::max_number_of_inputs(Test::MAX_NUM_INPUTS as usize)]
    fn validate_proof(#[case] n: usize) {
        new_test_ext().execute_with(|| {
            let (proof, vk, inputs) = Groth16::get_instance(n, None, Curve::Bn254);
            let hash = compute_groth16_hash(&vk, &inputs);
            assert_ok!(SettlementGroth16Pallet::submit_proof(
                RuntimeOrigin::signed(1),
                proof,
                vk.into(),
                inputs
            ));

            let events = System::events();
            assert_eq!(events.len(), 1);

            System::assert_last_event(
                on_proof_verified::pallet::Event::NewProof { value: hash }.into(),
            );
        });
    }

    #[apply(curves)]
    fn reject_incorrect_proof(#[case] curve: Curve) {
        new_test_ext().execute_with(|| {
            let (proof, _, _) = Groth16::get_instance(10, Some(0), curve);
            let (_, vk, inputs) = Groth16::get_instance(10, Some(42), curve);

            assert_noop!(
                SettlementGroth16Pallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    proof,
                    vk.into(),
                    inputs,
                ),
                Error::<Test>::VerifyError
            );
        });
    }

    #[apply(curves)]
    fn reject_malformed_proof(#[case] curve: Curve) {
        new_test_ext().execute_with(|| {
            let (mut proof, vk, inputs) = Groth16::get_instance(1, Some(0), curve);
            proof.a.0[0] += 1;

            assert_noop!(
                SettlementGroth16Pallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    proof,
                    vk.into(),
                    inputs,
                ),
                Error::<Test>::InvalidProof
            );
        });
    }

    #[apply(curves)]
    fn reject_malformed_vk(#[case] curve: Curve) {
        new_test_ext().execute_with(|| {
            let (proof, mut vk, inputs) = Groth16::get_instance(1, Some(0), curve);
            vk.alpha_g1.0[0] += 1;

            assert_noop!(
                SettlementGroth16Pallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    proof,
                    vk.into(),
                    inputs,
                ),
                Error::<Test>::InvalidVerificationKey
            );
        });
    }

    #[apply(curves)]
    fn reject_malformed_inputs(#[case] curve: Curve) {
        new_test_ext().execute_with(|| {
            let (proof, vk, mut inputs) = Groth16::get_instance(1, Some(0), curve);
            for byte in &mut inputs[0].0 {
                *byte = 0xff;
            }

            assert_noop!(
                SettlementGroth16Pallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    proof,
                    vk.into(),
                    inputs,
                ),
                Error::<Test>::InvalidInput
            );
        });
    }

    #[apply(curves)]
    fn reject_too_many_inputs(#[case] curve: Curve) {
        new_test_ext().execute_with(|| {
            let (proof, vk, inputs) =
                Groth16::get_instance(Test::MAX_NUM_INPUTS as usize + 1, Some(0), curve);

            assert_noop!(
                SettlementGroth16Pallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    proof,
                    vk.into(),
                    inputs,
                ),
                Error::<Test>::TooManyInputs
            );
        });
    }

    #[apply(curves)]
    fn reject_incoherent_vk_and_num_inputs(#[case] curve: Curve) {
        new_test_ext().execute_with(|| {
            let (proof, vk, _) = Groth16::get_instance(4, Some(0), curve);
            let (_, _, inputs) = Groth16::get_instance(5, Some(0), curve);

            assert_noop!(
                SettlementGroth16Pallet::submit_proof(
                    RuntimeOrigin::signed(1),
                    proof,
                    vk.into(),
                    inputs,
                ),
                Error::<Test>::VkAndInputsMismatch
            );
        });
    }

    #[apply(curves)]
    fn use_the_configured_weights(#[case] curve: Curve) {
        let num_inputs = 10;
        let (proof, vk, inputs) = Groth16::get_instance(num_inputs, None, curve);

        let info = crate::pallet::Call::<Test>::submit_proof {
            proof,
            vk: vk.into(),
            input: inputs,
        }
        .get_dispatch_info();

        assert_eq!(info.pays_fee, Pays::Yes);
        assert_eq!(
            info.weight,
            match curve {
                Curve::Bn254 => MockWeightInfo::submit_proof_bn254(num_inputs as u32),
                Curve::Bls12_381 => MockWeightInfo::submit_proof_bls12_381(num_inputs as u32),
            }
        );
    }
}
