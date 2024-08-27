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

//! Autogenerated weights for `pallet_fflonk_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-08-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `39b161791d93`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-fflonk-verifier
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
// /data/benchmark/runtime/src/weights/pallet_fflonk_verifier.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.3HgX1SvP0b

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_fflonk_verifier` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_fflonk_verifier::WeightInfo for ZKVWeight<T> {
    /// Storage: `Attestation::NextAttestation` (r:1 w:0)
    /// Proof: `Attestation::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Attestation::Values` (r:1 w:1)
    /// Proof: `Attestation::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Attestation::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Attestation::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 23_624_815_000 picoseconds.
        Weight::from_parts(23_699_807_000, 3537)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementFFlonkPallet::Vks` (r:1 w:0)
    /// Proof: `SettlementFFlonkPallet::Vks` (`max_values`: None, `max_size`: Some(545), added: 3020, mode: `MaxEncodedLen`)
    /// Storage: `Attestation::NextAttestation` (r:1 w:0)
    /// Proof: `Attestation::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Attestation::Values` (r:1 w:1)
    /// Proof: `Attestation::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Attestation::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Attestation::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `652`
        //  Estimated: `4010`
        // Minimum execution time: 20_820_253_000 picoseconds.
        Weight::from_parts(20_901_085_000, 4010)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementFFlonkPallet::Vks` (r:0 w:1)
    /// Proof: `SettlementFFlonkPallet::Vks` (`max_values`: None, `max_size`: Some(545), added: 3020, mode: `MaxEncodedLen`)
    fn register_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_789_995_000 picoseconds.
        Weight::from_parts(2_801_156_000, 0)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
