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

//! Autogenerated weights for `pallet_poe`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-09-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `LAPTOP-5V1NHBSA`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /home/danielecker/hl-crypto/zkVerify/target/production/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-poe
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/danielecker/hl-crypto/zkVerify/HEADER-APACHE2
// --output
// pallets/proof_of_existence/src/weight.rs
// --template
// /home/danielecker/hl-crypto/zkVerify/node/zkv-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_poe`.
pub trait WeightInfo {
    fn publish_attestations() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    /// Storage: `Poe::NextAttestation` (r:1 w:1)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:65 w:32)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Poe::OldestAttestation` (r:1 w:1)
    /// Proof: `Poe::OldestAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn publish_attestations() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `7040`
        //  Estimated: `166545`
        // Minimum execution time: 219_108_000 picoseconds.
        Weight::from_parts(251_585_000, 166545)
            .saturating_add(RocksDbWeight::get().reads(67_u64))
            .saturating_add(RocksDbWeight::get().writes(34_u64))
    }
}