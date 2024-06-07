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
use crate::mock::*;
use crate::weight::WeightInfo;
use crate::{compute_groth16_hash, groth16::Curve, groth16::Groth16};
use frame_support::dispatch::{GetDispatchInfo, Pays};

#[test]
fn valid_proof_passes_verification_and_is_notified() {
    new_test_ext().execute_with(|| {
        let (proof, vk, inputs) = Groth16::get_instance(10, None, Curve::Bls12_381);
        let hash = compute_groth16_hash(&vk, &inputs);
        assert!(SettlementGroth16Pallet::submit_proof(
            RuntimeOrigin::signed(1),
            proof,
            vk.into(),
            inputs
        )
        .is_ok());

        let events = mock::System::events();
        assert_eq!(events.len(), 1);

        mock::System::assert_last_event(
            crate::mock::on_proof_verified::pallet::Event::NewProof { value: hash }.into(),
        );
    });
}

#[test]
fn invalid_proof_fails_verification_and_is_not_notified() {
    new_test_ext().execute_with(|| {
        let (proof, _, _) = Groth16::get_instance(10, Some(0), Curve::Bn254);
        let (_, vk, inputs) = Groth16::get_instance(10, Some(42), Curve::Bn254);

        assert!(SettlementGroth16Pallet::submit_proof(
            RuntimeOrigin::signed(1),
            proof,
            vk.into(),
            inputs,
        )
        .is_err());

        let events = mock::System::events();
        assert_eq!(events.len(), 0);
    });
}

#[test]
fn should_use_the_configured_weights() {
    let num_inputs = 10;
    let (proof, vk, inputs) = Groth16::get_instance(num_inputs, None, Curve::Bn254);

    let info = crate::pallet::Call::<Test>::submit_proof {
        proof,
        vk: vk.into(),
        input: inputs,
    }
    .get_dispatch_info();

    assert_eq!(info.pays_fee, Pays::Yes);
    assert_eq!(
        info.weight,
        MockWeightInfo::submit_proof_bn254(num_inputs as u32)
    );
}