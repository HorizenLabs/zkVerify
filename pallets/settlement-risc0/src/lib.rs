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

/// This pallet provides risc0 verification.
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;
mod weight;

pub use weight::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::WeightInfo;
    use frame_support::{dispatch::DispatchResultWithPostInfo, ensure, pallet_prelude::Get};
    use frame_system::pallet_prelude::OriginFor;
    use hp_poe::OnProofVerified;
    use sp_core::H256;
    use sp_io::hashing::keccak_256;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Provided proof is not valid.
        InvalidProof,
        /// Provided proof has excessive size.
        InvalidProofSize,
        /// Provided public inputs are not valid.
        InvalidPublicInputs,
        /// Provided public inputs have excessive size.
        InvalidPublicInputsSize,
        /// Verify proof failed.
        VerifyError,
    }

    impl<T> From<native::VerifyError> for Error<T> {
        fn from(e: native::VerifyError) -> Self {
            match e {
                native::VerifyError::InvalidInput => Error::<T>::InvalidPublicInputs,
                native::VerifyError::InvalidProofData => Error::<T>::InvalidProof,
                native::VerifyError::VerifyError => Error::<T>::VerifyError,
            }
        }
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Proof verified call back
        type OnProofVerified: OnProofVerified;
        /// The weight definition for this pallet
        type WeightInfo: WeightInfo;
        /// Maximum number of bytes contained in the proof (otherwise rejected)
        type MaxProofSize: Get<u32>;
        /// Maximum number of bytes contained in the public inputs (otherwise rejected)
        type MaxPubsSize: Get<u32>;
    }

    pub fn verify_proof<T: Config>(
        vk: [u8; 32],
        proof: Vec<u8>,
        pubs: Vec<u8>,
    ) -> Result<(), Error<T>> {
        log::trace!("Checking size");
        ensure!(
            (proof).len() <= T::MaxProofSize::get() as usize,
            Error::<T>::InvalidProofSize
        );
        ensure!(
            (pubs).len() <= T::MaxPubsSize::get() as usize,
            Error::<T>::InvalidPublicInputsSize
        );

        log::trace!("Verifying (native)");
        native::risc_0_verify::verify(vk, &proof, &pubs)
            .map(|_x| T::OnProofVerified::on_proof_verified(compute_risc0_hash(vk, pubs)))
            .map_err(Into::into)
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        pub fn submit_proof(
            _origin: OriginFor<T>,
            vk_u8: [u8; 32],
            proof: Vec<u8>,
            pubs: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            log::trace!("Submitting proof");
            verify_proof::<T>(vk_u8, proof, pubs)
                .map(Into::into)
                .map_err(Into::into)
        }
    }

    const PREFIX: &[u8; 6] = b"risc0-";
    fn compute_risc0_hash(vk: [u8; 32], pubs: Vec<u8>) -> H256 {
        let mut data_to_hash = PREFIX.to_vec();
        data_to_hash.extend_from_slice(&vk);
        data_to_hash.extend_from_slice(b"-");
        data_to_hash.extend_from_slice(&pubs);
        H256(keccak_256(data_to_hash.as_slice()))
    }
}
