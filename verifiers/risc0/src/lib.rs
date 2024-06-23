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

use core::marker::PhantomData;

use frame_support::{ensure, weights::Weight};
use hp_verifiers::Verifier;
use sp_core::{Get, H256};
use sp_std::vec::Vec;

pub mod benchmarking;
mod verifier_should;
mod weight;

pub trait Config: 'static {
    /// Maximum number of bytes contained in the proof (otherwise rejected)
    type MaxProofSize: Get<u32>;
    /// Maximum number of bytes contained in the public inputs (otherwise rejected)
    type MaxPubsSize: Get<u32>;

    fn max_proof_size() -> u32 {
        Self::MaxProofSize::get()
    }

    fn max_pubs_size() -> u32 {
        Self::MaxPubsSize::get()
    }
}

#[pallet_verifiers::verifier]
pub struct Risc0<T>;
pub use weight::WeightInfo;

pub type Proof = Vec<u8>;
pub type Pubs = Vec<u8>;
pub type Vk = H256;

impl<T: Config> Verifier for Risc0<T> {
    type Proof = Proof;

    type Pubs = Pubs;

    type Vk = H256;

    fn hash_context_data() -> &'static [u8] {
        b"risc0"
    }

    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), hp_verifiers::VerifyError> {
        log::trace!("Checking size");
        ensure!(
            proof.len() <= T::MaxProofSize::get() as usize,
            hp_verifiers::VerifyError::InvalidProofData
        );
        ensure!(
            pubs.len() <= T::MaxPubsSize::get() as usize,
            hp_verifiers::VerifyError::InvalidInput
        );
        log::trace!("Verifying (native)");
        native::risc_0_verify::verify((*vk).into(), proof, pubs).map_err(Into::into)
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> hp_verifiers::Cow<[u8]> {
        hp_verifiers::Cow::Borrowed(pubs)
    }

    fn vk_hash(vk: &Self::Vk) -> H256 {
        *vk
    }

    fn vk_bytes(_vk: &Self::Vk) -> hp_verifiers::Cow<[u8]> {
        panic!("We cannot know vk preimage: use vk_hash() instead")
    }
}

/// The struct to use in runtime pallet configuration to map the weight computed by this crate
/// benchmarks to the weight needed by the `pallet-verifiers`.
/// In this case the implementation doesn't depends from the kind of proof or public input and
/// the crate's benchmarks are mapped 1-1 to the `pallet-verifiers`'s one.
pub struct Risc0Weight<W: weight::WeightInfo>(PhantomData<W>);

impl<T: Config, W: weight::WeightInfo> pallet_verifiers::WeightInfo<Risc0<T>> for Risc0Weight<W> {
    fn submit_proof(
        _proof: &<Risc0<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Risc0<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof()
    }

    fn submit_proof_with_vk_hash(
        _proof: &<Risc0<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Risc0<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof_with_vk_hash()
    }

    fn register_vk(_vk: &<Risc0<T> as hp_verifiers::Verifier>::Vk) -> Weight {
        W::register_vk()
    }
}
