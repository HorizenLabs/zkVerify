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
use sp_runtime::SaturatedConversion;

#[benchmarks]
mod benchmarks {
    use super::*;

    const OFFSET_FACTOR: u64 = 10000;
    #[benchmark]
    fn publish_attestation() {
        // Setup code
        // benchmark taking into account situations where attestations left to be cleaned in storage
        // are bigger than one. shouldn't happen very often so we consider only 0.01% of the
        // maximum supported ones.
        let max_attestations = T::MaxStorageAttestations::get().saturated_into::<u64>();
        let offset = max_attestations
            .div_ceil(OFFSET_FACTOR)
            .saturated_into::<u64>();

        let mut hash: [u8; 32] = [0; 32];

        for id in 0..max_attestations + offset {
            if id < offset + 1 {
                // benchmark with double the minimum number of elements for publishing an attestation
                for _ in 0..T::MinProofsForPublishing::get() * 2 {
                    hash[0] += 1;
                    Values::<T>::insert(id, H256::from_slice(&hash), ());
                }
            } else {
                Values::<T>::insert(id, H256::default(), ());
            }
        }

        NextAttestation::<T>::set(max_attestations + offset - 1);

        assert_eq!(
            Values::<T>::iter_keys().count(),
            (((offset + 1) * T::MinProofsForPublishing::get().saturated_into::<u64>()) * 2
                + (max_attestations - 1)) as usize
        );

        #[extrinsic_call]
        publish_attestation(RawOrigin::Root);

        // Check that indeed old attestations have been removed
        let mut test_hash: [u8; 32] = [0; 32];
        for id in 0..offset + 1 {
            for _ in 0..T::MinProofsForPublishing::get() * 2 {
                test_hash[0] += 1;
                assert!(Values::<T>::try_get(id, H256::from_slice(&test_hash)).is_err());
            }
        }

        for id in offset + 1..max_attestations + 1 {
            assert!(Values::<T>::try_get(id, H256::default()).is_ok());
        }
    }

    #[cfg(test)]
    use crate::Pallet as Poe;
    impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test,);
}
