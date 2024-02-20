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

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    type Proof = [u8; 25 * 32];
    pub fn verify_proof<T>(raw_proof: Proof) -> Result<(), Error<T>> {
        log::trace!("verifying proof");
        let pubs: fflonk_verifier::Public = (&raw_proof[32 * 24..])
            .try_into()
            .map_err(|e| log::error!("Cannot get public input: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;
        log::trace!("Extracted public input");
        let raw_proof = <[u8; 32 * 24]>::try_from(&raw_proof[..32 * 24])
            .map_err(|e| log::error!("Cannot get raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;
        log::trace!("Extracted proof data");
        let proof = fflonk_verifier::Proof::try_from(&raw_proof)
            .map_err(|e| log::debug!("Invalid raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
        log::trace!("Extracted proof data");
        proof
            .verify(pubs)
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| Error::<T>::VerifyError)
    }

    // The pallet's runtime storage items.
    // https://docs.substrate.io/v3/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// Invalid bytes input.
        InvalidInput,
        /// Provided data isn't a valid Proof.
        InvalidProofData,
        /// Verify proof failed.
        VerifyError,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/v3/runtime/origins
            let who = ensure_signed(origin)?;

            // Update storage.
            <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::SomethingStored(something, who));
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <Something<T>>::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <Something<T>>::put(new);
                    Ok(().into())
                }
            }
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn submit_proof(_origin: OriginFor<T>, raw_proof: Proof) -> DispatchResultWithPostInfo {
            log::trace!("Enter");
            verify_proof::<T>(raw_proof)
                .map(Into::into)
                .map_err(Into::into)
        }
    }
}
