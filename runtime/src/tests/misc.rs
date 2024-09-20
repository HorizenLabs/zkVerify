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

//! Here we write miscellaneous tests that don't fall in the other categories.

use frame_support::traits::ExistenceRequirement;

use super::*;

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn check_starting_balances_and_existential_limit() {
    test().execute_with(|| {
        use frame_support::traits::{fungible::Inspect, Currency};
        // This creates a few public keys used to be converted to AccountId

        for sample_user in &testsfixtures::SAMPLE_USERS {
            assert_eq!(
                Balances::balance(&sample_user.raw_account.into()),
                sample_user.starting_balance
            );
        }

        // Now perform a withdraw on the fourth account, leaving its balance under the EXISTENTIAL_DEPOSIT limit
        // This should kill the account, when executed with the ExistenceRequirement::AllowDeath option
        let _id_3_withdraw = Balances::withdraw(
            &testsfixtures::SAMPLE_USERS[3].raw_account.into(),
            testsfixtures::EXISTENTIAL_DEPOSIT_REMAINDER, // Withdrawing more th
            WithdrawReasons::TIP,
            ExistenceRequirement::AllowDeath,
        );

        // Verify that the fourth account balance is now 0
        assert_eq!(
            Balances::balance(&testsfixtures::SAMPLE_USERS[3].raw_account.into()),
            0
        );
    });
}
