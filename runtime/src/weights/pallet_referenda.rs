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

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-08-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `8097b66c43ea`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-referenda
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_referenda.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.3HgX1SvP0b

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_referenda` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_referenda::WeightInfo for ZKVWeight<T> {
    /// Storage: `Referenda::ReferendumCount` (r:1 w:1)
    /// Proof: `Referenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:0 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    fn submit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `181`
        //  Estimated: `13928`
        // Minimum execution time: 26_840_000 picoseconds.
        Weight::from_parts(27_712_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Retries` (r:0 w:1)
    /// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
    fn place_decision_deposit_preparing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `434`
        //  Estimated: `26866`
        // Minimum execution time: 36_949_000 picoseconds.
        Weight::from_parts(37_901_000, 26866)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Retries` (r:0 w:1)
    /// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
    fn place_decision_deposit_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3221`
        //  Estimated: `13928`
        // Minimum execution time: 48_911_000 picoseconds.
        Weight::from_parts(50_535_000, 13928)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Retries` (r:0 w:1)
    /// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
    fn place_decision_deposit_not_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3241`
        //  Estimated: `13928`
        // Minimum execution time: 49_543_000 picoseconds.
        Weight::from_parts(50_114_000, 13928)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Retries` (r:0 w:1)
    /// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
    fn place_decision_deposit_passing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `434`
        //  Estimated: `26866`
        // Minimum execution time: 43_883_000 picoseconds.
        Weight::from_parts(44_704_000, 26866)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Retries` (r:0 w:1)
    /// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
    fn place_decision_deposit_failing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `434`
        //  Estimated: `26866`
        // Minimum execution time: 42_630_000 picoseconds.
        Weight::from_parts(43_342_000, 26866)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    fn refund_decision_deposit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `351`
        //  Estimated: `3831`
        // Minimum execution time: 23_394_000 picoseconds.
        Weight::from_parts(23_865_000, 3831)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    fn refund_submission_deposit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `303`
        //  Estimated: `3831`
        // Minimum execution time: 22_642_000 picoseconds.
        Weight::from_parts(23_394_000, 3831)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Retries` (r:0 w:1)
    /// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
    fn cancel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `380`
        //  Estimated: `26866`
        // Minimum execution time: 25_788_000 picoseconds.
        Weight::from_parts(26_951_000, 26866)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::MetadataOf` (r:1 w:0)
    /// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Retries` (r:0 w:1)
    /// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
    fn kill() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `583`
        //  Estimated: `26866`
        // Minimum execution time: 56_717_000 picoseconds.
        Weight::from_parts(58_069_000, 26866)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Referenda::TrackQueue` (r:1 w:0)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    fn one_fewer_deciding_queue_empty() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `174`
        //  Estimated: `5477`
        // Minimum execution time: 8_516_000 picoseconds.
        Weight::from_parts(8_846_000, 5477)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn one_fewer_deciding_failing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3149`
        //  Estimated: `13928`
        // Minimum execution time: 33_593_000 picoseconds.
        Weight::from_parts(34_334_000, 13928)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn one_fewer_deciding_passing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3149`
        //  Estimated: `13928`
        // Minimum execution time: 35_497_000 picoseconds.
        Weight::from_parts(36_268_000, 13928)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_requeued_insertion() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3011`
        //  Estimated: `5477`
        // Minimum execution time: 17_823_000 picoseconds.
        Weight::from_parts(18_474_000, 5477)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_requeued_slide() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3011`
        //  Estimated: `5477`
        // Minimum execution time: 17_974_000 picoseconds.
        Weight::from_parts(18_324_000, 5477)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3015`
        //  Estimated: `5477`
        // Minimum execution time: 21_881_000 picoseconds.
        Weight::from_parts(22_342_000, 5477)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_not_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3035`
        //  Estimated: `5477`
        // Minimum execution time: 21_891_000 picoseconds.
        Weight::from_parts(22_352_000, 5477)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_no_deposit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `332`
        //  Estimated: `13928`
        // Minimum execution time: 17_963_000 picoseconds.
        Weight::from_parts(18_595_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_preparing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `380`
        //  Estimated: `13928`
        // Minimum execution time: 17_754_000 picoseconds.
        Weight::from_parts(18_284_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    fn nudge_referendum_timed_out() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `278`
        //  Estimated: `3831`
        // Minimum execution time: 11_892_000 picoseconds.
        Weight::from_parts(12_563_000, 3831)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_begin_deciding_failing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `380`
        //  Estimated: `13928`
        // Minimum execution time: 23_043_000 picoseconds.
        Weight::from_parts(23_594_000, 13928)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_begin_deciding_passing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `380`
        //  Estimated: `13928`
        // Minimum execution time: 23_804_000 picoseconds.
        Weight::from_parts(24_597_000, 13928)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_begin_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `433`
        //  Estimated: `13928`
        // Minimum execution time: 19_397_000 picoseconds.
        Weight::from_parts(19_907_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_end_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `416`
        //  Estimated: `13928`
        // Minimum execution time: 19_767_000 picoseconds.
        Weight::from_parts(20_659_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_continue_not_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `433`
        //  Estimated: `13928`
        // Minimum execution time: 19_206_000 picoseconds.
        Weight::from_parts(19_637_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_continue_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `437`
        //  Estimated: `13928`
        // Minimum execution time: 18_294_000 picoseconds.
        Weight::from_parts(18_966_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Lookup` (r:1 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    fn nudge_referendum_approved() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `437`
        //  Estimated: `26866`
        // Minimum execution time: 28_864_000 picoseconds.
        Weight::from_parts(30_287_000, 26866)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    fn nudge_referendum_rejected() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `433`
        //  Estimated: `13928`
        // Minimum execution time: 20_278_000 picoseconds.
        Weight::from_parts(21_260_000, 13928)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::MetadataOf` (r:0 w:1)
    /// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn set_some_metadata() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `460`
        //  Estimated: `3831`
        // Minimum execution time: 18_284_000 picoseconds.
        Weight::from_parts(18_805_000, 3831)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(366), added: 2841, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::MetadataOf` (r:1 w:1)
    /// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn clear_metadata() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `355`
        //  Estimated: `3831`
        // Minimum execution time: 13_465_000 picoseconds.
        Weight::from_parts(14_066_000, 3831)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
