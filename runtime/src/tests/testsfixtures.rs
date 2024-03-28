 // Copyright 2024, The Horizen Foundation
 
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

// Existential deposit used in pallet_balances
pub const EXISTENTIAL_DEPOSIT: u128 = 500;
pub const EXISTENTIAL_DEPOSIT_REMINDER: u128 = 1;

pub struct SampleAccount {
    pub raw_account: [u8; 32],
    pub starting_balance: u128,
}

// Build a vector containing a few sample user accounts, along with their starting balances
pub fn get_sample_users() -> Vec<SampleAccount> {
    vec![
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            starting_balance: 1000001,
        },
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 2,
            ],
            starting_balance: 12345432,
        },
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 3,
            ],
            starting_balance: 9955223,
        },
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 4,
            ],
            starting_balance: EXISTENTIAL_DEPOSIT,
        },
    ]
}
