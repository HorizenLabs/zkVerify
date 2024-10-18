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

#![cfg(test)]

use frame_support::assert_ok;
use rstest::*;
use sp_core::ConstU32;

use super::*;

struct ConfigWithMaxNuEqualTo5;

impl Config for ConfigWithMaxNuEqualTo5 {
    type LargestMaxNu = ConstU32<5>;
}

struct ConfigWithMaxNuEqualTo3;

impl Config for ConfigWithMaxNuEqualTo3 {
    type LargestMaxNu = ConstU32<3>;
}

struct TestData {
    vk: Vec<u8>,
    proof: Vec<u8>,
    pubs: Vec<u8>,
}

#[fixture]
fn valid_test_data() -> TestData {
    TestData {
        vk: include_bytes!("resources/VALID_VK_MAX_NU_4.bin").to_vec(),
        proof: include_bytes!("resources/VALID_PROOF_MAX_NU_4.bin").to_vec(),
        pubs: include_bytes!("resources/VALID_PUBS_MAX_NU_4.bin").to_vec(),
    }
}

#[rstest]
fn verify_valid_proof(valid_test_data: TestData) {
    assert_ok!(ProofOfSql::<ConfigWithMaxNuEqualTo5>::verify_proof(
        &valid_test_data.vk.into(),
        &valid_test_data.proof,
        &valid_test_data.pubs
    ));
}

mod reject {
    use frame_support::assert_err;
    use hp_verifiers::VerifyError;

    use super::*;

    #[rstest]
    fn invalid_proof(valid_test_data: TestData) {
        let mut invalid_pubs = valid_test_data.pubs.clone();
        let pubs_len = invalid_pubs.len();
        invalid_pubs[pubs_len - 1] = invalid_pubs[pubs_len - 1].wrapping_add(1);

        assert_err!(
            ProofOfSql::<ConfigWithMaxNuEqualTo5>::verify_proof(
                &valid_test_data.vk.into(),
                &valid_test_data.proof,
                &invalid_pubs,
            ),
            VerifyError::VerifyError
        )
    }

    #[rstest]
    fn undeserializable_proof(valid_test_data: TestData) {
        let mut malformed_proof = valid_test_data.proof.clone();
        malformed_proof[0] = malformed_proof[0].wrapping_add(1);

        assert_err!(
            ProofOfSql::<ConfigWithMaxNuEqualTo5>::verify_proof(
                &valid_test_data.vk.into(),
                &malformed_proof,
                &valid_test_data.pubs,
            ),
            VerifyError::InvalidProofData
        )
    }

    #[rstest]
    fn undeserializable_pubs(valid_test_data: TestData) {
        let mut malformed_pubs = valid_test_data.pubs.clone();
        malformed_pubs[0] = malformed_pubs[0].wrapping_add(1);

        assert_err!(
            ProofOfSql::<ConfigWithMaxNuEqualTo5>::verify_proof(
                &valid_test_data.vk.into(),
                &valid_test_data.proof,
                &malformed_pubs,
            ),
            VerifyError::InvalidInput
        )
    }

    #[rstest]
    fn undeserializable_vk(valid_test_data: TestData) {
        let mut malformed_vk = valid_test_data.vk.clone();
        malformed_vk[0] = malformed_vk[0].wrapping_add(1);

        assert_err!(
            ProofOfSql::<ConfigWithMaxNuEqualTo5>::verify_proof(
                &malformed_vk.into(),
                &valid_test_data.proof,
                &valid_test_data.pubs,
            ),
            VerifyError::InvalidVerificationKey
        )
    }

    #[rstest]
    fn too_big_vk(valid_test_data: TestData) {
        assert_err!(
            ProofOfSql::<ConfigWithMaxNuEqualTo3>::verify_proof(
                &valid_test_data.vk.into(),
                &valid_test_data.proof,
                &valid_test_data.pubs
            ),
            VerifyError::InvalidVerificationKey
        )
    }

    #[rstest]
    fn too_big_proof(valid_test_data: TestData) {
        let proof = vec![1u8; crate::MAX_PROOF_SIZE as usize + 1];
        assert_err!(
            ProofOfSql::<ConfigWithMaxNuEqualTo5>::verify_proof(
                &valid_test_data.vk.into(),
                &proof.into(),
                &valid_test_data.pubs
            ),
            VerifyError::InvalidProofData
        )
    }

    #[rstest]
    fn too_big_pubs(valid_test_data: TestData) {
        let pubs = vec![1u8; crate::MAX_PUBS_SIZE as usize + 1];
        assert_err!(
            ProofOfSql::<ConfigWithMaxNuEqualTo5>::verify_proof(
                &valid_test_data.vk.into(),
                &valid_test_data.proof,
                &pubs.into()
            ),
            VerifyError::InvalidInput
        )
    }
}
