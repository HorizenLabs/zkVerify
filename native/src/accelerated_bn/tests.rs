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

use crate::bn254::{g1, g2};
use crate::{
    bn254::Bn254 as Bn254Opt, bn254::Fr as FrOpt, bn254::G1Affine as G1AffineOpt,
    bn254::G1Projective as G1ProjectiveOpt, bn254::G2Affine as G2AffineOpt,
    bn254::G2Projective as G2ProjectiveOpt,
};
use ark_bn254::{G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, CurveGroup, Group};
use ark_ff::{PrimeField, Zero};

#[test]
pub fn test_pairing_opt() {
    // Compute e(g, h) using host function
    let g = G1ProjectiveOpt::generator();
    let h = G2ProjectiveOpt::generator();
    let lhs = Bn254Opt::multi_pairing([g], [h]).0;

    // Compute e(g, h) using Arkworks
    let g = ark_bn254::G1Projective::generator();
    let h = ark_bn254::G2Projective::generator();
    let rhs = ark_bn254::Bn254::multi_pairing([g], [h]).0;

    assert_eq!(lhs, rhs);
}

#[test]
pub fn msm_g1_opt() {
    // Some numbers that will serve as scalars
    let nums = vec![1_u64, 2_u64, 3_u64, 4_u64, 5_u64];

    // Compute MSM by using a host function

    // Convert numbers to scalars
    let scalars: Vec<FrOpt> = nums.clone().into_iter().map(FrOpt::from).collect();

    // Define fixed points in G1
    let points = vec![G1AffineOpt::generator(); 5];

    let lhs = <g1::Config as SWCurveConfig>::msm(&points, &scalars)
        .map(|result| result.into_affine())
        .map(|affine| {
            let x = affine.x.into_bigint();
            let y = affine.y.into_bigint();
            (x, y)
        });

    // Compute MSM by using Arkworks

    // Convert numbers to scalars
    let scalars: Vec<ark_bn254::Fr> = nums.into_iter().map(|x| ark_bn254::Fr::from(x)).collect();

    // Define fixed points in G1
    let points = vec![G1Affine::generator(); 5];

    let rhs = <ark_bn254::g1::Config as SWCurveConfig>::msm(&points, &scalars)
        .map(|result| result.into_affine())
        .map(|affine| {
            let x = affine.x.into_bigint();
            let y = affine.y.into_bigint();
            (x, y)
        });

    assert_eq!(lhs, rhs);
}

#[test]
pub fn msm_g2_opt() {
    // Some numbers that will serve as scalars
    let nums = vec![1_u64, 2_u64, 3_u64, 4_u64, 5_u64];

    // Compute MSM by using a host function

    // Convert numbers to scalars
    let scalars: Vec<FrOpt> = nums.clone().into_iter().map(FrOpt::from).collect();

    // Define fixed points in G1
    let points = vec![G2AffineOpt::generator(); 5];

    let lhs = <g2::Config as SWCurveConfig>::msm(&points, &scalars)
        .map(|result| result.into_affine())
        .map(|affine| {
            let x_c0 = affine.x.c0.into_bigint();
            let x_c1 = affine.x.c1.into_bigint();
            let y_c0 = affine.y.c0.into_bigint();
            let y_c1 = affine.y.c1.into_bigint();
            ((x_c0, x_c1), (y_c0, y_c1))
        });

    // Compute MSM by using Arkworks

    // Convert numbers to scalars
    let scalars: Vec<ark_bn254::Fr> = nums.into_iter().map(|x| ark_bn254::Fr::from(x)).collect();

    // Define fixed points in G1
    let points = vec![G2Affine::generator(); 5];

    let rhs = <ark_bn254::g2::Config as SWCurveConfig>::msm(&points, &scalars)
        .map(|result| result.into_affine())
        .map(|affine| {
            let x_c0 = affine.x.c0.into_bigint();
            let x_c1 = affine.x.c1.into_bigint();
            let y_c0 = affine.y.c0.into_bigint();
            let y_c1 = affine.y.c1.into_bigint();
            ((x_c0, x_c1), (y_c0, y_c1))
        });

    assert_eq!(lhs, rhs);
}

#[test]
pub fn mul_projective_g1_opt() {
    let scalar: u64 = 123456789;

    // Compute point multiplication by using a host function

    let point = G1ProjectiveOpt::generator();

    let lhs = {
        let res = <g1::Config as SWCurveConfig>::mul_projective(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x = affine.x.into_bigint();
            let y = affine.y.into_bigint();
            Some((x, y))
        }
    };

    // Compute point multiplication by using Arkworks

    let point = G1Projective::generator();

    let rhs = {
        let res = <ark_bn254::g1::Config as SWCurveConfig>::mul_projective(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x = affine.x.into_bigint();
            let y = affine.y.into_bigint();
            Some((x, y))
        }
    };

    assert_eq!(lhs, rhs);
}

#[test]
pub fn mul_affine_g1_opt() {
    let scalar: u64 = 123456789;

    // Compute point multiplication by using a host function

    let point = G1AffineOpt::generator();

    let lhs = {
        let res = <g1::Config as SWCurveConfig>::mul_affine(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x = affine.x.into_bigint();
            let y = affine.y.into_bigint();
            Some((x, y))
        }
    };

    // Compute point multiplication by using Arkworks

    let point = G1Affine::generator();

    let rhs = {
        let res = <ark_bn254::g1::Config as SWCurveConfig>::mul_affine(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x = affine.x.into_bigint();
            let y = affine.y.into_bigint();
            Some((x, y))
        }
    };

    assert_eq!(lhs, rhs);
}

#[test]
pub fn mul_projective_g2_opt() {
    let scalar: u64 = 123456789;

    // Compute point multiplication by using a host function

    let point = G2ProjectiveOpt::generator();

    let lhs = {
        let res = <g2::Config as SWCurveConfig>::mul_projective(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x_c0 = affine.x.c0.into_bigint();
            let x_c1 = affine.x.c1.into_bigint();
            let y_c0 = affine.y.c0.into_bigint();
            let y_c1 = affine.y.c1.into_bigint();
            Some(((x_c0, x_c1), (y_c0, y_c1)))
        }
    };

    // Compute point multiplication by using Arkworks

    let point = G2Projective::generator();

    let rhs = {
        let res = <ark_bn254::g2::Config as SWCurveConfig>::mul_projective(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x_c0 = affine.x.c0.into_bigint();
            let x_c1 = affine.x.c1.into_bigint();
            let y_c0 = affine.y.c0.into_bigint();
            let y_c1 = affine.y.c1.into_bigint();
            Some(((x_c0, x_c1), (y_c0, y_c1)))
        }
    };

    assert_eq!(lhs, rhs);
}

#[test]
pub fn mul_affine_g2_opt() {
    let scalar: u64 = 123456789;

    // Compute point multiplication by using a host function

    let point = G2AffineOpt::generator();

    let lhs = {
        let res = <g2::Config as SWCurveConfig>::mul_affine(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x_c0 = affine.x.c0.into_bigint();
            let x_c1 = affine.x.c1.into_bigint();
            let y_c0 = affine.y.c0.into_bigint();
            let y_c1 = affine.y.c1.into_bigint();
            Some(((x_c0, x_c1), (y_c0, y_c1)))
        }
    };

    // Compute point multiplication by using Arkworks

    let point = G2Affine::generator();

    let rhs = {
        let res = <ark_bn254::g2::Config as SWCurveConfig>::mul_affine(&point, &[scalar]);
        if res.is_zero() {
            None
        } else {
            let affine = res.into_affine();
            let x_c0 = affine.x.c0.into_bigint();
            let x_c1 = affine.x.c1.into_bigint();
            let y_c0 = affine.y.c0.into_bigint();
            let y_c1 = affine.y.c1.into_bigint();
            Some(((x_c0, x_c1), (y_c0, y_c1)))
        }
    };

    assert_eq!(lhs, rhs);
}
