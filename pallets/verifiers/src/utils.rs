//! Some utilities for benchmarking verifiers.

#![cfg(feature = "runtime-benchmarks")]
#![cfg(not(doc))]

use crate::Config;
use frame_benchmarking::whitelisted_caller;
use frame_support::{
    sp_runtime::traits::Bounded,
    traits::fungible::{Inspect, Mutate},
};
use hp_verifiers::Verifier;

type BalanceOf<T, I> =
    <<T as Config<I>>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

/// Return a whitelisted account with enough funds to do anything.
pub fn funded_account<T: Config<I>, I: Verifier>() -> T::AccountId {
    let caller: T::AccountId = whitelisted_caller();
    T::Currency::set_balance(&caller, BalanceOf::<T, I>::max_value() / 2u32.into());
    caller
}
