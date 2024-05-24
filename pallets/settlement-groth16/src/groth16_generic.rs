use ark_ec::pairing::Pairing;
use ark_groth16::prepare_verifying_key;
use core::marker::PhantomData;

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
            .into_iter()
            .map(|v| v.clone().try_into_scalar::<E::ScalarField>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Groth16Error::InvalidInput)?;
        ark_groth16::Groth16::<E>::verify_proof(&pvk, &proof, &inputs)
            .map_err(|_| Groth16Error::VerifyError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bls12_381::Bls12_381;
    use ark_bn254::Bn254;
    use ark_ec::pairing::Pairing;
    use ark_std::One;

    fn verify_succeeds<E: Pairing>() {
        let (proof, vk, inputs) = Groth16Generic::<E>::get_instance(10, None);

        assert!(Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    fn verify_with_wrong_vk_fails<E: Pairing>() {
        let (proof, _, inputs) = Groth16Generic::<E>::get_instance(10, Some(0));
        let (_, vk, _) = Groth16Generic::<E>::get_instance(10, Some(42));

        assert!(!Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    fn verify_with_wrong_inputs_fails<E: Pairing>() {
        let (proof, vk, _) = Groth16Generic::<E>::get_instance(10, Some(0));
        let (_, _, inputs) = Groth16Generic::<E>::get_instance(10, Some(42));

        assert!(!Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    fn verify_with_wrong_proof_fails<E: Pairing>() {
        let (_, vk, inputs) = Groth16Generic::<E>::get_instance(10, Some(0));
        let (proof, _, _) = Groth16Generic::<E>::get_instance(10, Some(42));

        assert!(!Groth16Generic::<E>::verify_proof(proof, vk, &inputs).unwrap())
    }

    fn verify_with_malformed_proof_fails_with_correct_error<E: Pairing>() {
        let (mut proof, vk, inputs) = Groth16Generic::<E>::get_instance(10, None);
        proof.a.0[0] += 1;

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::InvalidProof
        )
    }

    fn verify_with_malformed_vk_fails_with_correct_error<E: Pairing>() {
        let (proof, mut vk, inputs) = Groth16Generic::<E>::get_instance(10, None);
        vk.alpha_g1.0[0] += 1;

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::InvalidVerificationKey
        )
    }

    fn verify_with_malformed_inputs_fails_with_correct_error<E: Pairing>() {
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

    fn verify_with_too_many_inputs_fails_with_correct_error<E: Pairing>() {
        let (proof, vk, mut inputs) = Groth16Generic::<E>::get_instance(10, None);
        inputs.push(Scalar::try_from_scalar(E::ScalarField::one()).unwrap());

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::VerifyError
        )
    }

    fn verify_with_too_few_inputs_fails_with_correct_error<E: Pairing>() {
        let (proof, vk, mut inputs) = Groth16Generic::<E>::get_instance(10, None);
        inputs.pop();

        assert_eq!(
            Groth16Generic::<E>::verify_proof(proof, vk, &inputs)
                .err()
                .unwrap(),
            Groth16Error::VerifyError
        )
    }

    #[test]
    fn verify_succeeds_bn254() {
        verify_succeeds::<Bn254>()
    }

    #[test]
    fn verify_with_wrong_vk_fails_bn254() {
        verify_with_wrong_vk_fails::<Bn254>()
    }

    #[test]
    fn verify_with_wrong_inputs_fails_bn254() {
        verify_with_wrong_inputs_fails::<Bn254>()
    }

    #[test]
    fn verify_with_wrong_proof_fails_bn254() {
        verify_with_wrong_proof_fails::<Bn254>()
    }

    #[test]
    fn verify_with_malformed_proof_fails_with_correct_error_bn254() {
        verify_with_malformed_proof_fails_with_correct_error::<Bn254>()
    }

    #[test]
    fn verify_with_malformed_vk_fails_with_correct_error_bn254() {
        verify_with_malformed_vk_fails_with_correct_error::<Bn254>()
    }

    #[test]
    fn verify_with_malformed_inputs_fails_with_correct_error_bn254() {
        verify_with_malformed_inputs_fails_with_correct_error::<Bn254>()
    }

    #[test]
    fn verify_with_too_many_inputs_fails_with_correct_error_bn254() {
        verify_with_too_many_inputs_fails_with_correct_error::<Bn254>()
    }

    #[test]
    fn verify_with_too_few_inputs_fails_with_correct_error_bn254() {
        verify_with_too_few_inputs_fails_with_correct_error::<Bn254>()
    }

    #[test]
    fn verify_succeeds_bls12_381() {
        verify_succeeds::<Bls12_381>()
    }

    #[test]
    fn verify_with_wrong_vk_fails_bls12_381() {
        verify_with_wrong_vk_fails::<Bls12_381>()
    }

    #[test]
    fn verify_with_wrong_inputs_fails_bls12_381() {
        verify_with_wrong_inputs_fails::<Bls12_381>()
    }

    #[test]
    fn verify_with_wrong_proof_fails_bls12_381() {
        verify_with_wrong_proof_fails::<Bls12_381>()
    }

    #[test]
    fn verify_with_malformed_proof_fails_with_correct_error_bls12_381() {
        verify_with_malformed_proof_fails_with_correct_error::<Bls12_381>()
    }

    #[test]
    fn verify_with_malformed_vk_fails_with_correct_error_bls12_381() {
        verify_with_malformed_vk_fails_with_correct_error::<Bls12_381>()
    }

    #[test]
    fn verify_with_malformed_inputs_fails_with_correct_error_bls12_381() {
        verify_with_malformed_inputs_fails_with_correct_error::<Bls12_381>()
    }

    #[test]
    fn verify_with_too_many_inputs_fails_with_correct_error_bls12_381() {
        verify_with_too_many_inputs_fails_with_correct_error::<Bls12_381>()
    }

    #[test]
    fn verify_with_too_few_inputs_fails_with_correct_error_bls12_381() {
        verify_with_too_few_inputs_fails_with_correct_error::<Bls12_381>()
    }
}
