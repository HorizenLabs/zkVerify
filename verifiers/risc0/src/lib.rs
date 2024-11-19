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

    type Vk = Vk;

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
        panic!("Risc0 vk is already hashed and we cannot know its preimage: use vk_hash() instead")
    }
}

/// The struct to use in runtime pallet configuration to map the weight computed by this crate
/// benchmarks to the weight needed by the `pallet-verifiers`.
pub struct Risc0Weight<W: weight::WeightInfo>(PhantomData<W>);

pub static CYCLE_2_POW_FROM_12_TO_13: usize = 215538;
pub static CYCLE_2_POW_FROM_14_TO_17: usize = 238578;
pub static CYCLE_2_POW_FROM_18_TO_18: usize = 250290;
pub static CYCLE_2_POW_FROM_19_TO_19: usize = 262514;
pub static CYCLE_2_POW_FROM_20_TO_20: usize = 275762;
pub static CYCLE_2_POW_FROM_21_TO_21: usize = 514256;
pub static CYCLE_2_POW_FROM_22_TO_22: usize = 789934;
pub static CYCLE_2_POW_FROM_23_TO_23: usize = 1353002;
pub static CYCLE_2_POW_FROM_24_TO_24: usize = 2455714;

impl<T: Config, W: weight::WeightInfo> pallet_verifiers::WeightInfo<Risc0<T>> for Risc0Weight<W> {
    fn submit_proof(
        proof: &<Risc0<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Risc0<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        let len = proof.len();
        if len <= CYCLE_2_POW_FROM_12_TO_13 {
            W::submit_proof_cycle_2_pow_13()
        } else if len <= CYCLE_2_POW_FROM_14_TO_17 {
            W::submit_proof_cycle_2_pow_17()
        } else if len <= CYCLE_2_POW_FROM_18_TO_18 {
            W::submit_proof_cycle_2_pow_18()
        } else if len <= CYCLE_2_POW_FROM_19_TO_19 {
            W::submit_proof_cycle_2_pow_19()
        } else if len <= CYCLE_2_POW_FROM_20_TO_20 {
            W::submit_proof_cycle_2_pow_20()
        } else if len <= CYCLE_2_POW_FROM_21_TO_21 {
            W::submit_proof_cycle_2_pow_21()
        } else if len <= CYCLE_2_POW_FROM_22_TO_22 {
            W::submit_proof_cycle_2_pow_22()
        } else if len <= CYCLE_2_POW_FROM_23_TO_23 {
            W::submit_proof_cycle_2_pow_23()
        } else {
            W::submit_proof_cycle_2_pow_24()
        }
    }

    fn submit_proof_with_vk_hash(
        proof: &<Risc0<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<Risc0<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        let len = proof.len();
        if len <= CYCLE_2_POW_FROM_12_TO_13 {
            W::submit_proof_with_vk_hash_cycle_2_pow_13()
        } else if len <= CYCLE_2_POW_FROM_14_TO_17 {
            W::submit_proof_with_vk_hash_cycle_2_pow_17()
        } else if len <= CYCLE_2_POW_FROM_18_TO_18 {
            W::submit_proof_with_vk_hash_cycle_2_pow_18()
        } else if len <= CYCLE_2_POW_FROM_19_TO_19 {
            W::submit_proof_with_vk_hash_cycle_2_pow_19()
        } else if len <= CYCLE_2_POW_FROM_20_TO_20 {
            W::submit_proof_with_vk_hash_cycle_2_pow_20()
        } else if len <= CYCLE_2_POW_FROM_21_TO_21 {
            W::submit_proof_with_vk_hash_cycle_2_pow_21()
        } else if len <= CYCLE_2_POW_FROM_22_TO_22 {
            W::submit_proof_with_vk_hash_cycle_2_pow_22()
        } else if len <= CYCLE_2_POW_FROM_23_TO_23 {
            W::submit_proof_with_vk_hash_cycle_2_pow_23()
        } else {
            W::submit_proof_with_vk_hash_cycle_2_pow_24()
        }
    }

    fn register_vk(_vk: &<Risc0<T> as hp_verifiers::Verifier>::Vk) -> Weight {
        W::register_vk()
    }

    fn unregister_vk() -> frame_support::weights::Weight {
        W::unregister_vk()
    }
}
