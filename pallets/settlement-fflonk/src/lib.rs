// Copyright 2024, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

/// This pallet provides FFlonk verification for CDK prover.
pub use pallet::*;

mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;
mod vk;
mod weight;

pub use weight::WeightInfo;
pub const FULL_PROOF_SIZE: usize = 25 * 32;
pub const PUBS_SIZE: usize = 32;
pub const PROOF_SIZE: usize = 24 * 32;
pub type Proof = [u8; FULL_PROOF_SIZE];

#[frame_support::pallet]
pub mod pallet {
    use super::{vk::Vk, Proof, WeightInfo, FULL_PROOF_SIZE, PROOF_SIZE};
    use codec::Encode;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, Identity};
    use frame_system::pallet_prelude::*;
    use hp_poe::OnProofVerified;
    use sp_core::H256;
    use sp_io::hashing::keccak_256;
    use sp_std::boxed::Box;

    #[derive(Debug, Clone, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen)]
    pub enum VkOrHash {
        Vk(Vk),
        Hash(H256),
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Proof verified call back
        type OnProofVerified: OnProofVerified;
        /// The weight definition for this pallet
        type WeightInfo: WeightInfo;
    }

    pub fn verify_proof<T: Config>(
        vk: &fflonk_verifier::VerificationKey,
        full_proof: Proof,
    ) -> Result<(), Error<T>> {
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

        fflonk_verifier::verify(vk, &proof, &pubs)
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| Error::<T>::VerifyError)
    }

    /// Pallet specific events.
    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        VkRegistered { hash: H256 },
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
        /// Provided an invalid verification key.
        InvalidVerificationKey,
        /// Provided an unregistered verification key hash.
        VerificationKeyNotFound,
    }

    #[pallet::storage]
    #[pallet::getter(fn vks)]
    pub type Vks<T> = StorageMap<Hasher = Identity, Key = H256, Value = Vk>;

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(
            match &vk_or_hash {
                Some(VkOrHash::Vk(_)) => T::WeightInfo::submit_proof_with_vk(),
                Some(VkOrHash::Hash(_)) => T::WeightInfo::submit_proof_with_vk_hash(),
                None => T::WeightInfo::submit_proof_default(),
            })]
        pub fn submit_proof(
            _origin: OriginFor<T>,
            raw_proof: Box<Proof>,
            vk_or_hash: Option<VkOrHash>,
        ) -> DispatchResultWithPostInfo {
            log::trace!("Submitting proof");
            let vk = match &vk_or_hash {
                Some(VkOrHash::Hash(h)) => Vks::<T>::get(h)
                    .ok_or(Error::<T>::VerificationKeyNotFound)?
                    .try_into()
                    .map_err(|_| Error::<T>::InvalidVerificationKey)?,
                Some(VkOrHash::Vk(vk)) => vk
                    .clone()
                    .try_into()
                    .map_err(|_| Error::<T>::InvalidVerificationKey)?,
                None => fflonk_verifier::VerificationKey::default(),
            };
            verify_proof::<T>(&vk, *raw_proof)
                .map(|_x| {
                    T::OnProofVerified::on_proof_verified(compute_fflonk_hash(
                        *raw_proof, vk_or_hash,
                    ))
                })
                .map(Into::into)
                .map_err(Into::into)
        }

        #[pallet::call_index(1)]
        pub fn register_vk(_origin: OriginFor<T>, vk: Vk) -> DispatchResultWithPostInfo {
            log::trace!("Register vk");
            let _: fflonk_verifier::VerificationKey = vk
                .clone()
                .try_into()
                .map_err(|_| Error::<T>::InvalidVerificationKey)?;
            let hash = hash_vk(&vk);
            Vks::<T>::insert(hash, vk);
            Self::deposit_event(Event::VkRegistered { hash });
            Ok(().into())
        }
    }

    const PREFIX: &[u8; 6] = b"fflonk";
    fn compute_fflonk_hash(full_proof: Proof, vk_or_hash: Option<VkOrHash>) -> H256 {
        let mut data_to_hash = PREFIX.to_vec();
        if let Some(vk_or_hash) = vk_or_hash {
            let hash = match vk_or_hash {
                VkOrHash::Vk(vk) => hash_vk(&vk),
                VkOrHash::Hash(hash) => hash,
            };
            data_to_hash.extend_from_slice(b"-");
            data_to_hash.extend_from_slice(hash.as_bytes());
        }
        data_to_hash.extend_from_slice(b"-");
        data_to_hash.extend_from_slice(&full_proof[PROOF_SIZE..]);
        H256(keccak_256(data_to_hash.as_slice()))
    }

    fn hash_vk(vk: &Vk) -> H256 {
        H256(keccak_256(vk.encode().as_slice()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case::no_vk(None, crate::tests::VALID_HASH)]
        #[case::default_vk_hash(
            Some(VkOrHash::Hash(crate::tests::DEFAULT_VK_HASH)),
            crate::tests::VALID_HASH_WITH_VK
        )]
        fn fflonk_hash_as_expected(#[case] vk_hash: Option<VkOrHash>, #[case] expected: H256) {
            let hash = compute_fflonk_hash(crate::tests::VALID_PROOF, vk_hash);

            assert_eq!(hash, expected);
        }
    }
}
