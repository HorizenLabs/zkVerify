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

use crate::groth16::{Curve, Groth16};
#[allow(unused)]
use crate::Pallet as SettlementGroth16Pallet;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn submit_proof_bn254(n: Linear<0, <T as Config>::MAX_NUM_INPUTS>) {
        // setup code
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16::get_instance(n as usize, None, Curve::Bn254);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), proof, vk.into(), inputs);
    }

    #[benchmark]
    fn submit_proof_bls12_381(n: Linear<0, <T as Config>::MAX_NUM_INPUTS>) {
        // setup code
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16::get_instance(n as usize, None, Curve::Bls12_381);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), proof, vk.into(), inputs);
    }

    impl_benchmark_test_suite!(
        SettlementGroth16Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
