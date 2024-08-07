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

//! Autogenerated weights for `pallet_groth16_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-08-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `d109875a036e`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-groth16-verifier
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
// /data/benchmark/runtime/src/weights/pallet_groth16_verifier.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.3HgX1SvP0b

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_groth16_verifier` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_groth16_verifier::WeightInfo for ZKVWeight<T> {
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bn254(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 24_780_554_000 picoseconds.
        Weight::from_parts(24_889_565_694, 3537)
            // Standard Error: 2_691_754
            .saturating_add(Weight::from_parts(1_593_769_114, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bls12_381(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 26_284_161_000 picoseconds.
        Weight::from_parts(26_379_850_901, 3537)
            // Standard Error: 2_140_175
            .saturating_add(Weight::from_parts(2_006_855_214, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bn254_with_vk_hash(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `663 + n * (66 ±0)`
        //  Estimated: `7413`
        // Minimum execution time: 19_196_119_000 picoseconds.
        Weight::from_parts(19_343_223_152, 7413)
            // Standard Error: 2_677_874
            .saturating_add(Weight::from_parts(1_086_636_897, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn submit_proof_bls12_381_with_vk_hash(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `919 + n * (98 ±0)`
        //  Estimated: `7413`
        // Minimum execution time: 23_739_030_000 picoseconds.
        Weight::from_parts(23_808_394_549, 7413)
            // Standard Error: 2_656_418
            .saturating_add(Weight::from_parts(1_616_956_915, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn register_vk_bn254(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 5_592_509_000 picoseconds.
        Weight::from_parts(5_620_146_117, 0)
            // Standard Error: 237_823
            .saturating_add(Weight::from_parts(505_042_930, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `SettlementGroth16Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementGroth16Pallet::Vks` (`max_values`: None, `max_size`: Some(3948), added: 6423, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 16]`.
    fn register_vk_bls12_381(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_544_112_000 picoseconds.
        Weight::from_parts(2_555_989_840, 0)
            // Standard Error: 52_345
            .saturating_add(Weight::from_parts(399_656_864, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
