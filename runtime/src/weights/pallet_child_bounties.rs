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

/// Weight functions for `pallet_child_bounties`.
pub struct ZKVWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_child_bounties::WeightInfo for ZKVWeight<T> {
    /// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::Bounties` (r:1 w:0)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBountyCount` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBountyCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBountyDescriptions` (r:0 w:1)
    /// Proof: `ChildBounties::ChildBountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBounties` (r:0 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    /// The range of component `d` is `[0, 16384]`.
    fn add_child_bounty(d: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `508`
        //  Estimated: `6196`
        // Minimum execution time: 59_225_000 picoseconds.
        Weight::from_parts(60_300_197, 0)
            .saturating_add(Weight::from_parts(0, 6196))
            // Standard Error: 47
            .saturating_add(Weight::from_parts(793, 0).saturating_mul(d.into()))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(6))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:0)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
    /// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
    fn propose_curator() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `562`
        //  Estimated: `3642`
        // Minimum execution time: 17_752_000 picoseconds.
        Weight::from_parts(18_591_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:0)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn accept_curator() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `708`
        //  Estimated: `3642`
        // Minimum execution time: 29_052_000 picoseconds.
        Weight::from_parts(30_077_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::Bounties` (r:1 w:0)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn unassign_curator() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `708`
        //  Estimated: `3642`
        // Minimum execution time: 40_527_000 picoseconds.
        Weight::from_parts(41_204_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:0)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    fn award_child_bounty() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `605`
        //  Estimated: `3642`
        // Minimum execution time: 19_283_000 picoseconds.
        Weight::from_parts(19_797_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:3 w:3)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBountyDescriptions` (r:0 w:1)
    /// Proof: `ChildBounties::ChildBountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    fn claim_child_bounty() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `577`
        //  Estimated: `8799`
        // Minimum execution time: 100_048_000 picoseconds.
        Weight::from_parts(101_976_000, 0)
            .saturating_add(Weight::from_parts(0, 8799))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(6))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:0)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
    /// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBountyDescriptions` (r:0 w:1)
    /// Proof: `ChildBounties::ChildBountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    fn close_child_bounty_added() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `808`
        //  Estimated: `6196`
        // Minimum execution time: 65_320_000 picoseconds.
        Weight::from_parts(66_554_000, 0)
            .saturating_add(Weight::from_parts(0, 6196))
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(6))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:0)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ChildBounties` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:3 w:3)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
    /// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ParentChildBounties` (r:1 w:1)
    /// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildBountyDescriptions` (r:0 w:1)
    /// Proof: `ChildBounties::ChildBountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    fn close_child_bounty_active() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `995`
        //  Estimated: `8799`
        // Minimum execution time: 80_207_000 picoseconds.
        Weight::from_parts(81_635_000, 0)
            .saturating_add(Weight::from_parts(0, 8799))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(7))
    }
}