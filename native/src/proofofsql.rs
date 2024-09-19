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

use crate::VerifyError;
use sp_runtime_interface::runtime_interface;

#[cfg(feature = "std")]
impl From<proof_of_sql_verifier::VerifyError> for VerifyError {
    fn from(value: proof_of_sql_verifier::VerifyError) -> Self {
        match value {
            proof_of_sql_verifier::VerifyError::InvalidInput => VerifyError::InvalidInput,
            proof_of_sql_verifier::VerifyError::InvalidProofData => VerifyError::InvalidProofData,
            proof_of_sql_verifier::VerifyError::InvalidVerificationKey => {
                VerifyError::InvalidVerificationKey
            }
            proof_of_sql_verifier::VerifyError::VerificationFailed => VerifyError::VerifyError,
        }
    }
}

#[runtime_interface]
pub trait ProofofsqlVerify {
    fn verify(raw_vk: &[u8], proof: &[u8], pubs: &[u8]) -> Result<(), VerifyError> {
        proof_of_sql_verifier::verify(proof, pubs, raw_vk).map_err(Into::into)
    }

    fn validate_vk(raw_vk: &[u8]) -> Result<(), VerifyError> {
        let _vk = proof_of_sql_verifier::VerificationKey::try_from(raw_vk).map_err(|e| {
            log::debug!("Cannot parse verification key: {:?}", e);
            VerifyError::InvalidVerificationKey
        })?;
        Ok(())
    }

    fn verifier_key_size(max_nu: u32) -> u32 {
        proof_of_sql_verifier::VerificationKey::serialized_size(max_nu as usize) as u32
    }
}
