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
impl From<risc0_verifier::VerifyError> for VerifyError {
    fn from(value: risc0_verifier::VerifyError) -> Self {
        match value {
            risc0_verifier::VerifyError::InvalidData {
                cause: risc0_verifier::DeserializeError::InvalidProof,
            } => VerifyError::InvalidProofData,
            risc0_verifier::VerifyError::InvalidData {
                cause: risc0_verifier::DeserializeError::InvalidPublicInputs,
            } => VerifyError::InvalidInput,
            _ => VerifyError::VerifyError,
        }
    }
}

#[runtime_interface]
pub trait Risc0Verify {
    fn verify(vk: [u8; 32], proof: &[u8], pubs: &[u8]) -> Result<(), VerifyError> {
        risc0_verifier::verify(vk.into(), proof, pubs)
            .inspect_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(Into::into)
            .map(|_| log::trace!("verified"))
    }
}
