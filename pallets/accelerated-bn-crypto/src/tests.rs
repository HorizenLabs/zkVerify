use crate::{
    // benchmarking::{C_SERIALIZED, PROOF_SERIALIZED, VK_SERIALIZED},
    mock::*,
    utils::{
        make_msm_args,
        make_pairing_args,
        make_scalar_args,
        make_scalar_args_projective,
        // serialize_argument,
    },
};
// use ark_ff::{Fp, MontBackend};
// use ark_serialize::CanonicalDeserialize;
use codec::Encode;
use frame_support::assert_ok;

const SCALAR_WORDS: u32 = 3;
const MSM_LEN: u32 = 10;

// ---------------------------------------------
// Tests for BN254
// ---------------------------------------------

#[test]
fn bn254_pairing_opt() {
    let (a, b) = make_pairing_args::<native::G1Affine, native::G2Affine>();

    new_test_ext().execute_with(|| {
        assert_ok!(AccBnCrypto::bn254_pairing_opt(RuntimeOrigin::none(), a, b));
    });
}

#[test]
fn bn254_msm_g1_opt() {
    let (bases, scalars) = make_msm_args::<native::G1Projective>(MSM_LEN);

    new_test_ext().execute_with(|| {
        assert_ok!(AccBnCrypto::bn254_msm_g1_opt(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}

#[test]
fn bn254_msm_g2_opt() {
    let (bases, scalars) = make_msm_args::<native::G2Projective>(MSM_LEN);

    new_test_ext().execute_with(|| {
        assert_ok!(AccBnCrypto::bn254_msm_g2_opt(
            RuntimeOrigin::none(),
            bases.encode(),
            scalars.encode()
        ));
    });
}

#[test]
fn bn254_mul_projective_g1_opt() {
    new_test_ext().execute_with(|| {
        let (base, scalar) = make_scalar_args_projective::<native::G1Projective>(SCALAR_WORDS);

        assert_ok!(AccBnCrypto::bn254_mul_projective_g1_opt(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}

#[test]
fn bn254_mul_affine_g1_opt() {
    let (base, scalar) = make_scalar_args::<native::G1Affine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(AccBnCrypto::bn254_mul_affine_g1_opt(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}

#[test]
fn bn254_mul_projective_g2_opt() {
    let (base, scalar) = make_scalar_args_projective::<native::G2Projective>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(AccBnCrypto::bn254_mul_projective_g2_opt(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}

#[test]
fn bn254_mul_affine_g2_opt() {
    let (base, scalar) = make_scalar_args::<native::G2Affine>(SCALAR_WORDS);

    new_test_ext().execute_with(|| {
        assert_ok!(AccBnCrypto::bn254_mul_affine_g2_opt(
            RuntimeOrigin::none(),
            base.encode(),
            scalar.encode()
        ));
    });
}
