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

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weights for `pallet_scheduler` using the New Horizen node and recommended hardware.
pub struct NHWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_scheduler::WeightInfo for NHWeight<T> {
    /// Storage: `Scheduler::IncompleteSince` (r:1 w:1)
    /// Proof: `Scheduler::IncompleteSince` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    fn service_agendas_base() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `68`
        //  Estimated: `1489`
        // Minimum execution time: 2_869_000 picoseconds.
        Weight::from_parts(3_109_000, 0)
            .saturating_add(Weight::from_parts(0, 1489))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 50]`.
    fn service_agenda_base(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `115 + s * (177 ±0)`
        //  Estimated: `42428`
        // Minimum execution time: 3_326_000 picoseconds.
        Weight::from_parts(5_818_563, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            // Standard Error: 1_261
            .saturating_add(Weight::from_parts(336_446, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn service_task_base() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_007_000 picoseconds.
        Weight::from_parts(3_197_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
    }
    /// Storage: `Preimage::PreimageFor` (r:1 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `Measured`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[128, 4194304]`.
    fn service_task_fetched(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `251 + s * (1 ±0)`
        //  Estimated: `3716 + s * (1 ±0)`
        // Minimum execution time: 16_590_000 picoseconds.
        Weight::from_parts(16_869_000, 0)
            .saturating_add(Weight::from_parts(0, 3716))
            // Standard Error: 9
            .saturating_add(Weight::from_parts(1_308, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(s.into()))
    }
    /// Storage: `Scheduler::Lookup` (r:0 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    fn service_task_named() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_320_000 picoseconds.
        Weight::from_parts(4_594_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn service_task_periodic() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_956_000 picoseconds.
        Weight::from_parts(3_216_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
    }
    fn execute_dispatch_signed() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_824_000 picoseconds.
        Weight::from_parts(1_929_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
    }
    fn execute_dispatch_unsigned() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_749_000 picoseconds.
        Weight::from_parts(1_916_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
    }
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 49]`.
    fn schedule(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `115 + s * (177 ±0)`
        //  Estimated: `42428`
        // Minimum execution time: 9_086_000 picoseconds.
        Weight::from_parts(11_733_696, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            // Standard Error: 1_362
            .saturating_add(Weight::from_parts(375_266, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Lookup` (r:0 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[1, 50]`.
    fn cancel(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `115 + s * (177 ±0)`
        //  Estimated: `42428`
        // Minimum execution time: 12_716_000 picoseconds.
        Weight::from_parts(12_529_180, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            // Standard Error: 867
            .saturating_add(Weight::from_parts(548_188, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Scheduler::Lookup` (r:1 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 49]`.
    fn schedule_named(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `292 + s * (185 ±0)`
        //  Estimated: `42428`
        // Minimum execution time: 12_053_000 picoseconds.
        Weight::from_parts(15_358_056, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            // Standard Error: 3_176
            .saturating_add(Weight::from_parts(421_589, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Scheduler::Lookup` (r:1 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[1, 50]`.
    fn cancel_named(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `318 + s * (185 ±0)`
        //  Estimated: `42428`
        // Minimum execution time: 14_803_000 picoseconds.
        Weight::from_parts(15_805_714, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            // Standard Error: 2_597
            .saturating_add(Weight::from_parts(611_053, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}