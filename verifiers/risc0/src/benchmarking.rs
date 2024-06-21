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

use super::Risc0;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use hp_verifiers::Verifier;
use pallet_verifiers::{VkOrHash, Vks};

pub struct Pallet<T: Config>(crate::Pallet<T>);

pub trait Config: crate::Config {}
impl<T: crate::Config> Config for T {}
pub type Call<T> = pallet_verifiers::Call<T, Risc0<T>>;

include!("resources.rs");

#[benchmarks(where T: pallet_verifiers::Config<Risc0<T>>)]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn submit_proof() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = VALID_PROOF.to_vec().into();
        let pubs = VALID_PUBS.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = VALID_PROOF.to_vec().into();
        let pubs = VALID_PUBS.to_vec().into();
        Vks::<T, Risc0<T>>::insert(VALID_VK, VALID_VK);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn register_vk() {
        let caller = whitelisted_caller();
        let vk = VALID_VK.into();

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk);

        // Verify
        assert!(Vks::<T, Risc0<T>>::get(Risc0::<T>::vk_hash(&VALID_VK)).is_some());
    }
}
