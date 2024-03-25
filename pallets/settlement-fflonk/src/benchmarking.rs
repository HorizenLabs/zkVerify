//! Benchmarking setup for pallet-settlement-fflonk
use super::*;

#[allow(unused)]
use crate::Pallet as SettlementFFlonkPallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

include!("proof.rs");

benchmarks! {
    submit_proof {
        // we can use a single input rather than complexity parameters
        // (https://paritytech.github.io/polkadot-sdk/master/frame_benchmarking/macro.benchmarks.html),
        // as the `submit_proof` extrinsic should have O(1) complexity

        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), VALID_PROOF.into())
}

impl_benchmark_test_suite!(
    SettlementFFlonkPallet,
    crate::mock::new_test_ext(),
    crate::mock::Test,
);
