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

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_multisig` using the Substrate node and recommended hardware.
pub struct NHWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_multisig::WeightInfo for NHWeight<T> {
    /// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
    /// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `TxPause::PausedCalls` (r:1 w:0)
    /// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
    /// The range of component `z` is `[0, 10000]`.
    fn as_multi_threshold_1(z: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `145`
        //  Estimated: `3997`
        // Minimum execution time: 20_302_000 picoseconds.
        Weight::from_parts(21_362_808, 3997)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(432, 0).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
    }
    /// Storage: `Multisig::Multisigs` (r:1 w:1)
    /// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[2, 100]`.
    /// The range of component `z` is `[0, 10000]`.
    fn as_multi_create(s: u32, z: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `301 + s * (2 ±0)`
        //  Estimated: `6811`
        // Minimum execution time: 41_140_000 picoseconds.
        Weight::from_parts(31_518_927, 6811)
            // Standard Error: 754
            .saturating_add(Weight::from_parts(115_804, 0).saturating_mul(s.into()))
            // Standard Error: 7
            .saturating_add(Weight::from_parts(1_442, 0).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Multisig::Multisigs` (r:1 w:1)
    /// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[3, 100]`.
    /// The range of component `z` is `[0, 10000]`.
    fn as_multi_approve(s: u32, z: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `320`
        //  Estimated: `6811`
        // Minimum execution time: 27_375_000 picoseconds.
        Weight::from_parts(17_806_361, 6811)
            // Standard Error: 501
            .saturating_add(Weight::from_parts(107_042, 0).saturating_mul(s.into()))
            // Standard Error: 4
            .saturating_add(Weight::from_parts(1_491, 0).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Multisig::Multisigs` (r:1 w:1)
    /// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
    /// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `TxPause::PausedCalls` (r:1 w:0)
    /// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[2, 100]`.
    /// The range of component `z` is `[0, 10000]`.
    fn as_multi_complete(s: u32, z: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `571 + s * (33 ±0)`
        //  Estimated: `6811`
        // Minimum execution time: 54_427_000 picoseconds.
        Weight::from_parts(43_677_970, 6811)
            // Standard Error: 1_342
            .saturating_add(Weight::from_parts(154_697, 0).saturating_mul(s.into()))
            // Standard Error: 13
            .saturating_add(Weight::from_parts(1_534, 0).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Multisig::Multisigs` (r:1 w:1)
    /// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[2, 100]`.
    fn approve_as_multi_create(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `301 + s * (2 ±0)`
        //  Estimated: `6811`
        // Minimum execution time: 29_102_000 picoseconds.
        Weight::from_parts(30_317_105, 6811)
            // Standard Error: 903
            .saturating_add(Weight::from_parts(109_792, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Multisig::Multisigs` (r:1 w:1)
    /// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[2, 100]`.
    fn approve_as_multi_approve(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `320`
        //  Estimated: `6811`
        // Minimum execution time: 16_300_000 picoseconds.
        Weight::from_parts(17_358_877, 6811)
            // Standard Error: 522
            .saturating_add(Weight::from_parts(99_194, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Multisig::Multisigs` (r:1 w:1)
    /// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[2, 100]`.
    fn cancel_as_multi(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `492 + s * (1 ±0)`
        //  Estimated: `6811`
        // Minimum execution time: 30_147_000 picoseconds.
        Weight::from_parts(32_003_421, 6811)
            // Standard Error: 1_077
            .saturating_add(Weight::from_parts(108_567, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}