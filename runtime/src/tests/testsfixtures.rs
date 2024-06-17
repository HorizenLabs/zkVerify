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

use crate::{currency, Balance, EXISTENTIAL_DEPOSIT};
// Existential deposit used in pallet_balances
pub const EXISTENTIAL_DEPOSIT_REMAINDER: Balance = 1;
pub const NUM_TEST_ACCOUNTS: u32 = 4;
pub const STASH_DEPOSIT: Balance = 500; // MUST not be smaller than EXISTENTIAL_DEPOSIT

#[derive(Clone)]
pub struct SampleAccount {
    pub raw_account: [u8; 32],
    pub starting_balance: Balance,
    pub session_key_seed: u8,
}

// Build a vector containing a few sample user accounts, along with their starting balances
pub static SAMPLE_USERS: [SampleAccount; NUM_TEST_ACCOUNTS as usize] = [
    SampleAccount {
        raw_account: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ],
        starting_balance: 1000001 * currency::ACME,
        session_key_seed: 1,
    },
    SampleAccount {
        raw_account: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2,
        ],
        starting_balance: 12345432 * currency::ACME,
        session_key_seed: 2,
    },
    SampleAccount {
        raw_account: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 3,
        ],
        starting_balance: 9955223 * currency::ACME,
        session_key_seed: 3,
    },
    SampleAccount {
        raw_account: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 4,
        ],
        starting_balance: EXISTENTIAL_DEPOSIT,
        session_key_seed: 4,
    },
];
