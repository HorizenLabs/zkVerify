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

//! This module tests the correct computation of rewards for validators.

use pallet_staking::EraPayout;

use crate::{Balance, Runtime};

use super::test;

#[test]
fn check_era_rewards() {
    test().execute_with(|| {
        const ERA_DURATION_MILLIS: u64 = 6 * 60 * 60 * 1000;
        const TOTAL_STAKED: Balance = 900000000;
        const TOTAL_ISSUANCE: Balance = 1000000000;

        // Check the reward for an empty era.
        assert_eq!(
            <Runtime as pallet_staking::Config>::EraPayout::era_payout(0, 0, ERA_DURATION_MILLIS),
            (0u128, 0)
        );

        // Check the reward for a normal era
        assert_eq!(
            <Runtime as pallet_staking::Config>::EraPayout::era_payout(
                TOTAL_STAKED,
                TOTAL_ISSUANCE,
                ERA_DURATION_MILLIS
            ),
            (17313, 51133)
        );
    });
}
