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

use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::PrimeField;
use ark_serialize::SerializationError;
use codec::{Decode, Encode, MaxEncodedLen};
use core::fmt::Debug;
use scale_info::TypeInfo;
use sp_std::vec;
use sp_std::vec::Vec;

pub const G1_MAX_SIZE: u32 = 48;
pub const G2_MAX_SIZE: u32 = G1_MAX_SIZE * 2;

pub fn vec_max_encoded_len(max_vec_len: u32) -> usize {
    codec::Compact(max_vec_len).encoded_size() + max_vec_len as usize
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct G1(pub Vec<u8>);

impl MaxEncodedLen for G1 {
    fn max_encoded_len() -> usize {
        vec_max_encoded_len(G1_MAX_SIZE)
    }
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct G2(pub Vec<u8>);

impl MaxEncodedLen for G2 {
    fn max_encoded_len() -> usize {
        vec_max_encoded_len(G2_MAX_SIZE)
    }
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct Scalar(pub Vec<u8>);

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct Proof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct VerificationKey {
    pub alpha_g1: G1,
    pub beta_g2: G2,
    pub gamma_g2: G2,
    pub delta_g2: G2,
    pub gamma_abc_g1: Vec<G1>,
}

impl G1 {
    pub fn try_into_affine<R: AffineRepr>(self) -> Result<R, SerializationError> {
        R::deserialize_uncompressed(self.0.as_ref())
    }

    pub fn try_from_affine<R: AffineRepr>(value: R) -> Result<Self, SerializationError> {
        let mut result = Self(vec![0; value.uncompressed_size()]);
        value.serialize_uncompressed(result.0.as_mut_slice())?;
        Ok(result)
    }
}

impl G2 {
    pub fn try_into_affine<R: AffineRepr>(self) -> Result<R, SerializationError> {
        R::deserialize_uncompressed(self.0.as_ref())
    }

    pub fn try_from_affine<R: AffineRepr>(value: R) -> Result<Self, SerializationError> {
        let mut result = Self(vec![0; value.uncompressed_size()]);
        value.serialize_uncompressed(result.0.as_mut_slice())?;
        Ok(result)
    }
}

impl Scalar {
    pub fn try_into_scalar<P: PrimeField>(self) -> Result<P, SerializationError> {
        P::deserialize_uncompressed(self.0.as_ref())
    }

    pub fn try_from_scalar<P: PrimeField>(value: P) -> Result<Self, SerializationError> {
        let mut result = Self(vec![0; value.uncompressed_size()]);
        value.serialize_uncompressed(result.0.as_mut_slice())?;
        Ok(result)
    }
}

impl VerificationKey {
    pub fn num_inputs(&self) -> usize {
        self.gamma_abc_g1.len() - 1
    }
}

impl<E: Pairing> TryInto<ark_groth16::Proof<E>> for Proof {
    type Error = SerializationError;

    fn try_into(self) -> Result<ark_groth16::Proof<E>, Self::Error> {
        Ok(ark_groth16::Proof {
            a: self.a.try_into_affine::<E::G1Affine>()?,
            b: self.b.try_into_affine::<E::G2Affine>()?,
            c: self.c.try_into_affine::<E::G1Affine>()?,
        })
    }
}

impl<E: Pairing> TryFrom<VerificationKey> for ark_groth16::VerifyingKey<E> {
    type Error = SerializationError;

    fn try_from(value: VerificationKey) -> Result<Self, Self::Error> {
        Ok(ark_groth16::VerifyingKey {
            alpha_g1: value.alpha_g1.try_into_affine::<E::G1Affine>()?,
            beta_g2: value.beta_g2.try_into_affine::<E::G2Affine>()?,
            gamma_g2: value.gamma_g2.try_into_affine::<E::G2Affine>()?,
            delta_g2: value.delta_g2.try_into_affine::<E::G2Affine>()?,
            gamma_abc_g1: value
                .gamma_abc_g1
                .into_iter()
                .map(|v| v.try_into_affine::<E::G1Affine>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl<E: Pairing> TryFrom<ark_groth16::Proof<E>> for Proof {
    type Error = SerializationError;

    fn try_from(value: ark_groth16::Proof<E>) -> Result<Self, Self::Error> {
        Ok(Proof {
            a: G1::try_from_affine(value.a)?,
            b: G2::try_from_affine(value.b)?,
            c: G1::try_from_affine(value.c)?,
        })
    }
}

impl<E: Pairing> TryFrom<ark_groth16::VerifyingKey<E>> for VerificationKey {
    type Error = SerializationError;

    fn try_from(value: ark_groth16::VerifyingKey<E>) -> Result<Self, Self::Error> {
        Ok(VerificationKey {
            alpha_g1: G1::try_from_affine(value.alpha_g1)?,
            beta_g2: G2::try_from_affine(value.beta_g2)?,
            gamma_g2: G2::try_from_affine(value.gamma_g2)?,
            delta_g2: G2::try_from_affine(value.delta_g2)?,
            gamma_abc_g1: value
                .gamma_abc_g1
                .into_iter()
                .map(G1::try_from_affine)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[cfg(test)]
#[macro_use]
mod test {
    use super::*;
    use ark_bls12_381::Bls12_381;
    use ark_bn254::Bn254;
    use ark_ec::pairing::Pairing;
    use ark_ff::UniformRand;
    use ark_std::rand::{rngs::StdRng, SeedableRng};
    use core::marker::PhantomData;
    use frame_support::assert_ok;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    mod deserialize {
        use super::*;
        include!("resources.rs");

        #[rstest]
        #[case::bn254(PhantomData::<Bn254>, g1_bn254())]
        #[case::bls12_381(PhantomData::<Bls12_381>, g1_bls12_381())]
        fn g1<E: Pairing>(#[case] _p: PhantomData<E>, #[case] serialized_repr: G1) {
            assert_ok!(serialized_repr.try_into_affine::<E::G1Affine>());
        }

        #[rstest]
        #[case::bn254(PhantomData::<Bn254>, g2_bn254())]
        #[case::bls12_381(PhantomData::<Bls12_381>, g2_bls12_381())]
        fn g2<E: Pairing>(#[case] _p: PhantomData<E>, #[case] serialized_repr: G2) {
            assert_ok!(serialized_repr.try_into_affine::<E::G2Affine>());
        }

        #[rstest]
        #[case::bn254(PhantomData::<Bn254>, scalar_bn254())]
        #[case::bls12_381(PhantomData::<Bls12_381>, scalar_bls12_381())]
        fn scalar<E: Pairing>(#[case] _p: PhantomData<E>, #[case] serialized_repr: Scalar) {
            assert_ok!(serialized_repr.try_into_scalar::<E::ScalarField>());
        }

        #[rstest]
        #[case::bn254(PhantomData::<Bn254>, proof_bn254())]
        #[case::bls12_381(PhantomData::<Bls12_381>, proof_bls12_381())]
        fn proof<E: Pairing>(#[case] _p: PhantomData<E>, #[case] serialized_repr: Proof) {
            let deserialized_proof: Result<ark_groth16::Proof<E>, _> = serialized_repr.try_into();
            assert_ok!(deserialized_proof);
        }

        #[rstest]
        #[case::bn254(PhantomData::<Bn254>, verification_key_bn254())]
        #[case::bls12_381(PhantomData::<Bls12_381>, verification_key_bls12_381())]
        fn verification_key<E: Pairing>(
            #[case] _p: PhantomData<E>,
            #[case] serialized_repr: VerificationKey,
        ) {
            let deserialized_vk: Result<ark_groth16::VerifyingKey<E>, _> =
                serialized_repr.try_into();
            assert_ok!(deserialized_vk);
        }
    }

    #[template]
    #[rstest]
    #[case::bn254(PhantomData::<Bn254>)]
    #[case::bls12_381(PhantomData::<Bls12_381>)]
    fn curves<P: Pairing>(#[case] _p: P) {}

    mod serialize_and_deserialize {
        use super::*;

        #[apply(curves)]
        fn g1<E: Pairing>(_p: PhantomData<E>) {
            let mut rng = StdRng::seed_from_u64(0);

            let point: E::G1Affine = <E::G1 as UniformRand>::rand(&mut rng).into();
            let serialized_point = G1::try_from_affine(point).unwrap();
            let deserialized_point: E::G1Affine = serialized_point.try_into_affine().unwrap();

            assert_eq!(point, deserialized_point);
        }

        #[apply(curves)]
        fn g2<E: Pairing>(_p: PhantomData<E>) {
            let mut rng = StdRng::seed_from_u64(0);

            let point: E::G2Affine = <E::G2 as UniformRand>::rand(&mut rng).into();
            let serialized_point = G2::try_from_affine(point).unwrap();
            let deserialized_point: E::G2Affine = serialized_point.try_into_affine().unwrap();

            assert_eq!(point, deserialized_point);
        }

        #[apply(curves)]
        fn scalar<E: Pairing>(_p: PhantomData<E>) {
            let mut rng = StdRng::seed_from_u64(0);

            let scalar: E::ScalarField = <E::ScalarField as UniformRand>::rand(&mut rng);
            let serialized_scalar = Scalar::try_from_scalar(scalar).unwrap();
            let deserialized_scalar: E::ScalarField = serialized_scalar.try_into_scalar().unwrap();

            assert_eq!(scalar, deserialized_scalar);
        }

        #[apply(curves)]
        fn proof<E: Pairing>(_p: PhantomData<E>) {
            let mut rng = StdRng::seed_from_u64(0);

            let proof = ark_groth16::Proof::<E> {
                a: <E::G1 as UniformRand>::rand(&mut rng).into(),
                b: <E::G2 as UniformRand>::rand(&mut rng).into(),
                c: <E::G1 as UniformRand>::rand(&mut rng).into(),
            };

            let serialized_proof: Proof = proof.clone().try_into().unwrap();
            let deserialized_proof: ark_groth16::Proof<E> = serialized_proof.try_into().unwrap();

            assert_eq!(proof, deserialized_proof);
        }

        #[apply(curves)]
        fn verification_key<E: Pairing>(_p: PhantomData<E>) {
            let mut rng = StdRng::seed_from_u64(0);

            let vk = ark_groth16::VerifyingKey::<E> {
                alpha_g1: <E::G1 as UniformRand>::rand(&mut rng).into(),
                beta_g2: <E::G2 as UniformRand>::rand(&mut rng).into(),
                gamma_g2: <E::G2 as UniformRand>::rand(&mut rng).into(),
                delta_g2: <E::G2 as UniformRand>::rand(&mut rng).into(),
                gamma_abc_g1: vec![
                    <E::G1 as UniformRand>::rand(&mut rng).into(),
                    <E::G1 as UniformRand>::rand(&mut rng).into(),
                ],
            };

            let serialized_vk: VerificationKey = vk.clone().try_into().unwrap();
            let deserialized_vk: ark_groth16::VerifyingKey<E> = serialized_vk.try_into().unwrap();

            assert_eq!(vk, deserialized_vk);
        }
    }
}
