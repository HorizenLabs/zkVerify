#![cfg_attr(not(feature = "std"), no_std)]

/// This pallet provides FFlonk verification for CDK prover.
pub use pallet::*;
use sp_core::H256;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weight;

#[impl_trait_for_tuples::impl_for_tuples(10)]
pub trait OnProofVerified {
    fn on_proof_verified(pubs_hash: H256);
}

pub const FULL_PROOF_SIZE: usize = 25 * 32;
pub const PUBS_SIZE: usize = 32;
pub const PROOF_SIZE: usize = 24 * 32;
pub type Proof = [u8; FULL_PROOF_SIZE];

#[frame_support::pallet]
pub mod pallet {
    use super::weight::SubstrateWeight;
    use super::weight::WeightInfo;
    use super::{OnProofVerified, Proof, FULL_PROOF_SIZE, PROOF_SIZE};
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use sp_io::hashing::keccak_256;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type OnProofVerified: OnProofVerified;
    }

    pub fn verify_proof<T: Config>(full_proof: Proof) -> Result<(), Error<T>> {
        let pubs: fflonk_verifier::Public = (&full_proof[PROOF_SIZE..])
            .try_into()
            .map_err(|e| log::error!("Cannot extract public inputs: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;
        let raw_proof = <[u8; PROOF_SIZE]>::try_from(&full_proof[..PROOF_SIZE])
            .map_err(|e| log::error!("Cannot get raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
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
            .map(|_x| T::OnProofVerified::on_proof_verified(compute_fflonk_hash(full_proof)))
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| Error::<T>::VerifyError)
    }

    const PREFIX: &[u8; 7] = b"fflonk-";
    fn compute_fflonk_hash(full_proof: Proof) -> H256 {
        let mut data_to_hash = PREFIX.to_vec();
        data_to_hash.extend_from_slice(&full_proof[PROOF_SIZE..]);
        H256(keccak_256(data_to_hash.as_slice()))
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
            log::trace!("Submitting proof");
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
