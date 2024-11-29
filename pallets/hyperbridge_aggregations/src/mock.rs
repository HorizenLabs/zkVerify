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

#![cfg(test)]

use super::*;
use frame_support::pallet_prelude::ConstU32;
use frame_support::traits::ConstU128;
use frame_support::weights::RuntimeDbWeight;
use frame_support::{
    construct_runtime, derive_impl, parameter_types,
    traits::{ConstU16, ConstU64},
};
use ismp::host::StateMachine;
use ismp::router::IsmpRouter;
use pallet_ismp::NoOpMmrTree;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

pub type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;
pub type AccountId = u64;

parameter_types! {
    pub const MockDbWeight: RuntimeDbWeight = RuntimeDbWeight {
        read: 4_200_000,
        write: 2_400_000,
    };
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = frame_system::mocking::MockBlockU32<Test>;
    type AccountData = pallet_balances::AccountData<Balance>;
    type DbWeight = MockDbWeight;
}

impl pallet_timestamp::Config for Test {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
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

impl ismp_grandpa::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type IsmpHost = Ismp;
}

parameter_types! {
    pub const Coprocessor: Option<StateMachine> = Some(StateMachine::Kusama(4009));
    pub const HostStateMachine: StateMachine = StateMachine::Substrate(*b"zkv_");
}

impl pallet_ismp::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AdminOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type HostStateMachine = HostStateMachine;
    type TimestampProvider = Timestamp;
    type Currency = Balances;
    type Balance = Balance;
    type Router = ModuleRouter;
    type Coprocessor = Coprocessor;
    type ConsensusClients = (ismp_grandpa::consensus::GrandpaConsensusClient<Test>,);
    type Mmr = NoOpMmrTree<Test>;
    type WeightProvider = ();
}

#[derive(Default)]
pub struct ModuleRouter;
impl IsmpRouter for ModuleRouter {
    fn module_for_id(&self, id: Vec<u8>) -> Result<Box<dyn IsmpModule>, anyhow::Error> {
        match id.as_slice() {
            id if id == ZKV_MODULE_ID.to_bytes().as_slice() => {
                Ok(Box::new(crate::Pallet::<Test>::default()))
            }
            _ => Err(ismp::Error::ModuleNotFound(id))?,
        }
    }
}

pub struct MockWeightInfo;

impl MockWeightInfo {
    pub const REF_TIME: u64 = 42;
    pub const PROOF_SIZE: u64 = 24;
}

impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type IsmpDispatcher = pallet_ismp::Pallet<Test>;
    type WeightInfo = MockWeightInfo;
}

// Configure a mock runtime to test the pallet.
construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        HyperbridgeAggregations: crate,
        Timestamp: pallet_timestamp,
        Ismp: pallet_ismp,
        Balances: pallet_balances,
    }
);

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
