use crate::utils::ScalarFieldFor;
// use ark_bn254::{G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::pairing::Pairing;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ec::AffineRepr;
use native::{g1, g2};
use native::{
    Bn254 as Bn254Opt, G1Affine as G1AffineOpt, G1Projective as G1ProjectiveOpt,
    G2Affine as G2AffineOpt, G2Projective as G2ProjectiveOpt,
}; // Fr as FrOpt

#[inline]
pub fn pairing_opt(a: G1AffineOpt, b: G2AffineOpt) {
    let _out = Bn254Opt::multi_pairing([a], [b]);
}

#[inline]
pub fn msm_g1_opt(bases: &[G1AffineOpt], scalars: &[<G1AffineOpt as AffineRepr>::ScalarField]) {
    let _out = <g1::Config as SWCurveConfig>::msm(bases, scalars);
}

#[inline]
pub fn msm_g2_opt(bases: &[G2AffineOpt], scalars: &[ScalarFieldFor<G2AffineOpt>]) {
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
