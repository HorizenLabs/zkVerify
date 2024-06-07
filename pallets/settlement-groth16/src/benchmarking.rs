use super::*;

use crate::groth16::{Curve, Groth16};
#[allow(unused)]
use crate::Pallet as SettlementGroth16Pallet;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    const MAX_NUM_INPUTS: u32 = T::MaxNumInputs::get();

    #[benchmark]
    fn submit_proof_bn254(n: Linear<0, MAX_NUM_INPUTS>) {
        // setup code
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16::get_instance(n as usize, None, Curve::Bn254);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), proof, vk, inputs);
    }

    #[benchmark]
    fn submit_proof_bls12_381(n: Linear<0, MAX_NUM_INPUTS>) {
        // setup code
        let caller = whitelisted_caller();
        let (proof, vk, inputs) = Groth16::get_instance(n as usize, None, Curve::Bls12_381);

        #[extrinsic_call]
        submit_proof(RawOrigin::Signed(caller), proof, vk, inputs);
    }

    impl_benchmark_test_suite!(
        SettlementGroth16Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
