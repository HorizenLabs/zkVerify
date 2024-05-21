// Copyright 2024, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Benchmarking setup for pallet-settlement-fflonk
use super::*;

#[allow(unused)]
use crate::Pallet as SettlementFFlonkPallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

include!("proof.rs");

benchmarks! {
    submit_proof {
        // we can use a single input rather than complexity parameters
        // (https://paritytech.github.io/polkadot-sdk/master/frame_benchmarking/macro.benchmarks.html),
        // as the `submit_proof` extrinsic should have O(1) complexity

        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), VALID_PROOF.into())
}

impl_benchmark_test_suite!(
    SettlementFFlonkPallet,
    crate::mock::new_test_ext(),
    crate::mock::Test,
);
