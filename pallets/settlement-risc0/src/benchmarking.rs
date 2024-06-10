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

//! Benchmarking setup for pallet-settlement-risc0
use super::*;

#[allow(unused)]
use crate::Pallet as SettlementRisc0Pallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

include!("data.rs");

benchmarks! {
    submit_proof {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), VALID_VK.into(), VALID_PROOF.to_vec().into(), VALID_PUBS.to_vec().into())
}

impl_benchmark_test_suite!(
    SettlementRisc0Pallet,
    crate::mock::new_test_ext(),
    crate::mock::Test,
);
