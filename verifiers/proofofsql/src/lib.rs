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

use frame_support::{ensure, weights::Weight};
use hp_verifiers::{Cow, Verifier, VerifyError};
use sp_std::{marker::PhantomData, vec::{self, Vec}};

pub use native::PROOFOFSQL_VK_SIZE as VK_SIZE;
pub type Vk = [u8; VK_SIZE];

#[pallet_verifiers::verifier]
pub struct ProofOfSql;

impl Verifier for ProofOfSql {
    type Proof = Vec<u8>;

    type Pubs = Vec<u8>;

    type Vk = Vk;

    fn hash_context_data() -> &'static [u8] {
        b"proofofsql"
    }

    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        log::trace!("Verifying (native)");

        native::proofofsql_verify::verify(vk, proof, pubs).map_err(Into::into)
    }

    fn validate_vk(vk: &Self::Vk) -> Result<(), VerifyError> {
        native::proofofsql_verify::validate_vk(vk).map_err(Into::into)
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        let data = pubs.iter().copied().collect::<Vec<_>>();
        Cow::Owned(data)
    }
}