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

//! Generic executions of the operations for *Arkworks* elliptic curves.

// As not all functions are used by each elliptic curve and some elliptic
// curve may be excluded by the build we resort to `#[allow(unused)]` to
// suppress the expected warning.

use frame_support::derive_impl;
use frame_system::mocking::MockBlock;
use sp_runtime::BuildStorage;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        AccBnCrypto: crate,
    }
);

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type Block = MockBlock<Test>;
}

impl crate::Config for Test {
    type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
