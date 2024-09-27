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

/// Weight functions for `pallet_bags_list`.
pub struct ZKVWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bags_list::WeightInfo for ZKVWeight<T> {
    /// Storage: Staking Bonded (r:1 w:0)
    /// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
    /// Storage: Staking Ledger (r:1 w:0)
    /// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
    /// Storage: VoterList ListNodes (r:4 w:4)
    /// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
    /// Storage: VoterList ListBags (r:1 w:1)
    /// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
    fn rebag_non_terminal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1656`
        //  Estimated: `11506`
        // Minimum execution time: 60_240_000 picoseconds.
        Weight::from_parts(62_834_000, 0)
            .saturating_add(Weight::from_parts(0, 11506))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    /// Storage: Staking Bonded (r:1 w:0)
    /// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
    /// Storage: Staking Ledger (r:1 w:0)
    /// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
    /// Storage: VoterList ListNodes (r:3 w:3)
    /// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
    /// Storage: VoterList ListBags (r:2 w:2)
    /// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
    fn rebag_terminal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1550`
        //  Estimated: `8877`
        // Minimum execution time: 59_084_000 picoseconds.
        Weight::from_parts(60_589_000, 0)
            .saturating_add(Weight::from_parts(0, 8877))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    /// Storage: VoterList ListNodes (r:4 w:4)
    /// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
    /// Storage: Staking Bonded (r:2 w:0)
    /// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
    /// Storage: Staking Ledger (r:2 w:0)
    /// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
    /// Storage: VoterList CounterForListNodes (r:1 w:1)
    /// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: VoterList ListBags (r:1 w:1)
    /// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
    fn put_in_front_of() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1861`
        //  Estimated: `11506`
        // Minimum execution time: 65_945_000 picoseconds.
        Weight::from_parts(67_429_000, 0)
            .saturating_add(Weight::from_parts(0, 11506))
            .saturating_add(T::DbWeight::get().reads(10))
            .saturating_add(T::DbWeight::get().writes(6))
    }
}