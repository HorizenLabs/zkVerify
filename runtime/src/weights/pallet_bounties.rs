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

/// Weight functions for `pallet_bounties`.
pub struct ZKVWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bounties::WeightInfo for ZKVWeight<T> {
    /// Storage: `Bounties::BountyCount` (r:1 w:1)
    /// Proof: `Bounties::BountyCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
    /// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::Bounties` (r:0 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// The range of component `d` is `[0, 16384]`.
    fn propose_bounty(d: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `177`
        //  Estimated: `3593`
        // Minimum execution time: 21_824_000 picoseconds.
        Weight::from_parts(23_437_825, 0)
            .saturating_add(Weight::from_parts(0, 3593))
            // Standard Error: 8
            .saturating_add(Weight::from_parts(637, 0).saturating_mul(d.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::BountyApprovals` (r:1 w:1)
    /// Proof: `Bounties::BountyApprovals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
    fn approve_bounty() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `269`
        //  Estimated: `3642`
        // Minimum execution time: 11_204_000 picoseconds.
        Weight::from_parts(11_734_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    fn propose_curator() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `289`
        //  Estimated: `3642`
        // Minimum execution time: 10_795_000 picoseconds.
        Weight::from_parts(11_210_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn unassign_curator() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `465`
        //  Estimated: `3642`
        // Minimum execution time: 34_043_000 picoseconds.
        Weight::from_parts(34_947_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    fn accept_curator() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `461`
        //  Estimated: `3642`
        // Minimum execution time: 24_470_000 picoseconds.
        Weight::from_parts(24_875_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
    /// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    fn award_bounty() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `368`
        //  Estimated: `3642`
        // Minimum execution time: 13_964_000 picoseconds.
        Weight::from_parts(14_434_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:3 w:3)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ChildrenCuratorFees` (r:1 w:1)
    /// Proof: `ChildBounties::ChildrenCuratorFees` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
    /// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    fn claim_bounty() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `732`
        //  Estimated: `8799`
        // Minimum execution time: 99_910_000 picoseconds.
        Weight::from_parts(101_642_000, 0)
            .saturating_add(Weight::from_parts(0, 8799))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(6))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
    /// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
    /// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    fn close_bounty_proposed() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `412`
        //  Estimated: `3642`
        // Minimum execution time: 35_756_000 picoseconds.
        Weight::from_parts(37_193_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `ChildBounties::ParentChildBounties` (r:1 w:0)
    /// Proof: `ChildBounties::ParentChildBounties` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::BountyDescriptions` (r:0 w:1)
    /// Proof: `Bounties::BountyDescriptions` (`max_values`: None, `max_size`: Some(16400), added: 18875, mode: `MaxEncodedLen`)
    fn close_bounty_active() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `648`
        //  Estimated: `6196`
        // Minimum execution time: 67_273_000 picoseconds.
        Weight::from_parts(69_165_000, 0)
            .saturating_add(Weight::from_parts(0, 6196))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    /// Storage: `Bounties::Bounties` (r:1 w:1)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    fn extend_bounty_expiry() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `325`
        //  Estimated: `3642`
        // Minimum execution time: 11_562_000 picoseconds.
        Weight::from_parts(12_821_000, 0)
            .saturating_add(Weight::from_parts(0, 3642))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: `Bounties::BountyApprovals` (r:1 w:1)
    /// Proof: `Bounties::BountyApprovals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
    /// Storage: `Bounties::Bounties` (r:100 w:100)
    /// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:200 w:200)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `b` is `[0, 100]`.
    fn spend_funds(b: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + b * (297 ±0)`
        //  Estimated: `1887 + b * (5206 ±0)`
        // Minimum execution time: 2_835_000 picoseconds.
        Weight::from_parts(2_915_000, 0)
            .saturating_add(Weight::from_parts(0, 1887))
            // Standard Error: 18_278
            .saturating_add(Weight::from_parts(32_861_747, 0).saturating_mul(b.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(b.into())))
            .saturating_add(T::DbWeight::get().writes(1))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
            .saturating_add(Weight::from_parts(0, 5206).saturating_mul(b.into()))
    }
}