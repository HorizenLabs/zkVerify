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

use super::*;
use crate::utils::{
    make_msm_args, make_pairing_args, make_scalar_args, make_scalar_args_projective,
};
#[allow(unused)]
use crate::Pallet as Template;
use codec::Encode;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

// Min number of elements for multi scalar multiplication
const MSM_LEN_MIN: u32 = 10;
// Max number of elements for multi scalar multiplication
const MSM_LEN_MAX: u32 = 100;

// Scalar min words for single scalar multiplication (1 = 64 bit)
const SCALAR_WORDS_MIN: u32 = 1;
// Scalar max words for single scalar multiplication (16 = 1024 bit)
const SCALAR_WORDS_MAX: u32 = 16;

#[benchmarks]
mod benchmarks {
    use super::*;

    // ---------------------------------------------
    // Benchmarks for bn254
    // ---------------------------------------------

    #[benchmark]
    fn bn254_pairing_opt() {
        let (a, b) = make_pairing_args::<native::G1Affine, native::G2Affine>();

        #[extrinsic_call]
        bn254_pairing_opt(RawOrigin::None, a, b);
    }

    #[benchmark]
    fn bn254_msm_g1_opt(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) = make_msm_args::<native::G1Projective>(x);

        #[extrinsic_call]
        bn254_msm_g1_opt(RawOrigin::None, bases.encode(), scalars.encode());
    }

    #[benchmark]
    fn bn254_msm_g2_opt(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) = make_msm_args::<native::G2Projective>(x);

        #[extrinsic_call]
        bn254_msm_g2_opt(RawOrigin::None, bases.encode(), scalars.encode());
    }

    #[benchmark]
    fn bn254_mul_projective_g1_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args_projective::<native::G1Projective>(x);

        #[extrinsic_call]
        bn254_mul_projective_g1_opt(RawOrigin::None, base.encode(), scalar.encode());
    }

    #[benchmark]
    fn bn254_mul_affine_g1_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args::<native::G1Affine>(x);

        #[extrinsic_call]
        bn254_mul_affine_g1_opt(RawOrigin::None, base.encode(), scalar.encode());
    }

    #[benchmark]
    fn bn254_mul_projective_g2_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args_projective::<native::G2Projective>(x);

        #[extrinsic_call]
        bn254_mul_projective_g2_opt(RawOrigin::None, base.encode(), scalar.encode());
    }

    #[benchmark]
    fn bn254_mul_affine_g2_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args::<native::G2Affine>(x);

        #[extrinsic_call]
        bn254_mul_affine_g2_opt(RawOrigin::None, base.encode(), scalar.encode());
    }
}
