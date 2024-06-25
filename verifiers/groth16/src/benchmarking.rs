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

use super::Groth16;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use hp_verifiers::Verifier;
use pallet_verifiers::{VkOrHash, Vks};

pub struct Pallet<T: Config>(crate::Pallet<T>);
pub trait Config: crate::Config {}
impl<T: crate::Config> Config for T {}
pub type Call<T> = pallet_verifiers::Call<T, Groth16<T>>;
use crate::groth16::{Curve, Groth16 as Groth16Circuits};

#[benchmarks(where T: pallet_verifiers::Config<Groth16<T>>)]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn submit_proof_bn254(n: Linear<0, crate::MAX_NUM_INPUTS>) {
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16Circuits::get_instance(n as usize, None, Curve::Bn254);

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_vk(vk),
            proof.into(),
            inputs.into(),
        );
    }

    #[benchmark]
    fn submit_proof_bls12_381(n: Linear<0, crate::MAX_NUM_INPUTS>) {
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16Circuits::get_instance(n as usize, None, Curve::Bls12_381);

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_vk(vk),
            proof.into(),
            inputs.into(),
        );
    }

    #[benchmark]
    fn submit_proof_bn254_with_vk_hash(n: Linear<0, crate::MAX_NUM_INPUTS>) {
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16Circuits::get_instance(n as usize, None, Curve::Bn254);
        let hash = sp_core::H256::repeat_byte(2);
        Vks::<T, Groth16<T>>::insert(hash, vk);

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_hash(hash),
            proof.into(),
            inputs.into(),
        );
    }

    #[benchmark]
    fn submit_proof_bls12_381_with_vk_hash(n: Linear<0, crate::MAX_NUM_INPUTS>) {
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16Circuits::get_instance(n as usize, None, Curve::Bls12_381);
        let hash = sp_core::H256::repeat_byte(2);
        Vks::<T, Groth16<T>>::insert(hash, vk);

        #[extrinsic_call]
        submit_proof(
            RawOrigin::Signed(caller),
            VkOrHash::from_hash(hash),
            proof.into(),
            inputs.into(),
        );
    }

    #[benchmark]
    fn register_vk_bn254(n: Linear<0, crate::MAX_NUM_INPUTS>) {
        let caller = whitelisted_caller();
        let (_, vk, _) = Groth16Circuits::get_instance(n as usize, None, Curve::Bn254);

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk.clone().into());

        // Verify
        assert!(Vks::<T, Groth16<T>>::get(Groth16::<T>::vk_hash(&vk)).is_some());
    }

    #[benchmark]
    fn register_vk_bls12_381(n: Linear<0, crate::MAX_NUM_INPUTS>) {
        let caller = whitelisted_caller();
        let (_, vk, _) = Groth16Circuits::get_instance(n as usize, None, Curve::Bls12_381);

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk.clone().into());

        // Verify
        assert!(Vks::<T, Groth16<T>>::get(Groth16::<T>::vk_hash(&vk)).is_some());
    }
}
