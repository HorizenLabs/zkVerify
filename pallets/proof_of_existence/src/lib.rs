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

const COMMON_ATTESTATION_CHAIN_ID: u32 = 0u32;

#[frame_support::pallet]
pub mod pallet {

    use super::WeightInfo;
    use super::COMMON_ATTESTATION_CHAIN_ID;
    use binary_merkle_tree::MerkleProof;
    use pallet_timestamp::{self as timestamp};

    use sp_core::H256;
    use sp_std::{collections::btree_set::BTreeSet, result};

    use frame_support::pallet_prelude::{OptionQuery, *};
    use frame_support::sp_runtime::traits::{Keccak256, SaturatedConversion, Zero};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    pub use hp_poe::MaxStorageAttestations;
    use hp_poe::{InherentError, InherentType, INHERENT_IDENTIFIER};

    use crate::migration;

    #[derive(Clone, TypeInfo, PartialEq, Eq, Encode, Decode, Debug)]
    pub enum AttestationPathRequestError {
        ProofNotFound(u64, sp_core::H256, u32),
        AttestationIdNotPublished(u64, u32),
        UnregisteredAttestationChain(u32),
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + timestamp::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Minimum number of leaves in the tree that triggers the attestation publishing
        type MinProofsForPublishing: Get<u32>;
        /// Maximum time (ms) that an element can wait in a tree before the tree is published
        type MaxElapsedTimeMs: Get<Self::Moment>;
        /// Maximum number of attestations that are kept in storage (including the current one)
        type MaxStorageAttestations: Get<MaxStorageAttestations>;
        /// The weight definition for this pallet
        type WeightInfo: WeightInfo;
    }

    impl<T: Config> hp_poe::OnProofVerified for Pallet<T> {
        fn on_proof_verified(pubs_hash: H256, attestation_chain_id: Option<u32>) {
            let attestation_chain_id = attestation_chain_id.unwrap_or(COMMON_ATTESTATION_CHAIN_ID);
            Self::insert(pubs_hash, attestation_chain_id);
        }
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn oldest_attestation)]
    pub type OldestAttestation<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn oldest_attestation_v2)]
    pub type OldestAttestationV2<T> = StorageMap<Hasher = Identity, Key = u32, Value = u64>;

    #[pallet::storage]
    #[pallet::getter(fn next_attestation)]
    pub type NextAttestation<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_attestation_v2)]
    pub type NextAttestationV2<T> = StorageMap<Hasher = Identity, Key = u32, Value = u64>;

    #[pallet::storage]
    #[pallet::getter(fn last_publish_time)]
    pub type FirstInsertionTime<T: Config> = StorageValue<_, T::Moment, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn first_insertion_time_v2)]
    pub type FirstInsertionTimeV2<T: Config> =
        StorageMap<Hasher = Identity, Key = u32, Value = T::Moment>;

    #[pallet::storage]
    #[pallet::getter(fn values)]
    pub type Values<T> = StorageDoubleMap<
        Hasher1 = Blake2_128Concat,
        Key1 = u64,
        Hasher2 = Blake2_128Concat,
        Key2 = H256,
        Value = (),
    >;

    #[pallet::storage]
    #[pallet::getter(fn values_v2)]
    pub type ValuesV2<T> = StorageDoubleMap<
        Hasher1 = Blake2_128Concat,
        Key1 = (u64, u32), // (att_id, attestation_chain_id)
        Hasher2 = Blake2_128Concat,
        Key2 = H256, // (proof hash)
        Value = (),
    >;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub _phantom: PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            OldestAttestationV2::<T>::insert(COMMON_ATTESTATION_CHAIN_ID, 1);
            NextAttestationV2::<T>::insert(COMMON_ATTESTATION_CHAIN_ID, 1);
            FirstInsertionTimeV2::<T>::insert(COMMON_ATTESTATION_CHAIN_ID, T::Moment::zero());
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewElement {
            value: H256,
            attestation_id: u64,
            attestation_chain_id: u32,
        },
        NewAttestation {
            id: u64,
            attestation: H256,
            attestation_chain_id: u32,
        },
        NewElementForUnregisteredAttestationChain {
            value: H256,
            attestation_chain_id: u32,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        TooEarlyForASmallTree,
        AlreadyRegisteredChain,
    }

    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        /// Publish the attestation of Merkle tree and move to the next tree.
        #[pallet::call_index(0)]
        pub fn publish_attestation(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_none(origin.clone()).or_else(|_| ensure_root(origin.clone()))?;
            let mut attestation_chain_id_to_publish = Self::should_publish(Self::now());
            if ensure_none(origin.clone()).is_ok() && attestation_chain_id_to_publish.is_none() {
                log::trace!("Not publishing tree");
                return Err(Error::<T>::TooEarlyForASmallTree.into());
            } else if attestation_chain_id_to_publish.is_none() {
                attestation_chain_id_to_publish = Some(COMMON_ATTESTATION_CHAIN_ID);
            }

            let attestation_chain_id = attestation_chain_id_to_publish.unwrap();
            log::trace!("Publishing for attestation chain id: {attestation_chain_id}");

            let id = Self::next_attestation_v2(attestation_chain_id).unwrap();

            // In case a publish_attestation() is called by root but not proofs has been inserted yet
            // the corresponding entry is not created in the Values storage variable. To avoid this
            // inconsistency, we manually do this.
            if ensure_root(origin.clone()).is_ok()
                && ValuesV2::<T>::iter_key_prefix((id, attestation_chain_id))
                    .next()
                    .is_none()
            {
                log::debug!("Creating empty attestation with id: {}", id);
                ValuesV2::<T>::insert((id, attestation_chain_id), H256::default(), ());
            }

            let next_attestation_id = id + 1;
            NextAttestationV2::<T>::insert(attestation_chain_id, next_attestation_id);

            let attestation = binary_merkle_tree::merkle_root::<Keccak256, _>(
                ValuesV2::<T>::iter_key_prefix((id, attestation_chain_id)).collect::<BTreeSet<_>>(),
            );

            Self::deposit_event(Event::NewAttestation {
                id,
                attestation,
                attestation_chain_id,
            });

            // Prune old attestations
            // Rationale: ids are incremental, no more than one attestation
            // for each publish_attestation call will need to be removed from storage.
            let max_attestations: u64 = T::MaxStorageAttestations::get().into();
            if max_attestations != Into::<u64>::into(MaxStorageAttestations::default())
                && next_attestation_id >= max_attestations
            {
                let oldest_attestation_id =
                    Self::oldest_attestation_v2(attestation_chain_id).unwrap();

                // Handle situations in which there is more than one attestation to be cleared
                // from storage. This could only happen, for instance, due to a runtime upgrade
                // changing the MaxAttestations value.
                // NOTE: When doing this, make sure that the attestations left to be cleared are
                // not too many otherwise the transaction weight might explode and the transaction
                // never executed, thus blocking the chain.
                // This behaviour will be changed in the future.
                let limit = next_attestation_id - max_attestations + 1;
                for id in oldest_attestation_id..limit {
                    let _ = ValuesV2::<T>::clear_prefix((id, attestation_chain_id), u32::MAX, None);
                }
                OldestAttestationV2::<T>::insert(attestation_chain_id, limit);
            }

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn register_attestation_chain(
            _origin: OriginFor<T>,
            attestation_chain_id: u32,
        ) -> DispatchResultWithPostInfo {
            log::trace!("Registering attestation chain id {attestation_chain_id}");
            if Self::is_attestation_chain_registered(attestation_chain_id) {
                log::trace!("Already registered attestation chain");
                return Err(Error::<T>::AlreadyRegisteredChain.into());
            } else {
                OldestAttestationV2::<T>::insert(attestation_chain_id, 0);
                NextAttestationV2::<T>::insert(attestation_chain_id, 0);
                FirstInsertionTimeV2::<T>::insert(attestation_chain_id, T::Moment::zero());
            }
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn now() -> T::Moment {
            <timestamp::Pallet<T>>::get()
        }

        fn is_attestation_chain_registered(attestation_chain_id: u32) -> bool {
            Self::oldest_attestation_v2(attestation_chain_id).is_some()
                && Self::next_attestation_v2(attestation_chain_id).is_some()
                && Self::first_insertion_time_v2(attestation_chain_id).is_some()
        }

        /// Insert a new element into the next Merkle tree.
        fn insert(value: H256, attestation_chain_id: u32) {
            log::trace!("Inserting for attestation chain id: {attestation_chain_id}");

            if !Self::is_attestation_chain_registered(attestation_chain_id) {
                log::trace!("Unregistered attestation chain id");
                Self::deposit_event(Event::NewElementForUnregisteredAttestationChain {
                    value,
                    attestation_chain_id,
                });
            } else {
                let next_attestation = Self::next_attestation_v2(attestation_chain_id).unwrap();

                // Start counting for timeout when the first item of the new tree is inserted
                if ValuesV2::<T>::iter_key_prefix((next_attestation, attestation_chain_id))
                    .next()
                    .is_none()
                {
                    log::info!("Starting new tree with attestation id: {next_attestation}");
                    FirstInsertionTimeV2::<T>::insert(attestation_chain_id, Self::now());
                }

                log::trace!("Inserting element: {value}");
                ValuesV2::<T>::insert((next_attestation, attestation_chain_id), value, ());

                Self::deposit_event(Event::NewElement {
                    value,
                    attestation_id: next_attestation,
                    attestation_chain_id: attestation_chain_id,
                });
            }
        }

        fn should_publish(now: T::Moment) -> Option<u32> {
            for (attestation_chain_id, _value) in OldestAttestationV2::<T>::iter() {
                let id = Self::next_attestation_v2(attestation_chain_id).unwrap();
                let values_count = ValuesV2::<T>::iter_key_prefix((id, attestation_chain_id))
                    .count()
                    .saturated_into::<u32>();
                let deadline = Self::first_insertion_time_v2(attestation_chain_id)
                    .map(|t| t + T::MaxElapsedTimeMs::get())
                    .map(|d| now >= d);
                if values_count >= T::MinProofsForPublishing::get()
                    || (values_count > 0 && deadline.unwrap_or_default())
                {
                    return Some(attestation_chain_id);
                }
            }
            return None;
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
            attestation_chain_id: Option<u32>,
        ) -> Result<MerkleProof<H256, H256>, AttestationPathRequestError> {
            let attestation_chain_id = attestation_chain_id.unwrap_or(COMMON_ATTESTATION_CHAIN_ID);

            if !Self::is_attestation_chain_registered(attestation_chain_id) {
                return Err(AttestationPathRequestError::UnregisteredAttestationChain(
                    attestation_chain_id,
                ));
            } else {
                // Verify that the requested attestation id is valid, and return an error if we
                // request one which is greater or equal than the next id to be published
                if Self::next_attestation_v2(attestation_chain_id).unwrap() <= attestation_id {
                    return Err(AttestationPathRequestError::AttestationIdNotPublished(
                        attestation_id,
                        attestation_chain_id,
                    ));
                }

                // Collect the leaves associated with the attestation_id requested
                let leaves = ValuesV2::<T>::iter_key_prefix((attestation_id, attestation_chain_id))
                    .collect::<BTreeSet<_>>();

                // Check if the requested proof_hash belongs to this set, i.e. is within the set of leaves
                if !leaves.contains(&proof_hash) {
                    return Err(AttestationPathRequestError::ProofNotFound(
                        attestation_id,
                        proof_hash,
                        attestation_chain_id,
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
                    leaves,
                    proof_index,
                ))
            }
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;
        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            Self::ensure_inherent(data);
            Self::should_publish(Self::now()).and_then(|_| Some(Call::publish_attestation {}))
        }

        fn check_inherent(
            call: &Self::Call,
            data: &InherentData,
        ) -> result::Result<(), Self::Error> {
            if !Self::is_inherent(call) {
                return Ok(());
            };

            Self::ensure_inherent(data);
            Self::should_publish(Self::now())
                .and_then(|_| Some(()))
                .ok_or(InherentError::TooEarlyForASmallTree)
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::publish_attestation { .. })
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            migration::migrate_to_v2::<T>()
        }
    }
}
