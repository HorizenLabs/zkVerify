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

use sp_core::ConstU32;

use super::*;

struct Mock;

impl Config for Mock {
    type MaxProofSize = ConstU32<1000000>; // arbitrary length for tests
    type MaxPubsSize = ConstU32<100>; // arbitrary length for tests
}

include!("resources.rs");

#[test]
fn verify_valid_proof() {
    assert!(
        Risc0::<Mock>::verify_proof(&VALID_VK, &VALID_PROOF.to_vec(), &VALID_PUBS.to_vec()).is_ok()
    );
}

mod reject {
    use hp_verifiers::VerifyError;

    use super::*;

    #[test]
    fn invalid_proof() {
        let mut invalid_pubs = VALID_PUBS.clone();
        invalid_pubs[invalid_pubs.len() - 1] = invalid_pubs[invalid_pubs.len() - 1].wrapping_add(1);

        assert_eq!(
            Risc0::<Mock>::verify_proof(&VALID_VK, &VALID_PROOF.to_vec(), &invalid_pubs.to_vec()),
            Err(VerifyError::VerifyError)
        )
    }

    #[test]
    fn undeserializable_proof() {
        let mut malformed_proof = VALID_PROOF.clone();
        malformed_proof[0] = malformed_proof[0].wrapping_add(1);

        assert_eq!(
            Risc0::<Mock>::verify_proof(&VALID_VK, &malformed_proof.to_vec(), &VALID_PUBS.to_vec()),
            Err(VerifyError::InvalidProofData)
        )
    }

    #[test]
    fn undeserializable_pubs() {
        let mut malformed_pubs = VALID_PUBS.clone();
        malformed_pubs[0] = malformed_pubs[0].wrapping_add(1);

        assert_eq!(
            Risc0::<Mock>::verify_proof(&VALID_VK, &VALID_PROOF.to_vec(), &malformed_pubs.to_vec()),
            Err(VerifyError::InvalidInput)
        )
    }

    #[test]
    fn too_big_proof() {
        let too_big_proof = vec![0; Mock::max_proof_size() as usize + 1];

        assert_eq!(
            Risc0::<Mock>::verify_proof(&VALID_VK, &too_big_proof, &VALID_PUBS.to_vec()),
            Err(VerifyError::InvalidProofData)
        )
    }

    #[test]
    fn too_big_pubs() {
        let too_big_pubs = vec![0; Mock::max_pubs_size() as usize + 1];

        assert_eq!(
            Risc0::<Mock>::verify_proof(&VALID_VK, &VALID_PROOF.to_vec(), &too_big_pubs),
            Err(VerifyError::InvalidInput)
        )
    }
}
