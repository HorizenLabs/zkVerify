#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

/*
0x26538ed002cd6f8fa37291b8158e23adaab5c203fd9bb7138416e29d21e28eba
0x3343c29f5f2b57001479ecae98bf6f61f74e7681766712f862b771f05959e8a2
0x79b94f64c1031f5ff83da77db70a9d7fcaa264ff107f338e6b0a529feb49a7bb
0x4ae66967fee2612ff8253aec07240ac918554b5f6431ca6126339a50d5e3eaab
0x9ec1350c04237b493786fbd7208882b9c03d115833b5a4da396deff8a806f01e
0x8c7f810816420fa0f2703ea6ead4ef1e1770f681cc3e47aba2b32ca03d40e3f3
0x9f49d7f1fd4d6ede86c212edd42e0de46a30d40fa92602a54a6fa0543b6b42a1
0xa2ff8acf11a487ed0016f5cc8372833b425ce69da58903979e83e41cfa5cb2da

*/

#[frame_support::pallet(dev_mode)]
pub mod pallet {
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
        type MinProofsPerTree: Get<u32>;
        type MaxElapsedTimeMs: Get<Self::Moment>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_root)]
    pub type NextRoot<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_publish_time)]
    pub type LastPublishTime<T: Config> = StorageValue<_, T::Moment, OptionQuery>;

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
        NewElement { value: H256, root_id: u64 },
        NewRoot { id: u64, root: H256 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        TooEarlyForASmallTree,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Insert a new element into the next Merkle tree.
        pub fn insert(origin: OriginFor<T>, value: H256) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            let next_root = Self::next_root();
            Values::<T>::insert(next_root, &value, ());

            Self::deposit_event(Event::NewElement {
                value,
                root_id: next_root,
            });

            Ok(().into())
        }

        /// Publish the root of Merkle tree and move to the next tree.
        pub fn publish_root(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_none(origin.clone()).or_else(|_| ensure_root(origin.clone()))?;
            if ensure_none(origin.clone()).is_ok() && !Self::should_publish(Self::now()) {
                return Err(Error::<T>::TooEarlyForASmallTree.into());
            }

            let id = Self::next_root();
            NextRoot::<T>::set(id + 1);

            let root = binary_merkle_tree::merkle_root::<Keccak256, _>(
                Values::<T>::iter_key_prefix(id).collect::<BTreeSet<_>>(),
            );
            LastPublishTime::<T>::put(Self::now());

            Self::deposit_event(Event::NewRoot { id, root });

            Ok(().into())
        }
    }
    impl<T: Config> Pallet<T> {
        fn now() -> T::Moment {
            <timestamp::Pallet<T>>::get()
        }

        fn should_publish(now: T::Moment) -> bool {
            let id = Self::next_root();
            let values = Values::<T>::iter_key_prefix(id)
                .count()
                .saturated_into::<u32>();
            let deadline = Self::last_publish_time()
                .map(|t| t + T::MaxElapsedTimeMs::get())
                .map(|d| now >= d);
            values > 0 && (deadline.unwrap_or_default() || values >= T::MinProofsPerTree::get())
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = InherentError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let _inherent_data = data
                .get_data::<InherentType>(&INHERENT_IDENTIFIER)
                .expect("Inherent data not correctly encoded")
                .expect("Inherent data must be provided");

            if Self::should_publish(Self::now()) {
                Some(Call::publish_root {})
            } else {
                None
            }
        }

        fn check_inherent(
            call: &Self::Call,
            data: &InherentData,
        ) -> result::Result<(), Self::Error> {
            if !Self::is_inherent(call) {
                return Ok(());
            };

            let _inherent_data = data
                .get_data::<InherentType>(&INHERENT_IDENTIFIER)
                .expect("Inherent data not correctly encoded")
                .expect("Inherent data must be provided");

            if !Self::should_publish(Self::now()) {
                Err(InherentError::TooEarlyForASmallTree)
            } else {
                Ok(())
            }
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::publish_root { .. })
        }
    }
}
