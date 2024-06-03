#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, EncodeLike};
use scale_info::TypeInfo;
use sp_core::MaxEncodedLen;
pub use sp_std::borrow::Cow;
use sp_std::fmt::Debug;
use sp_weights::Weight;

pub trait Arg: Debug + Clone + PartialEq + Encode + Decode + TypeInfo + MaxEncodedLen {}
impl<T: Debug + Clone + PartialEq + Encode + Decode + TypeInfo + MaxEncodedLen> Arg for T {}
pub trait VkArg: Arg + EncodeLike {}
impl<T: Arg + EncodeLike> VkArg for T {}

#[derive(Debug, PartialEq)]
pub enum VerifyError {
    /// Provided data has not valid public inputs.
    InvalidInput,
    /// Provided data has not valid proof.
    InvalidProofData,
    /// Verify proof failed.
    VerifyError,
    /// Provided an invalid verification key.
    InvalidVerificationKey,
}

pub trait Verifier: 'static {
    /// The proof format type accepted by the verifier
    type Proof: Arg;
    /// The public inputs format
    type Pubs: Arg;
    /// The verification key format
    type Vk: VkArg;

    fn hash_context_data() -> &'static [u8];

    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), VerifyError>;

    fn validate_vk(_vk: &Self::Vk) -> Result<(), VerifyError> {
        Ok(())
    }

    fn vk_bytes(vk: &Self::Vk) -> Cow<[u8]> {
        Cow::Owned(vk.encode())
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]>;
}

pub trait WeightInfo<V: Verifier> {
    fn submit_proof(proof: &V::Proof, pubs: &V::Pubs) -> Weight;

    fn submit_proof_with_vk_hash(proof: &V::Proof, pubs: &V::Pubs) -> Weight;

    fn register_vk(vk: &V::Vk) -> Weight;
}

/// `()` is a verifier that reject the proof and returns `VerifyError::VerifyError`.
impl Verifier for () {
    type Proof = ();
    type Pubs = ();
    type Vk = ();

    fn hash_context_data() -> &'static [u8] {
        b"()"
    }

    fn verify_proof(
        _vk: &Self::Vk,
        _proof: &Self::Proof,
        _pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        Err(VerifyError::VerifyError)
    }

    fn validate_vk(_vk: &Self::Vk) -> Result<(), VerifyError> {
        Ok(())
    }

    fn pubs_bytes(_pubs: &Self::Pubs) -> sp_std::borrow::Cow<[u8]> {
        static EMPTY: [u8; 0] = [];
        // Example: If you would use something computed here you can use
        // sp_std::borrow::Cow::Owned(_pubs.encode())
        sp_std::borrow::Cow::Borrowed(&EMPTY)
    }
}

#[cfg(test)]
mod unit_verifier {
    use super::*;

    #[test]
    fn should_raise_error() {
        assert_eq!(
            VerifyError::VerifyError,
            <() as Verifier>::verify_proof(&(), &(), &()).unwrap_err()
        )
    }
}
