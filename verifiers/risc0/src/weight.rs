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

//! Autogenerated weights for `pallet_risc0_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 31.0.0
//! DATE: 2024-07-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `giacomo-Virtual-Machine`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /home/giacomo/Programming/zkVerify/target/production/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-risc0-verifier
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/giacomo/Programming/zkVerify/HEADER-APACHE2
// --output
// verifiers/risc0/src/weight.rs
// --template
// /home/giacomo/Programming/zkVerify/node/zkv-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_risc0_verifier`.
pub trait WeightInfo {
    fn submit_proof_cycle_2_pow_12() -> Weight;
    fn submit_proof_cycle_2_pow_13() -> Weight;
    fn submit_proof_cycle_2_pow_14() -> Weight;
    fn submit_proof_cycle_2_pow_15() -> Weight;
    fn submit_proof_cycle_2_pow_16() -> Weight;
    fn submit_proof_cycle_2_pow_17() -> Weight;
    fn submit_proof_cycle_2_pow_18() -> Weight;
    fn submit_proof_cycle_2_pow_19() -> Weight;
    fn submit_proof_cycle_2_pow_20() -> Weight;
    fn submit_proof_cycle_2_pow_21() -> Weight;
    fn submit_proof_cycle_2_pow_22() -> Weight;
    fn submit_proof_cycle_2_pow_23() -> Weight;
    fn submit_proof_cycle_2_pow_24() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_12() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_13() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_14() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_15() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_16() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_17() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_18() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_19() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_20() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_21() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_22() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_23() -> Weight;
    fn submit_proof_with_vk_hash_cycle_2_pow_24() -> Weight;
    fn register_vk() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_12() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 13_093_835_000 picoseconds.
        Weight::from_parts(13_152_835_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_13() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 13_063_134_000 picoseconds.
        Weight::from_parts(13_160_436_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_14() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 13_906_843_000 picoseconds.
        Weight::from_parts(13_964_044_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_15() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 13_896_443_000 picoseconds.
        Weight::from_parts(13_948_744_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_16() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 13_912_143_000 picoseconds.
        Weight::from_parts(13_971_744_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_17() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 13_897_642_000 picoseconds.
        Weight::from_parts(13_953_943_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_18() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 14_701_352_000 picoseconds.
        Weight::from_parts(14_841_252_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_19() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 15_533_160_000 picoseconds.
        Weight::from_parts(15_610_560_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_20() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 16_565_570_000 picoseconds.
        Weight::from_parts(16_601_971_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_21() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 29_290_101_000 picoseconds.
        Weight::from_parts(29_402_103_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_22() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 44_713_460_000 picoseconds.
        Weight::from_parts(44_816_361_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_23() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 76_266_184_000 picoseconds.
        Weight::from_parts(76_606_787_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_cycle_2_pow_24() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `74`
        //  Estimated: `3537`
        // Minimum execution time: 139_292_732_000 picoseconds.
        Weight::from_parts(139_924_938_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_12() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 13_200_436_000 picoseconds.
        Weight::from_parts(13_304_837_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_13() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 13_229_136_000 picoseconds.
        Weight::from_parts(13_330_037_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_14() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 14_068_245_000 picoseconds.
        Weight::from_parts(14_179_346_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_15() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 14_114_045_000 picoseconds.
        Weight::from_parts(14_160_145_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_16() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 14_078_445_000 picoseconds.
        Weight::from_parts(14_157_446_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_17() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 14_085_045_000 picoseconds.
        Weight::from_parts(14_152_745_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_18() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 14_919_154_000 picoseconds.
        Weight::from_parts(15_005_655_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_19() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 15_709_561_000 picoseconds.
        Weight::from_parts(15_803_663_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_20() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 16_738_773_000 picoseconds.
        Weight::from_parts(16_812_873_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_21() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 29_548_804_000 picoseconds.
        Weight::from_parts(29_683_905_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_22() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 45_010_463_000 picoseconds.
        Weight::from_parts(45_275_165_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_23() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 76_347_685_000 picoseconds.
        Weight::from_parts(76_997_591_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    /// Storage: `Poe::NextAttestation` (r:1 w:0)
    /// Proof: `Poe::NextAttestation` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::Values` (r:1 w:1)
    /// Proof: `Poe::Values` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Poe::FirstInsertionTime` (r:0 w:1)
    /// Proof: `Poe::FirstInsertionTime` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    fn submit_proof_with_vk_hash_cycle_2_pow_24() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `3537`
        // Minimum execution time: 137_884_617_000 picoseconds.
        Weight::from_parts(138_707_026_000, 3537)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `SettlementRisc0Pallet::Vks` (r:0 w:1)
    /// Proof: `SettlementRisc0Pallet::Vks` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
    fn register_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 5_300_000 picoseconds.
        Weight::from_parts(5_500_000, 0)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}