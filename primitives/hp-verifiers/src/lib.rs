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
#![deny(missing_docs)]

//! The traits and basic implementations for the verifier pallets based on `pallet-verifiers`
//! parametric pallet.

use codec::{Decode, Encode, EncodeLike};
use scale_info::TypeInfo;
use sp_core::{MaxEncodedLen, H256};
pub use sp_std::borrow::Cow;
use sp_std::fmt::Debug;
use sp_weights::Weight;

/// Define the minimum traits that proofs and public inputs should implement.
pub trait Arg: Debug + Clone + PartialEq + Encode + Decode + TypeInfo {}
impl<T: Debug + Clone + PartialEq + Encode + Decode + TypeInfo> Arg for T {}
/// Define the minimum traits that verification keys should implement.
pub trait VkArg: Arg + MaxEncodedLen + EncodeLike {}
impl<T: Arg + MaxEncodedLen + EncodeLike> VkArg for T {}

/// The verification error type
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

/// The trait that characterize a verifier.
pub trait Verifier: 'static {
    /// The proof format type accepted by the verifier
    type Proof: Arg;
    /// The public inputs format
    type Pubs: Arg;
    /// The verification key format
    type Vk: VkArg;

    /// The context used to generate the statements hash.
    fn hash_context_data() -> &'static [u8];

    /// Verify the proof: Should return `Ok(())` if the proof is coherent with the verification
    /// key and the it's valid against the provided public inputs `pubs`.   
    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), VerifyError>;

    /// Validate the verification key: Should return `Ok(())` if the verification key is valid.
    /// The default implementation accept all verification keys: our business logic could
    /// need something different.
    fn validate_vk(_vk: &Self::Vk) -> Result<(), VerifyError> {
        Ok(())
    }

    /// How to compute the verification key hash to use in statement hash computation.
    fn vk_hash(vk: &Self::Vk) -> H256 {
        sp_io::hashing::keccak_256(&Self::vk_bytes(vk)).into()
    }

    /// A vk's byte serialization used to compute the verification key hash. The default implementation
    /// use the `scale::encode()` one, but you can customize it.
    fn vk_bytes(vk: &Self::Vk) -> Cow<[u8]> {
        Cow::Owned(vk.encode())
    }

    /// A public inputs byte serialization used to compute the statement hash. There isn't any
    /// default implementation: you should implement it.
    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]>;
}

/// The trait used to map the `pallet-verifiers` extrinsic in you verifier implementation
/// weights. The methods provide a borrowed proof and public inputs but your code should
/// use them just to guess the _size_ of your verification and map the method in the weights
/// that you computed for your own verifier implementation.
pub trait WeightInfo<V: Verifier> {
    /// Here you should map the given request to a weight computed with your verifier
    /// in the case of the vk is explicit.
    fn submit_proof(proof: &V::Proof, pubs: &V::Pubs) -> Weight;

    /// Here you should map the given request to a weight computed with your verifier
    /// in the case of the vk is provided via a registered vk and its hash.
    fn submit_proof_with_vk_hash(proof: &V::Proof, pubs: &V::Pubs) -> Weight;

    /// Here you should map the given request to a weight computed with your verifier.
    fn register_vk(vk: &V::Vk) -> Weight;

    /// Here you should map the given unregister_vk request to the weight computed with
    /// your verifier.
    fn unregister_vk() -> Weight;
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
