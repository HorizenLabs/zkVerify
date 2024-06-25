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

// FAKE

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub struct NHWeight<T>(PhantomData<T>);

// For backwards compatibility and tests.
impl<T: frame_system::Config>  pallet_zksync_verifier::WeightInfo for NHWeight<T> {
    fn submit_proof() -> Weight {
		Weight::from_parts(35_000_000_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    fn submit_proof_with_vk_hash() -> Weight {
		Weight::from_parts(37_000_000_000, 4010)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    fn register_vk() -> Weight {
		Weight::from_parts(4_000_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
    }
}