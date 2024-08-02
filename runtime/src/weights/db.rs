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

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-08-01 (Y/M/D)
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//!
//! DATABASE: `RocksDb`, RUNTIME: `ZKV Testnet`
//! BLOCK-NUM: `BlockId::Number(1130146)`
//! SKIP-WRITE: `false`, SKIP-READ: `false`, WARMUPS: `1`
//! STATE-VERSION: `V1`, STATE-CACHE-SIZE: ``
//! WEIGHT-PATH: `runtime/src/weights/db.rs`
//! METRIC: `Average`, WEIGHT-MUL: `1.1`, WEIGHT-ADD: `0`

// Executed Command:
//   ./target/release/zkv-node
//   benchmark
//   storage
//   --template-path
//   node/zkv-db-weight-template.hbs
//   --header
//   HEADER-APACHE2
//   --mul
//   1.1
//   --state-version
//   1
//   --weight-path
//   runtime/src/weights/db.rs
//   -d
//   /home/mdamico/.local/share/zkv-node/

/// Storage DB weights for the `ZKV Testnet` runtime and `RocksDb`.
pub mod constants {
    use frame_support::weights::constants;
    use sp_core::parameter_types;
    use sp_weights::RuntimeDbWeight;

    parameter_types! {
        /// By default, Substrate uses `RocksDB`, so this will be the weight used throughout
        /// the runtime.
        pub const RocksDbWeight: RuntimeDbWeight = RuntimeDbWeight {
            // Time to read one storage item.
            // Calculated by multiplying the *Average* of all values with `1.1` and adding `0`.
            //
            // Stats nanoseconds:
            //   Min, Max: 1_387, 105_875
            //   Average:  10_263
            //   Median:   10_546
            //   Std-Dev:  2515.65
            //
            // Percentiles nanoseconds:
            //   99th: 17_430
            //   95th: 13_004
            //   75th: 11_234
            read: 11_290 * constants::WEIGHT_REF_TIME_PER_NANOS,

            // Time to write one storage item.
            // Calculated by multiplying the *Average* of all values with `1.1` and adding `0`.
            //
            // Stats nanoseconds:
            //   Min, Max: 8_948, 1_479_986
            //   Average:  23_656
            //   Median:   17_285
            //   Std-Dev:  17004.92
            //
            // Percentiles nanoseconds:
            //   99th: 45_935
            //   95th: 41_207
            //   75th: 34_361
            write: 26_022 * constants::WEIGHT_REF_TIME_PER_NANOS,
        };
    }

    #[cfg(test)]
    mod test_db_weights {
        use super::constants::RocksDbWeight as W;
        use sp_weights::constants;

        /// Checks that all weights exist and have sane values.
        // NOTE: If this test fails but you are sure that the generated values are fine,
        // you can delete it.
        #[test]
        fn bound() {
            // At least 1 µs.
            assert!(
                W::get().reads(1).ref_time() >= constants::WEIGHT_REF_TIME_PER_MICROS,
                "Read weight should be at least 1 µs."
            );
            assert!(
                W::get().writes(1).ref_time() >= constants::WEIGHT_REF_TIME_PER_MICROS,
                "Write weight should be at least 1 µs."
            );
            // At most 1 ms.
            assert!(
                W::get().reads(1).ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
                "Read weight should be at most 1 ms."
            );
            assert!(
                W::get().writes(1).ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
                "Write weight should be at most 1 ms."
            );
        }
    }
}
