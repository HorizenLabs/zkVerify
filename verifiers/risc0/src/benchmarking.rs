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
use pallet_verifiers::{utils::funded_account, VkEntry, VkOrHash, Vks};

pub struct Pallet<T: Config>(crate::Pallet<T>);

pub trait Config: crate::Config {}
impl<T: crate::Config> Config for T {}
pub type Call<T> = pallet_verifiers::Call<T, Risc0<T>>;

include!("resources_benchmarking/vk_pubs.rs");

#[benchmarks(where T: pallet_verifiers::Config<Risc0<T>>)]
mod benchmarks {

    use super::*;

    #[benchmark]
    fn submit_proof_cycle_2_pow_12() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_12.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_12.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_13() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_13.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_13.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_14() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_14.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_14.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_15() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_15.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_15.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_16() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_16.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_16.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_17() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_17.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_17.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_18() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_18.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_18.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_19() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_19.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_19.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_20() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_20.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_20.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_21() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_21.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_21.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_22() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_22.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_22.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_23() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_23.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_23.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_cycle_2_pow_24() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_vk(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_24.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_24.to_vec().into();

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_12() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_12.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_12.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_13() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_13.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_13.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_14() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_14.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_14.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_15() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_15.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_15.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_16() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_16.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_16.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_17() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_17.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_17.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_18() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_18.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_18.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_19() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_19.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_19.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_20() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_20.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_20.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_21() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_21.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_21.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_22() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_22.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_22.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_23() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_23.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_23.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn submit_proof_with_vk_hash_cycle_2_pow_24() {
        let caller = whitelisted_caller();
        let vk = VkOrHash::from_hash(VALID_VK);
        let proof = include_bytes!("resources_benchmarking/VALID_PROOF_CYCLE_2_POW_24.bin")
            .to_vec()
            .into();
        let pubs = VALID_PUBS_CYCLE_2_POW_24.to_vec().into();
        let vk_entry = VkEntry::new(VALID_VK);
        Vks::<T, Risc0<T>>::insert(VALID_VK, vk_entry);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), vk, proof, pubs);
    }

    #[benchmark]
    fn register_vk() {
        let caller: T::AccountId = funded_account::<T, Risc0<T>>();
        let vk = VALID_VK.into();

        #[extrinsic_call]
        register_vk(RawOrigin::Signed(caller), vk);

        // Verify
        assert!(Vks::<T, Risc0<T>>::get(Risc0::<T>::vk_hash(&VALID_VK)).is_some());
    }

    impl_benchmark_test_suite!(Pallet, super::mock::test_ext(), super::mock::Test);
}

#[cfg(test)]
mod mock {
    use frame_support::{
        derive_impl, parameter_types,
        sp_runtime::{traits::IdentityLookup, BuildStorage},
        traits::{fungible::HoldConsideration, LinearStoragePrice},
    };
    use sp_core::{ConstU128, ConstU32};

    type Balance = u128;
    type AccountId = u64;

    // Configure a mock runtime to test the pallet.
    frame_support::construct_runtime!(
        pub enum Test
        {
            System: frame_system,
            Balances: pallet_balances,
            CommonVerifiersPallet: pallet_verifiers::common,
            VerifierPallet: crate,
        }
    );

    parameter_types! {
        pub const Risc0MaxProofSize: u32 = 2455714; // 2455714: risc0 proof size for a 2^24 cycle-count run
        pub const Risc0MaxPubsSize: u32 = 8 + 4 + 32 * 64; // 8: for bincode::serialize,
                                                           // 4: bytes for payload length,
                                                           // 32 * 64: sufficient multiple of 32 bytes
    }

    impl crate::Config for Test {
        type MaxProofSize = Risc0MaxProofSize;
        type MaxPubsSize = Risc0MaxPubsSize;
    }

    #[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
    impl frame_system::Config for Test {
        type Block = frame_system::mocking::MockBlockU32<Test>;
        type AccountId = AccountId;
        type Lookup = IdentityLookup<Self::AccountId>;
        type AccountData = pallet_balances::AccountData<Balance>;
    }

    parameter_types! {
        pub const BaseDeposit: Balance = 1;
        pub const PerByteDeposit: Balance = 2;
        pub const HoldReasonVkRegistration: RuntimeHoldReason = RuntimeHoldReason::CommonVerifiersPallet(pallet_verifiers::common::HoldReason::VkRegistration);
    }

    impl pallet_verifiers::Config<crate::Risc0<Test>> for Test {
        type RuntimeEvent = RuntimeEvent;
        type OnProofVerified = ();
        type WeightInfo = crate::Risc0Weight<()>;
        type Ticket = HoldConsideration<
            AccountId,
            Balances,
            HoldReasonVkRegistration,
            LinearStoragePrice<BaseDeposit, PerByteDeposit, Balance>,
        >;
        type Currency = Balances;
    }

    impl pallet_balances::Config for Test {
        type MaxLocks = ConstU32<50>;
        type MaxReserves = ();
        type ReserveIdentifier = [u8; 8];
        /// The type for recording an account's balance.
        type Balance = Balance;
        /// The ubiquitous event type.
        type RuntimeEvent = RuntimeEvent;
        type DustRemoval = ();
        type ExistentialDeposit = ConstU128<1>;
        type AccountStore = System;
        type WeightInfo = ();
        type FreezeIdentifier = ();
        type MaxFreezes = ();
        type RuntimeHoldReason = RuntimeHoldReason;
        type RuntimeFreezeReason = ();
    }

    impl pallet_verifiers::common::Config for Test {
        type CommonWeightInfo = Test;
    }

    /// Build genesis storage according to the mock runtime.
    pub fn test_ext() -> sp_io::TestExternalities {
        let mut ext = sp_io::TestExternalities::from(
            frame_system::GenesisConfig::<Test>::default()
                .build_storage()
                .unwrap(),
        );
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
