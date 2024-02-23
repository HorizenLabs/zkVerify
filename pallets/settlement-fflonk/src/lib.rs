#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
    }

    type Proof = [u8; 25 * 32];
    pub fn verify_proof<T>(raw_proof: Proof) -> Result<(), Error<T>> {
        log::trace!("verifying proof");
        let pubs: fflonk_verifier::Public = (&raw_proof[32 * 24..])
            .try_into()
            .map_err(|e| log::error!("Cannot extract public input: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;
        log::trace!("Extracted public input");
        let raw_proof = <[u8; 32 * 24]>::try_from(&raw_proof[..32 * 24])
            .map_err(|e| log::error!("Cannot get raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
        log::trace!("Get proof data");
        let proof = fflonk_verifier::Proof::try_from(&raw_proof)
            .map_err(|e| log::debug!("Cannot extract raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
        log::trace!("Extracted proof data");
        proof
            .verify(pubs)
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| Error::<T>::VerifyError)
    }

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

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn submit_proof(_origin: OriginFor<T>, raw_proof: Proof) -> DispatchResultWithPostInfo {
            log::trace!("Enter");
            verify_proof::<T>(raw_proof)
                .map(Into::into)
                .map_err(Into::into)
        }
    }
}