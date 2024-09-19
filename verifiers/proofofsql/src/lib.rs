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

use codec::{Decode, Encode, MaxEncodedLen};
use derivative::Derivative;
use frame_support::weights::Weight;
use hp_verifiers::{Cow, Verifier, VerifyError};
use scale_info::TypeInfo;
use sp_core::Get;
use sp_std::{marker::PhantomData, vec::Vec};

pub mod benchmarking;
mod verifier_should;
mod weight;
pub use weight::WeightInfo;

#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(Clone, Debug, PartialEq)]
#[scale_info(skip_type_params(T))]
pub struct Vk<T>(Vec<u8>, PhantomData<T>);

impl<T: Config> Vk<T> {
    pub fn validate_size(&self) -> Result<(), VerifyError> {
        let max_nu = <T as Config>::largest_max_nu();
        let max_vk_size = native::proofofsql_verify::verifier_key_size(max_nu) as usize;
        if self.0.len() > max_vk_size {
            Err(VerifyError::InvalidVerificationKey)
        } else {
            Ok(())
        }
    }
}

impl<T> From<Vec<u8>> for Vk<T> {
    fn from(value: Vec<u8>) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: Config> MaxEncodedLen for Vk<T> {
    fn max_encoded_len() -> usize {
        let max_nu = <T as Config>::largest_max_nu();
        native::proofofsql_verify::verifier_key_size(max_nu) as usize
    }
}

pub trait Config: 'static {
    /// Maximum value allowed for `max_nu` in the verification key
    type LargestMaxNu: Get<u32>;

    fn largest_max_nu() -> u32 {
        Self::LargestMaxNu::get()
    }
}

#[pallet_verifiers::verifier]
pub struct ProofOfSql<T>;

impl<T: Config> Verifier for ProofOfSql<T> {
    type Proof = Vec<u8>;

    type Pubs = Vec<u8>;

    type Vk = Vk<T>;

    fn hash_context_data() -> &'static [u8] {
        b"proofofsql"
    }

    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        log::trace!("Verifying (native)");
        vk.validate_size()?;
        native::proofofsql_verify::verify(vk.0.as_slice(), proof, pubs).map_err(Into::into)
    }

    fn validate_vk(vk: &Self::Vk) -> Result<(), VerifyError> {
        vk.validate_size()?;
        native::proofofsql_verify::validate_vk(vk.0.as_slice()).map_err(Into::into)
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        let data = pubs.to_vec();
        Cow::Owned(data)
    }
}

pub struct ProofOfSqlWeight<W: weight::WeightInfo>(PhantomData<W>);

impl<T: Config, W: weight::WeightInfo> pallet_verifiers::WeightInfo<ProofOfSql<T>>
    for ProofOfSqlWeight<W>
{
    fn submit_proof(
        _proof: &<ProofOfSql<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<ProofOfSql<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof()
    }

    fn submit_proof_with_vk_hash(
        _proof: &<ProofOfSql<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<ProofOfSql<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof_with_vk_hash()
    }

    fn register_vk(_vk: &<ProofOfSql<T> as hp_verifiers::Verifier>::Vk) -> Weight {
        W::register_vk()
    }
}
