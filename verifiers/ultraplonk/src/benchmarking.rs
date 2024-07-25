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

#![cfg(feature = "runtime-benchmarks")]

use crate::Ultraplonk;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use hp_verifiers::Verifier;
use pallet_verifiers::{VkOrHash, Vks};

pub struct Pallet<T: Config>(crate::Pallet<T>);
pub trait Config: pallet_verifiers::Config<Ultraplonk> {}
impl<T: pallet_verifiers::Config<Ultraplonk>> Config for T {}
pub type Call<T> = pallet_verifiers::Call<T, Ultraplonk>;

include!("resources.rs");

#[benchmarks]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn submit_proof() {
        // setup code
        let caller = whitelisted_caller();
        let proof = VALID_PROOF;
        let pubs = public_input();
        let vk = VALID_VK;

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_vk(vk),
            proof.into(),
            pubs.into(),
        );
    }

    #[benchmark]
    fn submit_proof_with_vk_hash() {
        // setup code
        let caller = whitelisted_caller();
        let proof = VALID_PROOF;
        let pubs = public_input();
        let vk = VALID_VK;
        let hash = sp_core::H256::repeat_byte(2);
        Vks::<T, Ultraplonk>::insert(hash, vk);

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_hash(hash),
            proof.into(),
            pubs.into(),
        );
    }

    #[benchmark]
    fn register_vk() {
        // setup code
        let caller = whitelisted_caller();
        let vk = VALID_VK;

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk.clone().into());

        // Verify
        assert!(Vks::<T, Ultraplonk>::get(Ultraplonk::vk_hash(&vk)).is_some());
    }

    // impl_benchmark_test_suite!(Pallet, super::mock::test_ext(), super::mock::Test);
}
