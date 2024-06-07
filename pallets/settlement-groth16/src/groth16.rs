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

use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use codec::{Decode, Encode, MaxEncodedLen};
use core::fmt::Debug;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub use crate::data_structures::{Proof, Scalar};
pub use crate::groth16_generic::Groth16Error;
use crate::{
    data_structures::{G1, G2},
    groth16_generic::{Groth16Generic, VerificationKey},
};

#[derive(Copy, Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum Curve {
    Bn254,
    Bls12_381,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct VerificationKeyWithCurve {
    pub curve: Curve,
    pub alpha_g1: G1,
    pub beta_g2: G2,
    pub gamma_g2: G2,
    pub delta_g2: G2,
    pub gamma_abc_g1: Vec<G1>,
}

impl VerificationKeyWithCurve {
    pub fn from_curve_and_vk(curve: Curve, vk: VerificationKey) -> Self {
        Self {
            curve,
            alpha_g1: vk.alpha_g1,
            beta_g2: vk.beta_g2,
            gamma_g2: vk.gamma_g2,
            delta_g2: vk.delta_g2,
            gamma_abc_g1: vk.gamma_abc_g1,
        }
    }

    pub fn vk(self) -> VerificationKey {
        VerificationKey {
            alpha_g1: self.alpha_g1,
            beta_g2: self.beta_g2,
            gamma_g2: self.gamma_g2,
            delta_g2: self.delta_g2,
            gamma_abc_g1: self.gamma_abc_g1,
        }
    }
}

pub struct Groth16;

impl Groth16 {
    pub fn verify_proof(
        proof: Proof,
        vk: VerificationKeyWithCurve,
        inputs: &[Scalar],
    ) -> Result<bool, Groth16Error> {
        let curve = vk.curve;
        let vk = vk.vk();
        match curve {
            Curve::Bn254 => Groth16Generic::<Bn254>::verify_proof(proof, vk, inputs),
            Curve::Bls12_381 => Groth16Generic::<Bls12_381>::verify_proof(proof, vk, inputs),
        }
    }

    #[cfg(any(test, feature = "runtime-benchmarks"))]
    pub fn get_instance(
        num_inputs: usize,
        rng_seed: Option<u64>,
        curve: Curve,
    ) -> (Proof, VerificationKeyWithCurve, Vec<Scalar>) {
        let (proof, vk, inputs) = match curve {
            Curve::Bn254 => Groth16Generic::<Bn254>::get_instance(num_inputs, rng_seed),
            Curve::Bls12_381 => Groth16Generic::<Bls12_381>::get_instance(num_inputs, rng_seed),
        };

        (
            proof,
            VerificationKeyWithCurve::from_curve_and_vk(curve, vk),
            inputs,
        )
    }
}
