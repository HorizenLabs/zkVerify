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

use ark_ec::pairing::Pairing;
use ark_groth16::prepare_verifying_key;
use core::marker::PhantomData;
use sp_std::vec::Vec;

pub use crate::data_structures::{Proof, Scalar, VerificationKey};

#[derive(Debug, PartialEq)]
pub enum Groth16Error {
    InvalidProof,
    InvalidVerificationKey,
    InvalidInput,
    VerifyError,
}

pub struct Groth16Generic<E: Pairing> {
    _p: PhantomData<E>,
}

impl<E: Pairing> Groth16Generic<E> {
    pub fn verify_proof(
        proof: Proof,
        vk: VerificationKey,
        inputs: &[Scalar],
    ) -> Result<bool, Groth16Error> {
        let proof: ark_groth16::Proof<E> =
            proof.try_into().map_err(|_| Groth16Error::InvalidProof)?;
        let vk: ark_groth16::VerifyingKey<E> = vk
            .try_into()
            .map_err(|_| Groth16Error::InvalidVerificationKey)?;
        let pvk = prepare_verifying_key::<E>(&vk);
        let inputs = inputs
            .iter()
            .map(|v| v.clone().try_into_scalar::<E::ScalarField>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Groth16Error::InvalidInput)?;
        ark_groth16::Groth16::<E>::verify_proof(&pvk, &proof, &inputs)
            .map_err(|_| Groth16Error::VerifyError)
    }
}

#[cfg(test)]
mod verify_proof_should {
    use super::*;
    use ark_bls12_381::Bls12_381;
    use ark_bn254::Bn254;
    use ark_ec::pairing::Pairing;
    use ark_ff::One;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case::bn254(PhantomData::<Bn254>)]
    #[case::bls12_381(PhantomData::<Bls12_381>)]
    fn curves<P: Pairing>(#[case] _p: P) {}

    #[apply(curves)]
    fn succeed<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (proof, vk, inputs) = Groth16Generic::<E>::get_instance(10, None);

        assert!(Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    #[apply(curves)]
    fn fail_with_wrong_vk<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (proof, _, inputs) = Groth16Generic::<E>::get_instance(10, Some(0));
        let (_, vk, _) = Groth16Generic::<E>::get_instance(10, Some(42));

        assert!(!Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    #[apply(curves)]
    fn fail_with_wrong_inputs<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (proof, vk, _) = Groth16Generic::<E>::get_instance(10, Some(0));
        let (_, _, inputs) = Groth16Generic::<E>::get_instance(10, Some(42));

        assert!(!Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    #[apply(curves)]
    fn fail_with_wrong_proof<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (_, vk, inputs) = Groth16Generic::<E>::get_instance(10, Some(0));
        let (proof, _, _) = Groth16Generic::<E>::get_instance(10, Some(42));

        assert!(!Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    #[apply(curves)]
    fn fail_with_malformed_proof<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (mut proof, vk, inputs) = Groth16Generic::<E>::get_instance(10, None);
        proof.a.0[0] += 1;

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::InvalidProof
        )
    }

    #[apply(curves)]
    fn fail_with_malformed_vk<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (proof, mut vk, inputs) = Groth16Generic::<E>::get_instance(10, None);
        vk.alpha_g1.0[0] += 1;

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::InvalidVerificationKey
        )
    }

    #[apply(curves)]
    fn fail_with_malformed_inputs<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (proof, vk, mut inputs) = Groth16Generic::<E>::get_instance(10, None);
        // tamper input so that it overflows scalar modulus
        for v in &mut inputs {
            for byte in &mut v.0 {
                *byte = 0xff;
            }
        }

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::InvalidInput
        )
    }

    #[apply(curves)]
    fn fail_with_too_many_inputs<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (proof, vk, mut inputs) = Groth16Generic::<E>::get_instance(10, None);
        inputs.push(Scalar::try_from_scalar(E::ScalarField::one()).unwrap());

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::VerifyError
        )
    }

    #[apply(curves)]
    fn fail_with_too_few_inputs<E: Pairing>(#[case] _p: PhantomData<E>) {
        let (proof, vk, mut inputs) = Groth16Generic::<E>::get_instance(10, None);
        inputs.pop();

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::VerifyError
        )
    }
}
