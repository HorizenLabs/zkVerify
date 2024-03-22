use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_core::{Get, H256};

benchmarks! {
    publish_attestation {
        // benchmark with double the minimum number of elements for publishing an attestation
        let mut hash: [u8; 32] = [0; 32];
        for h in 0 .. T::MinProofsForPublishing::get() * 2 {
            hash[0] += 1;
            Values::<T>::insert(0, H256::from_slice(&hash), ());
        }
    }: _(RawOrigin::Root)
}

#[cfg(test)]
use crate::Pallet as Poe;
impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test,);
