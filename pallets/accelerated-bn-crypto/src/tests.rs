// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Generic executions of the operations for *Arkworks* elliptic curves.

// As not all functions are used by each elliptic curve and some elliptic
// curve may be excluded by the build we resort to `#[allow(unused)]` to
// suppress the expected warning.

use crate::{
    mock::*,
    utils::{make_msm_args, make_pairing_args, make_scalar_args, make_scalar_args_projective},
};
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
