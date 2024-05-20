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

use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_core::{Get, H256};

benchmarks! {
    publish_attestation {
        // benchmark with double the minimum number of elements for publishing an attestation
        let mut hash: [u8; 32] = [0; 32];
        for h in 0 .. T::MinProofsForPublishing::get() * 2 {
            hash[0] += 1;
            Values::<T>::insert(0, H256::from_slice(&hash), ());
        }
    }: _(RawOrigin::Root)
}

#[cfg(test)]
use crate::Pallet as Poe;
impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test,);
