#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;

#[cfg(test)]
pub use mock::FakeVerifier;

pub use fflonk_verifier::Fflonk;

pub use hp_verifiers::WeightInfo;

#[frame_support::pallet(dev_mode)]
pub mod pallet {

    use codec::Encode;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, Identity};
    use frame_system::pallet_prelude::*;
    use hp_poe::OnProofVerified;
    use sp_core::H256;
    use sp_io::hashing::keccak_256;
    use sp_std::boxed::Box;

    use hp_verifiers::{Verifier, VerifyError, WeightInfo};

    #[pallet::pallet]
    pub struct Pallet<T, I = ()>(_);

    #[derive(Debug, Clone, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen)]
    pub enum VkOrHash<K>
    where
        K: sp_std::fmt::Debug + Clone + PartialEq + Encode + Decode + TypeInfo + MaxEncodedLen,
    {
        Hash(H256),
        Vk(Box<K>),
    }

    impl<K> VkOrHash<K>
    where
        K: sp_std::fmt::Debug + Clone + PartialEq + Encode + Decode + TypeInfo + MaxEncodedLen,
    {
        pub fn from_vk(vk: K) -> Self {
            VkOrHash::Vk(Box::new(vk))
        }

        pub fn from_hash(hash: H256) -> Self {
            VkOrHash::Hash(hash)
        }
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config
    where
        I: Verifier,
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self, I>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Proof verified call back
        type OnProofVerified: OnProofVerified;
        /// Weights
        type WeightInfo: hp_verifiers::WeightInfo<I>;
    }

    fn statement_hash(ctx: &[u8], vk_hash: &H256, pubs: &[u8]) -> H256 {
        let mut data_to_hash = ctx.to_vec();
        data_to_hash.extend_from_slice(b"-");
        data_to_hash.extend_from_slice(vk_hash.as_bytes());
        data_to_hash.extend_from_slice(b"-");
        data_to_hash.extend_from_slice(pubs);
        H256(keccak_256(data_to_hash.as_slice()))
    }

    fn compute_hash<I: Verifier>(pubs: &I::Pubs, vk_or_hash: &VkOrHash<I::Vk>) -> H256 {
        let hash = match vk_or_hash {
            VkOrHash::Hash(h) => sp_std::borrow::Cow::Borrowed(h),
            VkOrHash::Vk(vk) => sp_std::borrow::Cow::Owned(hash_key::<I>(vk)),
        };
        statement_hash(
            I::hash_context_data(),
            hash.as_ref(),
            I::pubs_bytes(pubs).as_ref(),
        )
    }

    /// Pallet specific events.
    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()>
    where
        I: Verifier,
    {
        VkRegistered { hash: H256 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T, I = ()> {
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

    impl<T, I> From<VerifyError> for Error<T, I> {
        fn from(e: VerifyError) -> Self {
            match e {
                VerifyError::InvalidInput => Error::<T, I>::InvalidInput,
                VerifyError::InvalidProofData => Error::<T, I>::InvalidProofData,
                VerifyError::VerifyError => Error::<T, I>::VerifyError,
                VerifyError::InvalidVerificationKey => Error::<T, I>::InvalidVerificationKey,
            }
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn vks)]
    type Vks<T: Config<I>, I: 'static = ()>
    where
        I: Verifier,
    = StorageMap<Hasher = Identity, Key = H256, Value = I::Vk>;

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I>
    where
        I: Verifier,
    {
        #[pallet::call_index(0)]
        #[pallet::weight(match &vk_or_hash {
                VkOrHash::Vk(_) => T::WeightInfo::submit_proof(proof, pubs),
                VkOrHash::Hash(_) => T::WeightInfo::submit_proof_with_vk_hash(proof, pubs),
            })]
        pub fn submit_proof(
            _origin: OriginFor<T>,
            vk_or_hash: VkOrHash<I::Vk>,
            proof: Box<I::Proof>,
            pubs: Box<I::Pubs>,
        ) -> DispatchResultWithPostInfo {
            log::trace!("Submitting proof");
            let vk = match &vk_or_hash {
                VkOrHash::Hash(h) => {
                    Vks::<T, I>::get(h).ok_or(Error::<T, I>::VerificationKeyNotFound)?
                }
                VkOrHash::Vk(vk) => {
                    I::validate_vk(vk).map_err(Error::<T, I>::from)?;
                    vk.as_ref().clone()
                }
            };
            I::verify_proof(&vk, &proof, &pubs)
                .map(|_x| {
                    T::OnProofVerified::on_proof_verified(compute_hash::<I>(&pubs, &vk_or_hash))
                })
                .map_err(Error::<T, I>::from)?;
            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::register_vk(vk))]
        pub fn register_vk(_origin: OriginFor<T>, vk: Box<I::Vk>) -> DispatchResultWithPostInfo {
            log::trace!("Register vk");
            I::validate_vk(&vk).map_err(Error::<T, I>::from)?;
            let hash = hash_key::<I>(&vk);
            Vks::<T, I>::insert(hash, vk);
            Self::deposit_event(Event::VkRegistered { hash });
            Ok(().into())
        }
    }

    fn hash_key<I: Verifier>(vk: &I::Vk) -> H256 {
        H256(keccak_256(&I::vk_bytes(vk)))
    }

    #[cfg(test)]
    mod tests {
        use core::marker::PhantomData;

        use crate::{
            tests::submit_proof_should::{
                REGISTERED_VK, REGISTERED_VK_HASH, VALID_HASH_REGISTERED_VK,
            },
            FakeVerifier,
        };

        use super::*;
        use hp_verifiers::Verifier;
        use rstest::rstest;
        use sp_core::U256;

        struct OtherVerifier;
        impl Verifier for OtherVerifier {
            type Proof = u64;
            type Pubs = u64;
            type Vk = u64;
            fn hash_context_data() -> &'static [u8] {
                let context = b"other";
                assert_ne!(FakeVerifier::hash_context_data(), context);
                context
            }
            fn validate_vk(vk: &Self::Vk) -> Result<(), VerifyError> {
                FakeVerifier::validate_vk(vk)
            }
            fn verify_proof(
                vk: &Self::Vk,
                proof: &Self::Proof,
                pubs: &Self::Pubs,
            ) -> Result<(), VerifyError> {
                FakeVerifier::verify_proof(vk, proof, pubs)
            }
            fn pubs_bytes(pubs: &Self::Pubs) -> sp_std::borrow::Cow<[u8]> {
                FakeVerifier::pubs_bytes(pubs)
            }
        }

        #[rstest]
        #[case::vk_and_pubs_used_in_test(
            PhantomData::<FakeVerifier>,
            42,
            VkOrHash::from_vk(REGISTERED_VK),
            VALID_HASH_REGISTERED_VK
        )]
        #[case::same_from_vk_hash(
            PhantomData::<FakeVerifier>,
            42,
            VkOrHash::from_hash(REGISTERED_VK_HASH),
            VALID_HASH_REGISTERED_VK
        )]
        #[case::hash_as_documented(
            PhantomData::<FakeVerifier>,
            42,
            VkOrHash::from_vk(REGISTERED_VK),
            {
                let mut data_to_hash = b"fake".to_vec();
                data_to_hash.extend_from_slice(b"-");
                data_to_hash.extend_from_slice(REGISTERED_VK_HASH.as_bytes());
                data_to_hash.extend_from_slice(b"-");
                data_to_hash.extend_from_slice(42_u64.to_be_bytes().as_ref());
                H256(keccak_256(data_to_hash.as_slice()))
            }
        )]
        #[should_panic]
        #[case::should_take_care_of_pubs(
            PhantomData::<FakeVerifier>,
            24,
            VkOrHash::from_vk(REGISTERED_VK),
            VALID_HASH_REGISTERED_VK
        )]
        #[should_panic]
        #[case::should_take_care_of_context_data(
            PhantomData::<OtherVerifier>,
            42,
            VkOrHash::from_vk(REGISTERED_VK),
            VALID_HASH_REGISTERED_VK
        )]
        #[should_panic]
        #[case::should_take_care_of_vk(
            PhantomData::<FakeVerifier>,
            42,
            VkOrHash::from_vk(24),
            VALID_HASH_REGISTERED_VK
        )]
        fn hash_statement_as_expected<V: Verifier>(
            #[case] _verifier: PhantomData<V>,
            #[case] pubs: V::Pubs,
            #[case] vk_or_hash: VkOrHash<V::Vk>,
            #[case] expected: H256,
        ) {
            let hash = compute_hash::<V>(&pubs, &vk_or_hash);

            assert_eq!(hash, expected);
        }

        struct Other2Verifier;
        impl Verifier for Other2Verifier {
            type Proof = ();
            type Pubs = ();
            type Vk = U256;
            fn hash_context_data() -> &'static [u8] {
                b"more"
            }

            fn verify_proof(
                _vk: &Self::Vk,
                _proof: &Self::Proof,
                _pubs: &Self::Pubs,
            ) -> Result<(), VerifyError> {
                Ok(())
            }

            fn pubs_bytes(_pubs: &Self::Pubs) -> hp_verifiers::Cow<[u8]> {
                hp_verifiers::Cow::Borrowed(&[])
            }
        }

        #[rstest]
        #[case::vk_used_in_test(PhantomData::<FakeVerifier>, REGISTERED_VK, REGISTERED_VK_HASH)]
        #[should_panic]
        #[case::u256_vk_changed(PhantomData::<Other2Verifier>, U256::from(REGISTERED_VK), REGISTERED_VK_HASH)]
        fn hash_vk_as_expected<V: Verifier>(
            #[case] _verifier: PhantomData<V>,
            #[case] vk: V::Vk,
            #[case] expected: H256,
        ) {
            let hash = hash_key::<V>(&vk);

            assert_eq!(hash, expected);
        }
    }
}
