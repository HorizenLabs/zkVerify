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

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_core::{Get, H256};

#[benchmarks]
mod benchmarks {
    use super::{Pallet as Poe, *};

    #[benchmark]
    fn publish_attestation() {
        // Setup code
        let max_attestations = T::MaxStorageAttestations::get();
        let mut hash: [u8; 32] = [0; 32];
        // benchmark with double the maximum number of attestations in storage
        for id in 0..max_attestations * 2 {
            // benchmark with double the minimum number of elements for publishing an attestation
            for _ in 0..T::MinProofsForPublishing::get() * 2 {
                hash[0] += 1;
                Values::<T>::insert(id as u64, H256::from_slice(&hash), ());
            }
        }

        // Check all attestations are present in storage
        let mut test_hash: [u8; 32] = [0; 32];
        for id in 0..max_attestations * 2 {
            for _ in 0..T::MinProofsForPublishing::get() * 2 {
                test_hash[0] += 1;
                assert!(Values::<T>::try_get(id as u64, H256::from_slice(&test_hash)).is_ok());
            }
        }

        #[extrinsic_call]
        publish_attestation(RawOrigin::Root);

        // Check that indeed old attestations have been removed
        let mut test_hash: [u8; 32] = [0; 32];
        for id in 0..max_attestations  {
            for _ in 0..T::MinProofsForPublishing::get() * 2 {
                test_hash[0] += 1;
                assert!(Values::<T>::try_get(id as u64, H256::from_slice(&test_hash)).is_err());
            }
        }

        for id in max_attestations..max_attestations * 2 {
            for _ in 0..T::MinProofsForPublishing::get() * 2 {
                test_hash[0] += 1;
                assert!(Values::<T>::try_get(id as u64, H256::from_slice(&test_hash)).is_ok());
            }
        }
    }

    impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test,);
}
