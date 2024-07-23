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

/// Weight functions for `pallet_referenda`.
pub struct ZKVWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for ZKVWeight<T> {
    /// Storage: `Referenda::ReferendumCount` (r:1 w:1)
    /// Proof: `Referenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:0 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    fn submit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `186`
        //  Estimated: `42428`
        // Minimum execution time: 28_309_000 picoseconds.
        Weight::from_parts(29_092_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn place_decision_deposit_preparing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `439`
        //  Estimated: `83866`
        // Minimum execution time: 40_137_000 picoseconds.
        Weight::from_parts(40_975_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn place_decision_deposit_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3226`
        //  Estimated: `42428`
        // Minimum execution time: 53_644_000 picoseconds.
        Weight::from_parts(54_990_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn place_decision_deposit_not_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3246`
        //  Estimated: `42428`
        // Minimum execution time: 53_120_000 picoseconds.
        Weight::from_parts(54_303_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn place_decision_deposit_passing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `439`
        //  Estimated: `83866`
        // Minimum execution time: 48_446_000 picoseconds.
        Weight::from_parts(49_527_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn place_decision_deposit_failing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `439`
        //  Estimated: `83866`
        // Minimum execution time: 47_170_000 picoseconds.
        Weight::from_parts(48_568_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    fn refund_decision_deposit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `279`
        //  Estimated: `4401`
        // Minimum execution time: 22_858_000 picoseconds.
        Weight::from_parts(23_535_000, 0)
            .saturating_add(Weight::from_parts(0, 4401))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    fn refund_submission_deposit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `269`
        //  Estimated: `4401`
        // Minimum execution time: 23_093_000 picoseconds.
        Weight::from_parts(23_778_000, 0)
            .saturating_add(Weight::from_parts(0, 4401))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn cancel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `347`
        //  Estimated: `83866`
        // Minimum execution time: 26_219_000 picoseconds.
        Weight::from_parts(26_982_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::MetadataOf` (r:1 w:0)
    /// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn kill() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `588`
        //  Estimated: `83866`
        // Minimum execution time: 77_331_000 picoseconds.
        Weight::from_parts(78_613_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::TrackQueue` (r:1 w:0)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    fn one_fewer_deciding_queue_empty() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `102`
        //  Estimated: `5477`
        // Minimum execution time: 8_108_000 picoseconds.
        Weight::from_parts(8_497_000, 0)
            .saturating_add(Weight::from_parts(0, 5477))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn one_fewer_deciding_failing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3116`
        //  Estimated: `42428`
        // Minimum execution time: 39_409_000 picoseconds.
        Weight::from_parts(41_282_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn one_fewer_deciding_passing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `3116`
        //  Estimated: `42428`
        // Minimum execution time: 42_239_000 picoseconds.
        Weight::from_parts(43_303_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_requeued_insertion() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2939`
        //  Estimated: `5477`
        // Minimum execution time: 20_393_000 picoseconds.
        Weight::from_parts(20_859_000, 0)
            .saturating_add(Weight::from_parts(0, 5477))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_requeued_slide() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2939`
        //  Estimated: `5477`
        // Minimum execution time: 20_617_000 picoseconds.
        Weight::from_parts(21_626_000, 0)
            .saturating_add(Weight::from_parts(0, 5477))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2943`
        //  Estimated: `5477`
        // Minimum execution time: 24_457_000 picoseconds.
        Weight::from_parts(25_297_000, 0)
            .saturating_add(Weight::from_parts(0, 5477))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:0)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::TrackQueue` (r:1 w:1)
    /// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
    fn nudge_referendum_not_queued() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2963`
        //  Estimated: `5477`
        // Minimum execution time: 24_396_000 picoseconds.
        Weight::from_parts(25_169_000, 0)
            .saturating_add(Weight::from_parts(0, 5477))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_no_deposit() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `299`
        //  Estimated: `42428`
        // Minimum execution time: 17_827_000 picoseconds.
        Weight::from_parts(18_289_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_preparing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `347`
        //  Estimated: `42428`
        // Minimum execution time: 17_946_000 picoseconds.
        Weight::from_parts(18_590_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    fn nudge_referendum_timed_out() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `4401`
        // Minimum execution time: 11_145_000 picoseconds.
        Weight::from_parts(11_686_000, 0)
            .saturating_add(Weight::from_parts(0, 4401))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_begin_deciding_failing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `347`
        //  Estimated: `42428`
        // Minimum execution time: 24_506_000 picoseconds.
        Weight::from_parts(25_095_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::DecidingCount` (r:1 w:1)
    /// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_begin_deciding_passing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `347`
        //  Estimated: `42428`
        // Minimum execution time: 26_090_000 picoseconds.
        Weight::from_parts(26_804_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_begin_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `400`
        //  Estimated: `42428`
        // Minimum execution time: 24_053_000 picoseconds.
        Weight::from_parts(24_870_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_end_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `383`
        //  Estimated: `42428`
        // Minimum execution time: 24_788_000 picoseconds.
        Weight::from_parts(25_701_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_continue_not_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `400`
        //  Estimated: `42428`
        // Minimum execution time: 23_284_000 picoseconds.
        Weight::from_parts(24_258_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_continue_confirming() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `404`
        //  Estimated: `42428`
        // Minimum execution time: 22_230_000 picoseconds.
        Weight::from_parts(23_146_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Lookup` (r:1 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    fn nudge_referendum_approved() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `404`
        //  Estimated: `83866`
        // Minimum execution time: 33_389_000 picoseconds.
        Weight::from_parts(34_560_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn nudge_referendum_rejected() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `400`
        //  Estimated: `42428`
        // Minimum execution time: 24_457_000 picoseconds.
        Weight::from_parts(25_279_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::MetadataOf` (r:0 w:1)
    /// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn set_some_metadata() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `350`
        //  Estimated: `4401`
        // Minimum execution time: 16_834_000 picoseconds.
        Weight::from_parts(17_424_000, 0)
            .saturating_add(Weight::from_parts(0, 4401))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::MetadataOf` (r:1 w:1)
    /// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
    fn clear_metadata() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `283`
        //  Estimated: `4401`
        // Minimum execution time: 13_360_000 picoseconds.
        Weight::from_parts(13_881_000, 0)
            .saturating_add(Weight::from_parts(0, 4401))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}