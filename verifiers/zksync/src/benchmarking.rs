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

#![cfg(feature = "runtime-benchmarks")]

use crate::Zksync;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use pallet_verifiers::{VkOrHash, Vks};

pub struct Pallet<T: Config>(pallet_verifiers::Pallet<T, Zksync>);
pub trait Config: pallet_verifiers::Config<Zksync> {}
impl<T: pallet_verifiers::Config<Zksync>> Config for T {}
pub type Call<T> = pallet_verifiers::Call<T, Zksync>;

include!("resources.rs");

#[benchmarks]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn submit_proof() {
        // setup code
        let caller = whitelisted_caller();

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_vk(()),
            PROOF.into(),
            PUBS.into(),
        );
    }

    #[benchmark]
    fn submit_proof_with_vk_hash() {
        // setup code
        let caller = whitelisted_caller();
        let hash = sp_core::H256::repeat_byte(2);
        Vks::<T, Zksync>::insert(hash, ());

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_hash(hash),
            PROOF.into(),
            PUBS.into(),
        );
    }

    #[benchmark]
    fn register_vk() {
        // setup code
        let caller = whitelisted_caller();

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), ().into());

        // Verify
        assert!(Vks::<T, Zksync>::get(pallet_verifiers::hash_key::<Zksync>(&())).is_some());
    }
}
