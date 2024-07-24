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

use core::marker::PhantomData;
use std::{os::macos::raw, vec};

use frame_support::weights::Weight;
use hp_verifiers::{Cow, Verifier, VerifyError};

pub const PROOF_SIZE: usize = 2144;
pub const VK_SIZE: usize = 1719;
pub type Proof = [u8; PROOF_SIZE];
pub type Pubs = Vec<[u8; 32]>;
pub type Vk = [u8; VK_SIZE];
// pub use weight::WeightInfo;

mod benchmarking;
mod verifier_should;

#[pallet_verifiers::verifier]
pub struct Ultraplonk;

impl Verifier for Ultraplonk {
    type Proof = Proof;

    type Pubs = Pubs;

    type Vk = Vk;

    fn hash_context_data() -> &'static [u8] {
        b"ultraplonk"
    }

    fn verify_proof(
        raw_vk: &Self::Vk,
        raw_proof: &Self::Proof,
        raw_pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        log::trace!("Verifying (native)");

        // make sure the public inputs are the correct size
        if raw_pubs.len() % 32 != 0 {
            log::debug!("Invalid public inputs size");
            return Err(VerifyError::InvalidInput);
        }

        let vk = ultraplonk_verifier::VerificationKey::try_from(&raw_vk[..])
            .map_err(|e| {
                log::debug!("Cannot parse verification key: {:?}", e);
                VerifyError::InvalidVerificationKey
            })?;
        let proof = raw_proof.to_vec();
        let pubs = raw_pubs.to_vec();

        match ultraplonk_verifier::verify(&vk, &proof, &pubs) {
            Ok(true) => Ok(()),
            Ok(false) => Err(VerifyError::VerifyError),
            Err(e) => {
                log::debug!("Cannot verify proof: {:?}", e);
                Err(VerifyError::VerifyError)
            }
        }
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        let data = pubs
            .iter()
            .flat_map(|s| s.iter().cloned())
            .collect::<Vec<_>>();
        Cow::Owned(data)
    }
}
