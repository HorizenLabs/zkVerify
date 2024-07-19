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

/// Weight functions for `pallet_conviction_voting`.
pub struct NHWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_conviction_voting::WeightInfo for NHWeight<T> {
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
    /// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(329), added: 2804, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn vote_new() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `13480`
        //  Estimated: `42428`
        // Minimum execution time: 136_288_000 picoseconds.
        Weight::from_parts(147_883_000, 0)
            .saturating_add(Weight::from_parts(0, 42428))
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
    /// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(329), added: 2804, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn vote_existing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `14201`
        //  Estimated: `83866`
        // Minimum execution time: 169_651_000 picoseconds.
        Weight::from_parts(177_209_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(6))
    }
    /// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    fn remove_vote() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `13919`
        //  Estimated: `83866`
        // Minimum execution time: 139_988_000 picoseconds.
        Weight::from_parts(152_950_000, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    /// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    fn remove_other_vote() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `13005`
        //  Estimated: `30706`
        // Minimum execution time: 84_340_000 picoseconds.
        Weight::from_parts(89_490_000, 0)
            .saturating_add(Weight::from_parts(0, 30706))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
    /// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:512 w:512)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(329), added: 2804, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
    /// The range of component `r` is `[0, 512]`.
    fn delegate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `28987 + r * (364 ±0)`
        //  Estimated: `83866 + r * (3411 ±0)`
        // Minimum execution time: 63_628_000 picoseconds.
        Weight::from_parts(808_059_906, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            // Standard Error: 79_356
            .saturating_add(Weight::from_parts(20_224_140, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(6))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 3411).saturating_mul(r.into()))
    }
    /// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
    /// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
    /// Storage: `Referenda::ReferendumInfoFor` (r:512 w:512)
    /// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:2 w:2)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
    /// The range of component `r` is `[0, 512]`.
    fn undelegate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `28868 + r * (364 ±0)`
        //  Estimated: `83866 + r * (3411 ±0)`
        // Minimum execution time: 38_682_000 picoseconds.
        Weight::from_parts(727_546_385, 0)
            .saturating_add(Weight::from_parts(0, 83866))
            // Standard Error: 54_985
            .saturating_add(Weight::from_parts(20_577_723, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(4))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 3411).saturating_mul(r.into()))
    }
    /// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
    /// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
    /// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(329), added: 2804, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
    fn unlock() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `12270`
        //  Estimated: `30706`
        // Minimum execution time: 98_219_000 picoseconds.
        Weight::from_parts(106_083_000, 0)
            .saturating_add(Weight::from_parts(0, 30706))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
}