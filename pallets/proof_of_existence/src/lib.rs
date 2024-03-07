#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weight;

#[frame_support::pallet]
pub mod pallet {
    use super::weight::SubstrateWeight;
    use super::weight::WeightInfo;
    use pallet_timestamp::{self as timestamp};

    use sp_core::H256;
    use sp_std::{collections::btree_set::BTreeSet, result};

    use frame_support::pallet_prelude::{OptionQuery, *};
    use frame_support::sp_runtime::traits::{Keccak256, SaturatedConversion};
    use frame_system::pallet_prelude::*;

    use hp_poe::{InherentError, InherentType, INHERENT_IDENTIFIER};

    #[pallet::config]
    pub trait Config: frame_system::Config + timestamp::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Minimum number of leaves in the tree that triggers the attestation publishing
        type MinProofsForPublishing: Get<u32>;
        /// Maximum time (ms) that an element can wait in a tree before the tree is published
        type MaxElapsedTimeMs: Get<Self::Moment>;
    }

    impl<T: Config> hp_poe::OnProofVerified for Pallet<T> {
        fn on_proof_verified(pubs_hash: H256) {
            Self::insert(pubs_hash);
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_attestation)]
    pub type NextAttestation<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_publish_time)]
    pub type FirstInsertionTime<T: Config> = StorageValue<_, T::Moment, OptionQuery>;

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
    pub enum Error<T> {
        TooEarlyForASmallTree,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Publish the attestation of Merkle tree and move to the next tree.
        #[pallet::call_index(0)]
        #[pallet::weight(SubstrateWeight::<T>::publish_attestation())]
        pub fn publish_attestation(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_none(origin.clone()).or_else(|_| ensure_root(origin.clone()))?;
            if ensure_none(origin.clone()).is_ok() && !Self::should_publish(Self::now()) {
                log::trace!("Not publishing tree");
                return Err(Error::<T>::TooEarlyForASmallTree.into());
            }

            let id = Self::next_attestation();
            NextAttestation::<T>::set(id + 1);

            let attestation = binary_merkle_tree::merkle_root::<Keccak256, _>(
                Values::<T>::iter_key_prefix(id).collect::<BTreeSet<_>>(),
            );

            Self::deposit_event(Event::NewAttestation { id, attestation });

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

            // Start counting for timeout when the first item of the new tree is inserted
            if Values::<T>::iter_key_prefix(next_attestation)
                .next()
                .is_none()
            {
                log::info!("Starting new tree with id: {next_attestation}");
                FirstInsertionTime::<T>::put(Self::now());
            }

            log::trace!("Inserting element: {value}");
            Values::<T>::insert(next_attestation, &value, ());

            Self::deposit_event(Event::NewElement {
                value,
                attestation_id: next_attestation,
            });
        }

        fn should_publish(now: T::Moment) -> bool {
            let id = Self::next_attestation();
            let values = Values::<T>::iter_key_prefix(id)
                .count()
                .saturated_into::<u32>();
            let deadline = Self::last_publish_time()
                .map(|t| t + T::MaxElapsedTimeMs::get())
                .map(|d| now >= d);
            values >= T::MinProofsForPublishing::get()
                || (values > 0 && deadline.unwrap_or_default())
        }

        fn ensure_inherent(data: &InherentData) {
            let _inherent_data = data
                .get_data::<InherentType>(&INHERENT_IDENTIFIER)
                .expect("Inherent data not correctly encoded")
                .expect("Inherent data must be provided");
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;
        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            Self::ensure_inherent(data);
            Self::should_publish(Self::now()).then_some(Call::publish_attestation {})
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
                .then_some(())
                .ok_or(InherentError::TooEarlyForASmallTree)
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::publish_attestation { .. })
        }
    }
}
