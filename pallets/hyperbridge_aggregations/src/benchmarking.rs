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
use frame_support::traits::fungible::Inspect;
use frame_system::RawOrigin;
use hex_literal::hex;
use ismp::host::StateMachine;
use sp_runtime::traits::Zero;

type BalanceOf<T> = <<T as pallet_ismp::Config>::Currency as Inspect<
    <T as frame_system::Config>::AccountId,
>>::Balance;

pub mod utils {
    use super::*;
    use frame_support::traits::fungible::Mutate;
    use sp_runtime::traits::Bounded;

    /// Return a whitelisted account with enough founds to do anything.
    pub fn funded_account<T: Config>() -> T::AccountId {
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::set_balance(&caller, BalanceOf::<T>::max_value() / 2u32.into());
        caller
    }
}

#[benchmarks]
mod benchmarks {
    use super::utils::*;
    use super::*;

    #[benchmark]
    fn dispatch_aggregation() {
        let caller: T::AccountId = funded_account::<T>();

        pub static DEFAULT_EMPTY_ATT: [u8; 32] =
            hex!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563");

        pub static TEST_CONTRACT: [u8; 20] = hex!("d9145CCE52D386f254917e481eB44e9943F39138");

        // Prepare the parameters
        let params = Params {
            aggregation_id: 1u64,
            aggregation: sp_core::H256(DEFAULT_EMPTY_ATT),
            module: sp_core::H160(TEST_CONTRACT),
            destination: StateMachine::Kusama(4009),
            timeout: 0,
            fee: Zero::zero(),
        };

        #[extrinsic_call]
        dispatch_aggregation(RawOrigin::Signed(caller), params);

        // Add assertion to verify the dispatch was successful
    }

    #[cfg(test)]
    use crate::Pallet as HyperbridgeAggregations;
    impl_benchmark_test_suite!(
        HyperbridgeAggregations,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
