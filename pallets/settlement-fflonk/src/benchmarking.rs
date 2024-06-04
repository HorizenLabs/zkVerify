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

#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as SettlementFFlonkPallet;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

include!("resources.rs");

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn submit_proof_default() {
        // setup code
        let caller = whitelisted_caller();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), VALID_PROOF.into(), None);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash() {
        // setup code
        let caller = whitelisted_caller();
        let vk: crate::vk::Vk = fflonk_verifier::VerificationKey::default().into();
        Vks::<T>::insert(DEFAULT_VK_HASH.clone(), vk);
        let vk_or_hash = Some(VkOrHash::Hash(DEFAULT_VK_HASH.clone()));

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), VALID_PROOF.into(), vk_or_hash);
    }

    #[benchmark]
    fn submit_proof_with_vk() {
        // setup code
        let caller = whitelisted_caller();
        let vk_or_hash = Some(VkOrHash::Vk(Box::new(
            fflonk_verifier::VerificationKey::default().into(),
        )));

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), VALID_PROOF.into(), vk_or_hash);
    }

    #[benchmark]
    fn register_vk() {
        // setup code
        let caller = whitelisted_caller();
        let vk = Box::new(fflonk_verifier::VerificationKey::default().into());

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk);

        // Verify
        assert!(Vks::<T>::get(&DEFAULT_VK_HASH.clone()).is_some());
    }

    impl_benchmark_test_suite!(
        SettlementFFlonkPallet,
        crate::mock::test_ext(),
        crate::mock::Test,
    );
}
