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
use sp_std::{marker::PhantomData, vec::Vec};

use native::ULTRAPLONK_PROOF_SIZE as PROOF_SIZE;
use native::ULTRAPLONK_PUBS_SIZE as PUBS_SIZE;
use native::ULTRAPLONK_VK_SIZE as VK_SIZE;
pub type Proof = [u8; PROOF_SIZE];
pub type Pubs = Vec<[u8; PUBS_SIZE]>;
pub type Vk = [u8; VK_SIZE];

pub const MAX_NUM_INPUTS: u32 = 32;

pub trait Config: 'static {
    /// Maximum supported number of public inputs.
    const MAX_NUM_INPUTS: u32;
}

mod benchmarking;
mod verifier_should;

#[pallet_verifiers::verifier]
pub struct Ultraplonk<T>;

impl<T: Config> Verifier for Ultraplonk<T> {
    type Proof = Proof;

    type Pubs = Pubs;

    type Vk = Vk;

    fn hash_context_data() -> &'static [u8] {
        b"ultraplonk"
    }

    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        ensure!(
            pubs.len() <= T::MAX_NUM_INPUTS as usize,
            hp_verifiers::VerifyError::InvalidInput
        );

        log::trace!("Verifying (native)");
        native::ultraplonk_verify::verify((*vk).into(), proof, pubs).map_err(Into::into)
    }

    fn validate_vk(vk: &Self::Vk) -> Result<(), VerifyError> {
        native::ultraplonk_verify::validate_vk(vk).map_err(Into::into)
    }

    fn vk_bytes(vk: &Self::Vk) -> Cow<[u8]> {
        Cow::Borrowed(vk)
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        let data = pubs
            .iter()
            .flat_map(|s| s.iter().cloned())
            .collect::<Vec<_>>();
        Cow::Owned(data)
    }
}

/// The struct to use in runtime pallet configuration to map the weight computed by this crate
/// benchmarks to the weight needed by the `pallet-verifiers`.
pub struct UltraplonkWeight<W: weight::WeightInfo>(PhantomData<W>);

impl<T: Config, W: weight::WeightInfo> pallet_verifiers::WeightInfo<Ultraplonk<T>>
    for UltraplonkWeight<W>
{
    fn submit_proof(
        _proof: &<Ultraplonk<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Ultraplonk<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof()
    }

    fn submit_proof_with_vk_hash(
        _proof: &<Ultraplonk<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Ultraplonk<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof_with_vk_hash()
    }

    fn register_vk(_vk: &<Ultraplonk<T> as hp_verifiers::Verifier>::Vk) -> Weight {
        W::register_vk()
    }
}

mod weight {
    use frame_support::weights::Weight;

    pub trait WeightInfo {
        fn submit_proof() -> Weight;

        fn submit_proof_with_vk_hash() -> Weight;

        fn register_vk() -> Weight;
    }
}
