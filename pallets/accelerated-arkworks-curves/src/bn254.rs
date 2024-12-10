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

// use crate::utils::ScalarFieldFor;
use ark_ec::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr};
use native::bn254::{g1, g2};
use native::{
    bn254::Bn254 as Bn254Opt, bn254::G1Affine as G1AffineOpt,
    bn254::G1Projective as G1ProjectiveOpt, bn254::G2Affine as G2AffineOpt,
    bn254::G2Projective as G2ProjectiveOpt,
};

#[inline]
pub fn pairing_opt(a: G1AffineOpt, b: G2AffineOpt) {
    let _out = Bn254Opt::multi_pairing([a], [b]);
}

#[inline]
pub fn msm_g1_opt(bases: &[G1AffineOpt], scalars: &[<G1AffineOpt as AffineRepr>::ScalarField]) {
    let _out = <g1::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_g2_opt(bases: &[G2AffineOpt], scalars: &[<G2AffineOpt as AffineRepr>::ScalarField]) {
    let _out = <g2::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn mul_projective_g1_opt(base: &G1ProjectiveOpt, scalar: &[u64]) {
    let _out = <g1::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_affine_g1_opt(base: &G1AffineOpt, scalar: &[u64]) {
    let _out = <g1::Config as SWCurveConfig>::mul_affine(base, scalar);
}

#[inline]
pub fn mul_projective_g2_opt(base: &G2ProjectiveOpt, scalar: &[u64]) {
    let _out = <g2::Config as SWCurveConfig>::mul_projective(base, scalar);
}

#[inline]
pub fn mul_affine_g2_opt(base: &G2AffineOpt, scalar: &[u64]) {
    let _out = <g2::Config as SWCurveConfig>::mul_affine(base, scalar);
}
