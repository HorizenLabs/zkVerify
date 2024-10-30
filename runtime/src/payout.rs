// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module contains a custom implementation for the pallet_staking::EraPayout trait, which
//! provides a custom inflation model which tries to drive the staking rate toward an ideal target.
//! Details on the actual formula and its parameters are given below.

use super::*;
pub use sp_runtime::{Percent, Perquintill};

fn abs(v: f64) -> f64 {
    if v > 0f64 {
        v
    } else {
        -v
    }
}

// Sums: 1 + x + x^2/2! + x^3/3! + x^4/4! + x^5/5! + ...
// until the error goes below ExpPrecision.
// If |p| <= 1, the number of terms (iterations) is limited by n s.t. 1/n! <= ExpPrecision.
// For example, if ExpPrecision == 10e-15 => n == 18
fn exp(p: f64) -> f64 {
    let mut res: f64 = 0f64;
    let mut next = 1f64;
    let mut i = 1f64;
    while abs(next) > ExpPrecision::get() {
        res += next;
        next = (next * p) / i;
        i += 1f64;
    }
    res
}

fn to_inflation(p: f64) -> Perquintill {
    Perquintill::from_rational(
        (p * (0.01f64 / ExpPrecision::get())) as u64,
        (1f64 / ExpPrecision::get()) as u64,
    )
}

/// Implements a custom inflation model based on the following formula:
/// I(s_c) = I_b + I_v(s_c)
///
/// with:
/// I_v(x) = C * exp(K * (1 - (s_c / s_t)))
///
/// where:
/// I_b: base inflation
/// I_v: variable inflation
/// s_t: target staking rate
/// K: sensitivity coefficient
/// C: multiplier
pub struct ZKVPayout;

parameter_types! {
    /// The target precions for exp(); impacts on the final precision for inflation computation.
    pub const ExpPrecision: f64 = 10e-15f64;
    /// Base inflation (I_b).
    pub InflationBase: Perquintill = Perquintill::from_percent(5);
    /// The optimal staking rate (s_t).
    pub StakingTarget: Percent = Percent::from_percent(50);
    /// Sensitivity coefficient (K).
    pub const K: f64 = 1f64;
    /// Multiplier (C).
    pub const C: f64 = 1f64;
    /// Percentage of the minted tokens that goes to the validators (leaving the rest to the
    /// others).
    pub ValidatorsSplit: Percent = Percent::from_percent(50);
}

impl pallet_staking::EraPayout<Balance> for ZKVPayout {
    /// Calculates the validators reward based on the duration of the era.
    fn era_payout(
        total_staked: Balance,
        total_issuance: Balance,
        era_duration_millis: u64,
    ) -> (Balance, Balance) {
        if total_issuance == 0 {
            return (0, 0);
        }

        const MILLISECS_PER_YEAR: u64 = 1000 * 60 * 60 * 24 * 36525 / 100;
        const SCALE: f64 = 1f64 / ExpPrecision::get();
        let time_portion = Perquintill::from_rational(era_duration_millis, MILLISECS_PER_YEAR);

        let staking_current: Perbill = Perbill::from_rational(total_staked, total_issuance);

        // s = s_c / s_t
        let s = StakingTarget::get().saturating_reciprocal_mul(staking_current * SCALE as u128);

        // exp_arg = k * (1 - s)
        let exp_arg = (SCALE - s as f64) * K::get() / SCALE;

        // inflation_arg = C * e^(exp_arg)
        let inflation_arg = C::get() * exp(exp_arg);

        let inflation_var = to_inflation(inflation_arg);

        let inflation_tot: Balance =
            time_portion * (InflationBase::get() + inflation_var) * total_issuance;

        (
            ValidatorsSplit::get() * inflation_tot,
            (Percent::from_percent(100) - ValidatorsSplit::get()) * inflation_tot,
        )
    }
}

#[test]
fn check_exp() {
    const TEST_VALUES: [f64; 9] = [
        -K::get(),
        -1f64,
        -0.5f64,
        -ExpPrecision::get(),
        0f64,
        ExpPrecision::get(),
        0.5f64,
        1f64,
        K::get(),
    ];

    for v in TEST_VALUES {
        assert!(
            (exp(v) - v.exp()).abs() <= ExpPrecision::get(),
            "failed for {}",
            v
        );
    }
}
