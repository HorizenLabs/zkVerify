#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use impl_trait_for_tuples;
use sp_core::H256;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weight;

#[impl_trait_for_tuples::impl_for_tuples(10)]
pub trait OnProofVerified {
    fn on_proof_verified(pubs_hash: H256);
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo};
    use frame_system::pallet_prelude::*;
    use sp_core::keccak_256;
    use super::weight::WeightInfo;
    use super::weight::SubstrateWeight;
    use super::OnProofVerified;
    use sp_core::H256; 

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type OnProofVerified: OnProofVerified;
    }

    type Proof = [u8; 25 * 32];
    pub fn verify_proof<T: Config>(full_proof: Proof) -> Result<(), Error<T>> {
        log::trace!("verifying proof");
        let pubs: fflonk_verifier::Public = (&full_proof[32 * 24..])
            .try_into()
            .map_err(|e| log::error!("Cannot extract public input: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;
        log::trace!("Extracted public input");
        let raw_proof = <[u8; 32 * 24]>::try_from(&full_proof[..32 * 24])
            .map_err(|e| log::error!("Cannot get raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
        log::trace!("Get proof data");
        let proof = fflonk_verifier::Proof::try_from(&raw_proof)
            .map_err(|e| log::debug!("Cannot extract raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
        log::trace!(
            "Extracted public inputs [{:?}...{:?}] and proof data [{:?}...{:?}]",
            &full_proof[PROOF_SIZE],
            &full_proof[FULL_PROOF_SIZE - 1],
            &full_proof[0],
            &full_proof[PROOF_SIZE - 1]
        );

        proof
            .verify(pubs)
            .map(|_x|T::OnProofVerified::on_proof_verified(compute_fflonk_hash(full_proof)))
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| Error::<T>::VerifyError)
    }

    fn compute_fflonk_hash(full_proof: Proof) -> H256 {
        let mut data_to_hash = b"fflonk-".to_vec();
        data_to_hash.extend_from_slice(&full_proof[32 * 24..]);
        let data_to_hash: [u8; 7 + 32] = data_to_hash.try_into().unwrap();
        let hash = H256(keccak_256(&data_to_hash));
        hash
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
        #[pallet::weight(SubstrateWeight::<T>::submit_proof())]
        pub fn submit_proof(_origin: OriginFor<T>, raw_proof: Proof) -> DispatchResultWithPostInfo {
            log::trace!("Enter");
            verify_proof::<T>(raw_proof)
                .map(Into::into)
                .map_err(Into::into)
        }
    }

    #[test]
    fn fflonk_hash_as_expected() {
        let proof = hex::decode(crate::tests::VALID_PROOF_HEX)
            .expect("Decoding failed")
            .try_into()
            .expect("Wrong size");
        let hash = compute_fflonk_hash(proof);
        let expected_hash = hex::decode(crate::tests::VALID_HASH)
            .expect("Decoding failed")
            .try_into()
            .expect("Wrong size");
        let expected_hash = H256(expected_hash);
        assert_eq!(hash, expected_hash);
    }
}
