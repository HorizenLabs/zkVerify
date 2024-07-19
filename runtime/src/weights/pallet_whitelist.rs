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

// TODO: Autogenerate with remote machine and add details + executed command

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_whitelist`.
pub struct NHWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_whitelist::WeightInfo for NHWeight<T> {
    /// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
    /// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn whitelist_call() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `118`
        //  Estimated: `3556`
        // Minimum execution time: 15_543_000 picoseconds.
        Weight::from_parts(16_305_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
    /// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn remove_whitelisted_call() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `247`
        //  Estimated: `3556`
        // Minimum execution time: 15_172_000 picoseconds.
        Weight::from_parts(15_806_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
    /// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::PreimageFor` (r:1 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `Measured`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 4194294]`.
    fn dispatch_whitelisted_call(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `323 + n * (1 ±0)`
        //  Estimated: `3787 + n * (1 ±0)`
        // Minimum execution time: 25_100_000 picoseconds.
        Weight::from_parts(25_253_000, 0)
            .saturating_add(Weight::from_parts(0, 3787))
            // Standard Error: 1
            .saturating_add(Weight::from_parts(1_212, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: `Whitelist::WhitelistedCall` (r:1 w:1)
    /// Proof: `Whitelist::WhitelistedCall` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 10000]`.
    fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `247`
        //  Estimated: `3556`
        // Minimum execution time: 18_677_000 picoseconds.
        Weight::from_parts(19_237_053, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            // Standard Error: 3
            .saturating_add(Weight::from_parts(1_420, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}