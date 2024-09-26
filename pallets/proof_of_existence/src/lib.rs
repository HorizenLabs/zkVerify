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
pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub mod mock;

mod migration;

mod benchmarking;

mod weight;
pub use weight::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::WeightInfo;
    use binary_merkle_tree::MerkleProof;
    use pallet_timestamp::{self as timestamp};

    use sp_core::H256;
    use sp_std::{collections::btree_set::BTreeSet, result};

    use frame_support::pallet_prelude::{OptionQuery, *};
    use frame_support::sp_runtime::traits::{Keccak256, SaturatedConversion};
    use frame_system::pallet_prelude::*;

    use hp_poe::{InherentError, InherentType, INHERENT_IDENTIFIER};
    use sp_std::vec;
    use sp_std::vec::Vec;

    use crate::migration;

    #[derive(Clone, TypeInfo, PartialEq, Eq, Encode, Decode, Debug)]
    pub enum AttestationPathRequestError {
        ProofNotFound(u64, sp_core::H256),
        AttestationIdNotPublished(u64),
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + timestamp::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Maximum time (ms) that an element can wait in a tree before the tree is published
        type MaxElapsedTimeMs: Get<Self::Moment>;
        /// Number of proofs that a single attestation contain
        type ProofsPerAttestation: Get<u32>;
        /// Max number of attestations to be cleared in the next block
        type MaxAttestationsToClear: Get<u32>;
        /// The weight definition for this pallet
        type WeightInfo: WeightInfo;
    }

    impl<T: Config> hp_poe::OnProofVerified for Pallet<T> {
        fn on_proof_verified(pubs_hash: H256) {
            Self::insert(pubs_hash);
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn oldest_attestation)]
    pub type OldestAttestation<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_attestation)]
    pub type NextAttestation<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_publish_time)]
    pub type FirstInsertionTime<T: Config> = StorageValue<_, T::Moment, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn attestations_with_proofs_to_be_published)]
    pub type AttestationsWithProofsToBePublished<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, BoundedVec<H256, T::ProofsPerAttestation>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn attestations_to_be_cleared)]
    pub type AttestationsToBeCleared<T: Config> =
        StorageValue<_, BoundedVec<u64, T::MaxAttestationsToClear>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn values)]
    pub type Values<T> = StorageDoubleMap<
        Hasher1 = Blake2_128Concat,
        Key1 = u64,
        Hasher2 = Blake2_128Concat,
        Key2 = H256,
        Value = (),
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewElement { value: H256, attestation_id: u64 },
        NewAttestation { id: u64, attestation: H256 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    #[derive(Clone, PartialEq, Eq)]
    pub enum Error<T> {
        TooEarlyForASmallTree,
        AttestationsToBeClearedMaxReached,
    }

    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        /// Publish the attestations of Merkle trees and move to the next tree.
        #[pallet::call_index(0)]
        pub fn publish_attestations(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_none(origin.clone()).or_else(|_| ensure_root(origin.clone()))?;
            let mut attestations_ready_to_be_published = Self::attestations_ready_to_be_published();
            if ensure_none(origin.clone()).is_ok() && attestations_ready_to_be_published.is_empty()
            {
                log::trace!("Not publishing tree");
                return Err(Error::<T>::TooEarlyForASmallTree.into());
            }

            let id = Self::next_attestation();

            // In case a publish_attestations() is called by root but not proofs has been inserted yet
            // the corresponding entry is not created in the Values storage variable. To avoid this
            // inconsistency, we manually do this.
            if ensure_root(origin.clone()).is_ok()
                && AttestationsWithProofsToBePublished::<T>::get(id).is_empty()
            {
                log::debug!("Creating empty attestation with id: {}", id);
                let bounded_vec = BoundedVec::truncate_from(vec![H256::default()]);
                AttestationsWithProofsToBePublished::<T>::insert(id, bounded_vec);

                attestations_ready_to_be_published.push(id);
            }

            let mut published_ids = BoundedVec::<u64, T::MaxAttestationsToClear>::default();
            for &id in &attestations_ready_to_be_published {
                let proofs = AttestationsWithProofsToBePublished::<T>::get(id);
                if !proofs.is_empty() {
                    let sorted_proofs: BTreeSet<_> = proofs.iter().cloned().collect();
                    let merkle_root =
                        binary_merkle_tree::merkle_root::<Keccak256, _>(sorted_proofs);
                    Self::deposit_event(Event::NewAttestation {
                        id,
                        attestation: merkle_root,
                    });
                    published_ids
                        .try_push(id)
                        .map_err(|_| Error::<T>::AttestationsToBeClearedMaxReached)?;
                }
            }

            // Check if the last attestation is being published
            if let Some(&last_id) = attestations_ready_to_be_published.iter().max() {
                if last_id == Self::next_attestation() {
                    let new_attestation = Self::next_attestation() + 1;
                    NextAttestation::<T>::set(new_attestation);
                }
            }

            AttestationsToBeCleared::<T>::put(published_ids);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn now() -> T::Moment {
            <timestamp::Pallet<T>>::get()
        }

        /// Insert a new element into the next Merkle tree.
        fn insert(value: H256) {
            let next_attestation = Self::next_attestation();

            AttestationsWithProofsToBePublished::<T>::mutate(next_attestation, |proofs| {
                if proofs.is_empty() {
                    log::info!("Starting new tree with id: {next_attestation}");
                    FirstInsertionTime::<T>::put(Self::now());
                }

                // Check if the proof already exists, if it does don't do anything
                if proofs.contains(&value) {
                    return;
                }

                match proofs.try_push(value) {
                    Ok(_) => {} // Successfully inserted proof into current attestation
                    Err(_) => {
                        // if we've reached the maximum number of proofs per attestation, create new attestation
                        log::info!("Starting new tree with id: {next_attestation}");
                        FirstInsertionTime::<T>::put(Self::now());

                        let new_attestation = next_attestation + 1;
                        NextAttestation::<T>::set(new_attestation);

                        // Insert into the new attestation
                        AttestationsWithProofsToBePublished::<T>::insert(
                            new_attestation,
                            BoundedVec::truncate_from(vec![value]),
                        );
                    }
                }
            });

            Self::deposit_event(Event::NewElement {
                value,
                attestation_id: Self::next_attestation(),
            });
        }

        pub fn attestations_ready_to_be_published() -> Vec<u64> {
            let now = Self::now();
            let mut ready_attestations = Vec::new();
            let attestations = AttestationsWithProofsToBePublished::<T>::iter().collect::<Vec<_>>();

            if attestations.len() > 1 {
                // If there's more than one attestation, all except the last are ready
                ready_attestations.extend(
                    attestations
                        .iter()
                        .take(attestations.len() - 1)
                        .map(|(id, _)| *id),
                );
            }

            // Check the last (or only) attestation
            if let Some((id, proofs)) = attestations.last() {
                let proofs_count = proofs.len().saturated_into::<u32>();
                let deadline = Self::last_publish_time()
                    .map(|t| t + T::MaxElapsedTimeMs::get())
                    .map(|d| now >= d);

                if proofs_count >= T::ProofsPerAttestation::get()
                    || (proofs_count > 0 && deadline.unwrap_or_default())
                {
                    ready_attestations.push(*id);
                }
            }

            ready_attestations
        }

        fn ensure_inherent(data: &InherentData) {
            let _inherent_data = data
                .get_data::<InherentType>(&INHERENT_IDENTIFIER)
                .expect("Inherent data not correctly encoded")
                .expect("Inherent data must be provided");
        }

        pub fn get_proof_path_from_pallet(
            attestation_id: u64,
            proof_hash: H256,
        ) -> Result<MerkleProof<H256, H256>, AttestationPathRequestError> {
            // Verify that the requested attestation id is valid, and return an error if we
            // request one which is greater or equal than the next id to be published
            if Self::next_attestation() <= attestation_id {
                return Err(AttestationPathRequestError::AttestationIdNotPublished(
                    attestation_id,
                ));
            }

            // Collect the leaves associated with the attestation_id requested
            let leaves = AttestationsWithProofsToBePublished::<T>::get(attestation_id);
            if leaves.is_empty() {
                return Err(AttestationPathRequestError::AttestationIdNotPublished(
                    attestation_id,
                ));
            }

            // Check if the requested proof_hash belongs to this set, i.e. is within the set of leaves
            if !leaves.contains(&proof_hash) {
                return Err(AttestationPathRequestError::ProofNotFound(
                    attestation_id,
                    proof_hash,
                ));
            }

            // Retrieve the index of the proof_hash in the leaves
            // This should not fail, given the previous checks (proof_hash is present in the map for the
            // submitted attestation id)
            let proof_index = leaves.iter().position(|v| v == &proof_hash).expect(
                "The proof_hash should be present in the leaves, as we have already checked for it",
            );

            // Evaluate the Merkle proof and return a MerkleProof structure to the caller
            Ok(binary_merkle_tree::merkle_proof::<Keccak256, _, _>(
                leaves.into_iter().collect::<BTreeSet<_>>(),
                proof_index,
            ))
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            let mut weight = T::DbWeight::get().reads(1);

            let ids_to_clear = AttestationsToBeCleared::<T>::take();
            weight = weight.saturating_add(T::DbWeight::get().writes(1));

            for id in ids_to_clear.iter() {
                AttestationsWithProofsToBePublished::<T>::remove(id);
                weight = weight.saturating_add(T::DbWeight::get().writes(1));
            }

            weight
        }

        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            migration::migrate_to_new_storage::<T>()
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;
        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            Self::ensure_inherent(data);
            (!Self::attestations_ready_to_be_published().is_empty())
                .then_some(Call::publish_attestations {})
        }

        fn check_inherent(
            call: &Self::Call,
            data: &InherentData,
        ) -> result::Result<(), Self::Error> {
            if !Self::is_inherent(call) {
                return Ok(());
            };

            Self::ensure_inherent(data);
            (!Self::attestations_ready_to_be_published().is_empty())
                .then_some(())
                .ok_or(InherentError::TooEarlyForASmallTree)
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::publish_attestations { .. })
        }
    }
}
