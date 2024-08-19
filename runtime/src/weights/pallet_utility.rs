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

/// Weight functions for `pallet_utility`.
pub struct ZKVWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for ZKVWeight<T> {
    /// The range of component `c` is `[0, 1000]`.
    fn batch(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_791_000 picoseconds.
        Weight::from_parts(7_720_310, 0)
            .saturating_add(Weight::from_parts(0, 0))
            // Standard Error: 2_420
            .saturating_add(Weight::from_parts(5_114_338, 0).saturating_mul(c.into()))
    }
    fn as_derivative() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_892_000 picoseconds.
        Weight::from_parts(5_122_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
    }
    /// The range of component `c` is `[0, 1000]`.
    fn batch_all(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_816_000 picoseconds.
        Weight::from_parts(12_736_198, 0)
            .saturating_add(Weight::from_parts(0, 0))
            // Standard Error: 2_696
            .saturating_add(Weight::from_parts(5_378_828, 0).saturating_mul(c.into()))
    }
    fn dispatch_as() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 8_890_000 picoseconds.
        Weight::from_parts(9_286_000, 0)
            .saturating_add(Weight::from_parts(0, 0))
    }
    /// The range of component `c` is `[0, 1000]`.
    fn force_batch(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_823_000 picoseconds.
        Weight::from_parts(7_235_613, 0)
            .saturating_add(Weight::from_parts(0, 0))
            // Standard Error: 2_817
            .saturating_add(Weight::from_parts(5_113_539, 0).saturating_mul(c.into()))
    }
}