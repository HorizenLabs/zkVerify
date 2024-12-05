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

//! *BN254* types and host functions.

use super::BnCryptoError;
use crate::accelerated_bn::utils;
use ark_bn254_ext::CurveHooks;
use ark_ec::{pairing::Pairing, CurveConfig};
use sp_runtime_interface::runtime_interface;

/// First pairing group definitions.
pub mod g1 {
    /// Group configuration.
    pub type Config = ark_bn254_ext::g1::Config<super::HostHooks>;
    /// Short Weierstrass form point affine representation.
    pub type G1Affine = ark_bn254_ext::g1::G1Affine<super::HostHooks>;
    /// Short Weierstrass form point projective representation.
    pub type G1Projective = ark_bn254_ext::g1::G1Projective<super::HostHooks>;
}

/// Second pairing group definitions.
pub mod g2 {
    /// Group configuration.
    pub type Config = ark_bn254_ext::g2::Config<super::HostHooks>;
    /// Short Weierstrass form point affine representation.
    pub type G2Affine = ark_bn254_ext::g2::G2Affine<super::HostHooks>;
    /// Short Weierstrass form point projective representation.
    pub type G2Projective = ark_bn254_ext::g2::G2Projective<super::HostHooks>;
}

pub use self::{
    g1::{Config as G1Config, G1Affine, G1Projective},
    g2::{Config as G2Config, G2Affine, G2Projective},
};

/// Curve hooks jumping into [`host_calls`] host functions.
#[derive(Copy, Clone)]
pub struct HostHooks;

/// Configuration for *BN254* curve.
#[allow(dead_code)]
pub type Config = ark_bn254_ext::Config<HostHooks>;

/// *BN254* definition.
///
/// A generic *BN254* model specialized with *BN254* configuration.
pub type Bn254 = ark_bn254_ext::Bn254<HostHooks>;

impl CurveHooks for HostHooks {
    fn bn254_multi_miller_loop(
        g1: impl Iterator<Item = <Bn254 as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <Bn254 as Pairing>::G2Prepared>,
    ) -> Result<<Bn254 as Pairing>::TargetField, ()> {
        let g1 = utils::encode(g1.collect::<Vec<_>>());
        let g2 = utils::encode(g2.collect::<Vec<_>>());
        let res = host_calls::bn254_multi_miller_loop(g1, g2).unwrap_or_default();
        utils::decode(res).map_err(|_| ())
    }

    fn bn254_final_exponentiation(
        target: <Bn254 as Pairing>::TargetField,
    ) -> Result<<Bn254 as Pairing>::TargetField, ()> {
        let target = utils::encode(target);
        let res = host_calls::bn254_final_exponentiation(target).unwrap_or_default();
        utils::decode(res).map_err(|_| ())
    }

    fn bn254_msm_g1(
        bases: &[G1Affine],
        scalars: &[<G1Config as CurveConfig>::ScalarField],
    ) -> Result<G1Projective, ()> {
        let bases = utils::encode(bases);
        let scalars = utils::encode(scalars);
        let res = host_calls::bn254_msm_g1(bases, scalars).unwrap_or_default();
        utils::decode_proj_sw(res).map_err(|_| ())
    }

    fn bn254_msm_g2(
        bases: &[G2Affine],
        scalars: &[<G2Config as CurveConfig>::ScalarField],
    ) -> Result<G2Projective, ()> {
        let bases = utils::encode(bases);
        let scalars = utils::encode(scalars);
        let res = host_calls::bn254_msm_g2(bases, scalars).unwrap_or_default();
        utils::decode_proj_sw(res).map_err(|_| ())
    }

    fn bn254_mul_projective_g1(base: &G1Projective, scalar: &[u64]) -> Result<G1Projective, ()> {
        let base = utils::encode_proj_sw(base);
        let scalar = utils::encode(scalar);
        let res = host_calls::bn254_mul_projective_g1(base, scalar).unwrap_or_default();
        utils::decode_proj_sw(res).map_err(|_| ())
    }

    fn bn254_mul_projective_g2(base: &G2Projective, scalar: &[u64]) -> Result<G2Projective, ()> {
        let base = utils::encode_proj_sw(base);
        let scalar = utils::encode(scalar);
        let res = host_calls::bn254_mul_projective_g2(base, scalar).unwrap_or_default();
        utils::decode_proj_sw(res).map_err(|_| ())
    }
}

/// Interfaces for working with *Arkworks* *BN254* elliptic curve related types
/// from within the runtime.
///
/// All types are (de-)serialized through the wrapper types from the `ark-scale` trait,
/// with `ark_scale::{ArkScale, ArkScaleProjective}`.
///
/// `ArkScale`'s `Usage` generic parameter is expected to be set to "not-validated"
/// and "not-compressed".
#[runtime_interface]
pub trait HostCalls {
    /// Pairing multi Miller loop for *BN254*.
    ///
    /// - Receives encoded:
    ///   - `a`: `ArkScale<Vec<G1Affine>>`.
    ///   - `b`: `ArkScale<Vec<G2Affine>>`.
    /// - Returns encoded:  `ArkScale<Bn254::TargetField>`.
    fn bn254_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, BnCryptoError> {
        utils::multi_miller_loop::<ark_bn254::Bn254>(a, b)
    }

    /// Pairing final exponentiation for *BN254*.
    ///
    /// - Receives encoded: `ArkScale<Bn254::TargetField>`.
    /// - Returns encoded:  `ArkScale<Bn254::TargetField>`.
    fn bn254_final_exponentiation(f: Vec<u8>) -> Result<Vec<u8>, BnCryptoError> {
        utils::final_exponentiation::<ark_bn254::Bn254>(f)
    }

    /// Multi scalar multiplication on *G1* for *BN254*.
    ///
    /// - Receives encoded:
    ///   - `bases`: `ArkScale<Vec<G1Affine>>`.
    ///   - `scalars`: `ArkScale<Vec<G1Config::ScalarField>>`.
    /// - Returns encoded: `ArkScaleProjective<G1Projective>`.
    fn bn254_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, BnCryptoError> {
        utils::msm_sw::<ark_bn254::g1::Config>(bases, scalars)
    }

    /// Multi scalar multiplication on *G2* for *BN254*.
    ///
    /// - Receives encoded:
    ///   - `bases`: `ArkScale<Vec<G2Affine>>`.
    ///   - `scalars`: `ArkScale<Vec<G2Config::ScalarField>>`.
    /// - Returns encoded: `ArkScaleProjective<G2Projective>`.
    fn bn254_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, BnCryptoError> {
        utils::msm_sw::<ark_bn254::g2::Config>(bases, scalars)
    }

    /// Projective multiplication on *G1* for *BN254*.
    ///
    /// - Receives encoded:
    ///   - `base`: `ArkScaleProjective<G1Projective>`.
    ///   - `scalar`: `ArkScale<Vec<u64>>`.
    /// - Returns encoded: `ArkScaleProjective<G1Projective>`.
    fn bn254_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, BnCryptoError> {
        utils::mul_projective_sw::<ark_bn254::g1::Config>(base, scalar)
    }

    /// Projective multiplication on *G2* for *BN254*.
    ///
    /// - Receives encoded:
    ///   - `base`: `ArkScaleProjective<G2Projective>`.
    ///   - `scalar`: `ArkScale<Vec<u64>>`.
    /// - Returns encoded: `ArkScaleProjective<ark_bn254::G2Projective>`.
    fn bn254_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, BnCryptoError> {
        utils::mul_projective_sw::<ark_bn254::g2::Config>(base, scalar)
    }
}
