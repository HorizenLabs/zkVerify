use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::PrimeField;
use ark_serialize::SerializationError;
use codec::{Decode, Encode};
use core::fmt::Debug;
use scale_info::TypeInfo;

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct G1(pub Vec<u8>);

#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub struct G2(pub Vec<u8>);

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

impl<E: Pairing> TryInto<ark_groth16::VerifyingKey<E>> for VerificationKey {
    type Error = SerializationError;

    fn try_into(self) -> Result<ark_groth16::VerifyingKey<E>, Self::Error> {
        Ok(ark_groth16::VerifyingKey {
            alpha_g1: self.alpha_g1.try_into_affine::<E::G1Affine>()?,
            beta_g2: self.beta_g2.try_into_affine::<E::G2Affine>()?,
            gamma_g2: self.gamma_g2.try_into_affine::<E::G2Affine>()?,
            delta_g2: self.delta_g2.try_into_affine::<E::G2Affine>()?,
            gamma_abc_g1: self
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
                .map(|v| G1::try_from_affine(v))
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

    fn serialize_deserialize_g1<E: Pairing>() {
        let mut rng = StdRng::seed_from_u64(0);

        let point: E::G1Affine = <E::G1 as UniformRand>::rand(&mut rng).into();
        let serialized_point = G1::try_from_affine(point.clone()).unwrap();
        let deserialized_point: E::G1Affine = serialized_point.try_into_affine().unwrap();

        assert_eq!(point, deserialized_point);
    }

    fn serialize_deserialize_g2<E: Pairing>() {
        let mut rng = StdRng::seed_from_u64(0);

        let point: E::G2Affine = <E::G2 as UniformRand>::rand(&mut rng).into();
        let serialized_point = G2::try_from_affine(point.clone()).unwrap();
        let deserialized_point: E::G2Affine = serialized_point.try_into_affine().unwrap();

        assert_eq!(point, deserialized_point);
    }

    fn serialize_deserialize_scalar<E: Pairing>() {
        let mut rng = StdRng::seed_from_u64(0);

        let scalar: E::ScalarField = <E::ScalarField as UniformRand>::rand(&mut rng).into();
        let serialized_scalar = Scalar::try_from_scalar(scalar.clone()).unwrap();
        let deserialized_scalar: E::ScalarField = serialized_scalar.try_into_scalar().unwrap();

        assert_eq!(scalar, deserialized_scalar);
    }

    fn serialize_deserialize_proof<E: Pairing>() {
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

    fn serialize_deserialize_verification_key<E: Pairing>() {
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

    #[test]
    fn serialize_deserialize_g1_bn254() {
        serialize_deserialize_g1::<Bn254>()
    }

    #[test]
    fn serialize_deserialize_g2_bn254() {
        serialize_deserialize_g2::<Bn254>()
    }

    #[test]
    fn serialize_deserialize_scalar_bn254() {
        serialize_deserialize_scalar::<Bn254>()
    }

    #[test]
    fn serialize_deserialize_proof_bn254() {
        serialize_deserialize_proof::<Bn254>()
    }

    #[test]
    fn serialize_deserialize_verification_key_bn254() {
        serialize_deserialize_verification_key::<Bn254>()
    }

    #[test]
    fn serialize_deserialize_proof_bls12_381() {
        serialize_deserialize_proof::<Bls12_381>()
    }

    #[test]
    fn serialize_deserialize_verification_key_bls12_381() {
        serialize_deserialize_verification_key::<Bls12_381>()
    }
}
