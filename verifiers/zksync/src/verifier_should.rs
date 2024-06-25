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

use hp_verifiers::Verifier;

use crate::Zksync;
include!("resources.rs");

#[test]
fn verify_valid_proof() {
    assert!(Zksync::verify_proof(&(), &PROOF, &PUBS).is_ok());
}

#[test]
fn return_the_same_bytes_as_public_inputs() {
    // We use some other bytes to be sure that the pubs are not hardcoded
    let data: [u8; 32] = PROOF[0..32].try_into().unwrap();
    assert_eq!(Zksync::pubs_bytes(&data).as_ref(), &data);
}

mod reject {
    use hp_verifiers::VerifyError;

    use super::*;

    #[test]
    fn invalid_pubs() {
        let mut invalid_pubs = PUBS;
        invalid_pubs[0] = invalid_pubs[0].wrapping_add(1);

        assert_eq!(
            Zksync::verify_proof(&(), &PROOF, &invalid_pubs),
            Err(VerifyError::VerifyError)
        );
    }

    #[test]
    fn invalid_proof() {
        let mut invalid_proof = PROOF;
        invalid_proof[730] = 0x00;

        assert_eq!(
            Zksync::verify_proof(&(), &invalid_proof, &PUBS),
            Err(VerifyError::VerifyError)
        );
    }

    #[test]
    fn reject_malformed_proof() {
        let mut malformed_proof = PROOF;
        malformed_proof[0] = 0xff;

        assert_eq!(
            Zksync::verify_proof(&(), &malformed_proof, &PUBS),
            Err(VerifyError::InvalidProofData)
        );
    }
}
