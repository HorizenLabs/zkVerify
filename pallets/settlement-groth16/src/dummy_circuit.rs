#![cfg(any(test, feature = "runtime-benchmarks"))]

use ark_crypto_primitives::snark::SNARK;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{rand::rngs::StdRng, rand::SeedableRng, UniformRand};
use sp_std::vec::Vec;

use crate::groth16_generic::{Groth16Generic, Proof, Scalar, VerificationKey};

#[derive(Clone, Debug)]
pub struct DummyCircuit<F: PrimeField> {
    pub inputs: Vec<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for DummyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        for input in self.inputs {
            let _ = cs.new_input_variable(|| Ok(input))?;
        }
        Ok(())
    }
}

impl<E: Pairing> Groth16Generic<E> {
    pub fn get_instance(
        num_inputs: usize,
        rng_seed: Option<u64>,
    ) -> (Proof, VerificationKey, Vec<Scalar>) {
        let rng = &mut StdRng::seed_from_u64(rng_seed.unwrap_or(0));

        let circuit = crate::dummy_circuit::DummyCircuit {
            inputs: (0..num_inputs).map(|_| E::ScalarField::rand(rng)).collect(),
        };

        let (pk, vk) =
            ark_groth16::Groth16::<E>::circuit_specific_setup(circuit.clone(), rng).unwrap();
        let proof = ark_groth16::Groth16::<E>::prove(&pk, circuit.clone(), rng).unwrap();

        let proof: Proof = proof.try_into().unwrap();
        let vk: VerificationKey = vk.try_into().unwrap();
        let inputs: Vec<Scalar> = circuit
            .inputs
            .into_iter()
            .map(|v| Scalar::try_from_scalar(v))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        return (proof, vk, inputs);
    }
}
