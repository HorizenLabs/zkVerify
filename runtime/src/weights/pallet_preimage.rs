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

/// Weights for `pallet_preimage` using the New Horizen node and recommended hardware.
pub struct NHWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_preimage::WeightInfo for NHWeight<T> {
    fn ensure_updated(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `193 + n * (91 ±0)`
        //  Estimated: `3593 + n * (2566 ±0)`
        // Minimum execution time: 2_000_000 picoseconds.
        Weight::from_parts(2_000_000, 3593)
            // Standard Error: 13_720
            .saturating_add(Weight::from_parts(17_309_199, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2566).saturating_mul(n.into()))
    }

    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_preimage(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `215`
        //  Estimated: `3556`
        // Minimum execution time: 31_040_000 picoseconds.
        Weight::from_parts(31_236_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            // Standard Error: 1
            .saturating_add(Weight::from_parts(1_974, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_requested_preimage(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 18_025_000 picoseconds.
        Weight::from_parts(18_264_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            // Standard Error: 1
            .saturating_add(Weight::from_parts(1_974, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_no_deposit_preimage(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 17_122_000 picoseconds.
        Weight::from_parts(17_332_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            // Standard Error: 1
            .saturating_add(Weight::from_parts(1_968, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    fn unnote_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `361`
        //  Estimated: `3556`
        // Minimum execution time: 38_218_000 picoseconds.
        Weight::from_parts(39_841_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    fn unnote_no_deposit_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `216`
        //  Estimated: `3556`
        // Minimum execution time: 23_217_000 picoseconds.
        Weight::from_parts(24_246_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `260`
        //  Estimated: `3556`
        // Minimum execution time: 21_032_000 picoseconds.
        Weight::from_parts(21_844_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_no_deposit_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `216`
        //  Estimated: `3556`
        // Minimum execution time: 13_954_000 picoseconds.
        Weight::from_parts(14_501_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_unnoted_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `114`
        //  Estimated: `3556`
        // Minimum execution time: 14_874_000 picoseconds.
        Weight::from_parts(15_380_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn request_requested_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 10_199_000 picoseconds.
        Weight::from_parts(10_493_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    /// Storage: Preimage PreimageFor (r:0 w:1)
    /// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: MaxEncodedLen)
    fn unrequest_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `216`
        //  Estimated: `3556`
        // Minimum execution time: 21_772_000 picoseconds.
        Weight::from_parts(22_554_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn unrequest_unnoted_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 10_115_000 picoseconds.
        Weight::from_parts(10_452_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: Preimage StatusFor (r:1 w:1)
    /// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
    fn unrequest_multi_referenced_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 10_031_000 picoseconds.
        Weight::from_parts(10_310_000, 0)
            .saturating_add(Weight::from_parts(0, 3556))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}