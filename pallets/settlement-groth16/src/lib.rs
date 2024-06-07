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

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
pub mod mock;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod dummy_circuit;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod data_structures;
mod groth16;
mod groth16_generic;
mod weight;

pub use weight::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::WeightInfo;
    use crate::groth16::{Curve, Groth16, Groth16Error, Proof, Scalar, VerificationKeyWithCurve};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use hp_poe::OnProofVerified;
    use sp_core::H256;
    use sp_io::hashing::keccak_256;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type OnProofVerified: OnProofVerified;
        type WeightInfo: WeightInfo;
        const MAX_NUM_INPUTS: u32;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Public inputs are malformed.
        InvalidInput,
        /// Proof is malformed.
        InvalidProof,
        /// Verification key is malformed.
        InvalidVerificationKey,
        /// Too many inputs.
        TooManyInputs,
        /// Number of inputs not coherent with verification key.
        VkAndInputsMismatch,
        /// Proof verification failed.
        VerifyError,
    }

    impl<T> From<Groth16Error> for Error<T> {
        fn from(e: Groth16Error) -> Self {
            match e {
                Groth16Error::InvalidInput => Error::<T>::InvalidInput,
                Groth16Error::InvalidProof => Error::<T>::InvalidProof,
                Groth16Error::VerifyError => Error::<T>::VerifyError,
                Groth16Error::InvalidVerificationKey => Error::<T>::InvalidVerificationKey,
            }
        }
    }

    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(
            match vk.curve {
                Curve::Bn254 => T::WeightInfo::submit_proof_bn254(input.len() as u32 ),
                Curve::Bls12_381 => T::WeightInfo::submit_proof_bls12_381(input.len() as u32 ),
            }
        )]
        pub fn submit_proof(
            _origin: OriginFor<T>,
            proof: Proof,
            vk: Box<VerificationKeyWithCurve>,
            input: Vec<Scalar>,
        ) -> DispatchResult {
            if input.len() > T::MAX_NUM_INPUTS as usize {
                return Err(Into::into(Error::<T>::TooManyInputs));
            }
            if input.len() + 1 != vk.gamma_abc_g1.len() {
                return Err(Into::into(Error::<T>::VkAndInputsMismatch));
            }

            let successful_verification = Groth16::verify_proof(proof, *vk.clone(), &input)
                .map_err(<Groth16Error as Into<Error<T>>>::into)?;

            if successful_verification {
                T::OnProofVerified::on_proof_verified(compute_groth16_hash(&vk, &input));
                Ok(())
            } else {
                Err(Into::into(Error::<T>::VerifyError))
            }
        }
    }

    pub fn compute_groth16_hash(vk: &VerificationKeyWithCurve, input: &[Scalar]) -> H256 {
        const PREFIX: &str = "groth16-";
        let vk_hash = keccak_256(vk.encode().as_slice());
        let input_hash = keccak_256(input.encode().as_slice());
        H256(keccak_256(
            &[PREFIX.as_bytes(), vk_hash.as_slice(), input_hash.as_slice()].concat(),
        ))
    }
}
