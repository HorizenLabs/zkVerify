// Copyright 2024, The Horizen Foundation
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

pub use native::ZKSYNC_PROOF_SIZE as PROOF_SIZE;
pub use native::ZKSYNC_PUBS_SIZE as PUBS_SIZE;
pub const FULL_PROOF_SIZE: usize = PROOF_SIZE + PUBS_SIZE;

/// This pallet provides ZkSync-Era verification.
pub use pallet::*;

pub type Proof = [u8; FULL_PROOF_SIZE];

#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weight;

#[frame_support::pallet]
pub mod pallet {
    use super::{weight, Proof, PROOF_SIZE, PUBS_SIZE};
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_system::pallet_prelude::OriginFor;
    use hp_poe::OnProofVerified;
    use sp_core::H256;
    use sp_io::hashing::keccak_256;
    use sp_std::boxed::Box;
    use weight::WeightInfo;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Provided data has not valid public inputs.
        InvalidInput,
        /// Provided data has not valid proof.
        InvalidProofData,
        /// Verify proof failed.
        VerifyError,
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config(with_default)]
    pub trait Config: frame_system::Config {
        #[pallet::no_default]
        type OnProofVerified: OnProofVerified;
        type WeightInfo: weight::WeightInfo;
    }

    pub mod config_preludes {
        #[cfg(test)]
        pub use testing::*;

        #[cfg(test)]
        mod testing {
            use frame_support::pallet_prelude::*;
            use frame_support::{derive_impl, register_default_impl};

            pub struct TestDefaultConfig;

            #[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig, no_aggregated_types)]
            impl frame_system::DefaultConfig for TestDefaultConfig {
                #[inject_runtime_type]
                type RuntimeEvent = ();
            }

            #[register_default_impl(TestDefaultConfig)]
            impl crate::pallet::DefaultConfig for TestDefaultConfig {
                type WeightInfo = ();
            }
        }
    }

    pub fn verify_proof<T: Config>(full_proof: Proof) -> Result<(), Error<T>> {
        let pubs_bytes = &full_proof[PROOF_SIZE..];
        let proof_bytes = &full_proof[..PROOF_SIZE];
        let pubs = <[u8; PUBS_SIZE]>::try_from(pubs_bytes)
            .map_err(|e| log::error!("Cannot get pubs data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;
        let raw_proof = <[u8; PROOF_SIZE]>::try_from(proof_bytes)
            .map_err(|e| log::error!("Cannot get raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;

        native::zksync_verify::verify(&raw_proof, pubs)
            .map(|_x| T::OnProofVerified::on_proof_verified(compute_hash(pubs_bytes)))
            .map_err(Into::into)
    }

    impl<T> From<native::VerifyError> for Error<T> {
        fn from(e: native::VerifyError) -> Self {
            match e {
                native::VerifyError::InvalidInput => Error::<T>::InvalidInput,
                native::VerifyError::InvalidProofData => Error::<T>::InvalidProofData,
                native::VerifyError::VerifyError => Error::<T>::VerifyError,
            }
        }
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        /// Submit a new ZkSync-Era proof.
        #[pallet::call_index(0)]
        pub fn submit_proof(
            _origin: OriginFor<T>,
            raw_proof: Box<Proof>,
        ) -> DispatchResultWithPostInfo {
            log::trace!("Submitting proof");
            verify_proof::<T>(*raw_proof)
                .map(Into::into)
                .map_err(Into::into)
        }
    }

    const PREFIX: &[u8; 7] = b"zksync-";
    fn compute_hash(pubs: &[u8]) -> H256 {
        let mut data_to_hash = PREFIX.to_vec();
        data_to_hash.extend_from_slice(pubs);
        H256(keccak_256(data_to_hash.as_slice()))
    }
}
