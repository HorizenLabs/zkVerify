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

#![allow(dead_code)]

use crate::ArkScale;
use ark_scale::hazmat::ArkScaleProjective;
use ark_serialize::{CanonicalSerialize, Compress};
use ark_std::{test_rng, vec, vec::Vec, UniformRand};

// `words_count` is the scalar length in words, with 1 word assumed to be 64 bits.
// Most significant bit is set.
fn make_scalar(words_count: u32) -> Vec<u64> {
    let mut scalar: Vec<_> = (0..words_count as usize)
        .map(|_| u64::rand(&mut test_rng()))
        .collect();
    // Arkworks assumes scalar to be in **big endian**
    scalar[0] |= 1 << 63;
    scalar
}

fn make_base<Group: UniformRand>() -> Group {
    Group::rand(&mut test_rng())
}

// `words_count` is the scalar length in words, with 1 word assumed to be 64 bits.
// Most significant bit is set.
pub fn make_scalar_args<Group: UniformRand>(
    words_count: u32,
) -> (ArkScale<Group>, ArkScale<Vec<u64>>) {
    (make_base::<Group>().into(), make_scalar(words_count).into())
}

// `words_count` is the scalar length in words, with 1 word assumed to be 64 bits.
// Most significant bit is set.
pub fn make_scalar_args_projective<Group: UniformRand>(
    words_count: u32,
) -> (ArkScaleProjective<Group>, ArkScale<Vec<u64>>) {
    (make_base::<Group>().into(), make_scalar(words_count).into())
}

pub fn make_pairing_args<GroupA: UniformRand, GroupB: UniformRand>(
) -> (ArkScale<GroupA>, ArkScale<GroupB>) {
    (make_base::<GroupA>().into(), make_base::<GroupB>().into())
}

pub fn make_msm_args<Group: ark_ec::VariableBaseMSM>(
    size: u32,
) -> (ArkScale<Vec<Group>>, ArkScale<Vec<Group::ScalarField>>) {
    let rng = &mut test_rng();
    let scalars = (0..size)
        .map(|_| Group::ScalarField::rand(rng))
        .collect::<Vec<_>>();
    let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
    let bases: ArkScale<Vec<Group>> = bases.into();
    let scalars: ArkScale<Vec<Group::ScalarField>> = scalars.into();
    (bases, scalars)
}

pub fn serialize_argument(argument: impl CanonicalSerialize) -> Vec<u8> {
    let mut buf = vec![0; argument.serialized_size(Compress::No)];
    argument.serialize_uncompressed(buf.as_mut_slice()).unwrap();
    buf
}
