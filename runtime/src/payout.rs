use super::*;
pub use sp_runtime::{Percent, Perquintill};

fn abs(v: f64) -> f64 {
    if v > 0f64 {
        v
    } else {
        -v
    }
}

fn exp(p: f64) -> f64 {
    let mut res: f64 = 0f64;
    // Sums: 1 + x + x^2/2! + x^3/3! + x^4/4! + x^5/5! +
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
        (exp(p) * (0.01f64 / ExpPrecision::get())) as u64,
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
/// s_t: target inflation
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
    pub const K: i128 = 1i128;
    /// Multiplier (C).
    pub const C: u128 = 1u128;
    /// Percentage of the minted tokens that goes to the validators.
    pub ValidatorsSplit: Percent = Percent::from_percent(100);
    /// Percentage of the minted tokens that goes to the rest (i.e. treasury).
    pub OthersSplit: Percent = Percent::from_percent(0);
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

        const MILLISECS_PER_YEAR: u64 = 1000 * 60 * 60 * 24 * (36525 / 100);
        let time_portion = Perquintill::from_rational(era_duration_millis, MILLISECS_PER_YEAR);

        let staking_current: Perbill = Perbill::from_rational(total_staked, total_issuance);
        let s = StakingTarget::get()
            .saturating_reciprocal_mul(staking_current * (1f64 / ExpPrecision::get()) as u128);
        let exp_arg = ((1f64 / ExpPrecision::get()) as i128 - s as i128) * K::get();
        let inflation_var = to_inflation(exp_arg as f64 / (1f64 / ExpPrecision::get()));

        let inflation_tot: Balance =
            C::get() * (time_portion * (InflationBase::get() + inflation_var) * total_issuance);

        (
            ValidatorsSplit::get() * inflation_tot,
            OthersSplit::get() * inflation_tot,
        )
    }
}

/// This module tests the correct computation of rewards for validators.
#[cfg(test)]
use pallet_staking::EraPayout;

#[cfg(test)]
use crate::{Balance, Perbill, Runtime, ACME, MILLICENTS};

#[test]
fn check_params_sanity() {
    // staking_target should be at least 0.5 (and so 0 <= s_c / s_t <= 2)
    assert!(StakingTarget::get() * 10u16 >= 5u16);
    // base inflation is 5%
    assert!(InflationBase::get() * 100u64 == 5u64);
    // The sum of the splits is exactly 100%
    assert!(ValidatorsSplit::get() * 100u16 + OthersSplit::get() * 100u16 == 100u16);
}

#[test]
fn check_exp() {
    const TEST_VALUES: [f64; 5] = [0f64, 1f64, 0.5f64, -0.5f64, -1f64];
    for v in TEST_VALUES {
        assert!((exp(v) - v.exp()).abs() <= ExpPrecision::get());
    }
}

#[test]
fn check_era_rewards() {
    const ERA_DURATION_MILLIS: u64 = 1000 * 60 * 60 * 24 * (36525 / 100); // 1 year era
    const TOT_ISSUANCE: Balance = 1_000_000_000 * ACME;

    // Check the reward for an empty era.
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(0, 0, ERA_DURATION_MILLIS),
        (0, 0)
    );

    // Check the reward for a normal era, s_c == s_t
    let tot_issuance: Balance = TOT_ISSUANCE;
    let tot_staked: Balance = Perbill::from_percent(50) * tot_issuance;
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(
            tot_staked,
            tot_issuance,
            ERA_DURATION_MILLIS
        ),
        (60000000 * ACME, 0)
    );

    // Check the reward for a normal era, s_c == s_t / 2
    let tot_issuance: Balance = TOT_ISSUANCE;
    let tot_staked: Balance = Perbill::from_percent(25) * tot_issuance;
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(
            tot_staked,
            tot_issuance,
            ERA_DURATION_MILLIS
        ),
        (66487212 * ACME + 70700 * MILLICENTS, 0)
    );

    // Check the reward for a normal era, s_c == 0.0 (min)
    let tot_issuance: Balance = TOT_ISSUANCE;
    let tot_staked: Balance = Perbill::from_percent(0) * tot_issuance;
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(
            tot_staked,
            tot_issuance,
            ERA_DURATION_MILLIS
        ),
        (77182818 * ACME + 28459 * MILLICENTS, 0)
    );

    // Check the reward for a normal era, s_c == 1.0 (max)
    let tot_issuance: Balance = TOT_ISSUANCE;
    let tot_staked: Balance = Perbill::from_percent(100) * tot_issuance;
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(
            tot_staked,
            tot_issuance,
            ERA_DURATION_MILLIS
        ),
        (53678794 * ACME + 41171 * MILLICENTS, 0)
    );

    // Check the reward for an era with half the duration, s_c == 1.0 (max)
    let tot_issuance: Balance = TOT_ISSUANCE;
    let tot_staked: Balance = Perbill::from_percent(100) * tot_issuance;
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(
            tot_staked,
            tot_issuance,
            ERA_DURATION_MILLIS / 2
        ),
        ((53678794 * ACME + 41171 * MILLICENTS) / 2, 0)
    );

    // Check the reward for an era with double the duration, s_c == 1.0 (max)
    let tot_issuance: Balance = TOT_ISSUANCE;
    let tot_staked: Balance = Perbill::from_percent(100) * tot_issuance;
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(
            tot_staked,
            tot_issuance,
            ERA_DURATION_MILLIS * 2
        ),
        (53678794 * ACME + 41171 * MILLICENTS, 0) // capped at 1year
    );

    // Check the reward for an era with zero duration, s_c == 1.0 (max)
    let tot_issuance: Balance = TOT_ISSUANCE;
    let tot_staked: Balance = Perbill::from_percent(100) * tot_issuance;
    assert_eq!(
        <Runtime as pallet_staking::Config>::EraPayout::era_payout(tot_staked, tot_issuance, 0),
        (0, 0)
    );
}
