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

pub const PROOF_SIZE: usize = 2144;
pub const PUBS_SIZE: usize = 32;
pub const VK_SIZE: usize = 1719;

#[cfg(feature = "std")]
impl From<ultraplonk_verifier::VerifyError> for VerifyError {
    fn from(value: ultraplonk_verifier::VerifyError) -> Self {
        match value {
            ultraplonk_verifier::VerifyError::BackendError(e) => {
                log::warn!("Ultraplonk Backend error on verify proof: {e:?}");
                VerifyError::VerifyError
            }
            ultraplonk_verifier::VerifyError::KeyError(e) => {
                log::debug!("Invalid verification key on verify proof should be a simple verify error: {e:?}");
                VerifyError::VerifyError
            }
            ultraplonk_verifier::VerifyError::PublicInputError { .. } => VerifyError::InvalidInput,
            ultraplonk_verifier::VerifyError::VerificationError => VerifyError::VerifyError,
        }
    }
}

#[runtime_interface]
pub trait UltraplonkVerify {
    fn verify(
        raw_vk: [u8; VK_SIZE],
        raw_proof: &[u8],
        pubs: &[[u8; PUBS_SIZE]],
    ) -> Result<(), VerifyError> {
        let vk = ultraplonk_verifier::VerificationKey::try_from(&raw_vk[..]).map_err(|e| {
            log::debug!("Cannot parse verification key: {:?}", e);
            VerifyError::InvalidVerificationKey
        })?;
        let proof = raw_proof
            .try_into()
            .expect("Just vec of {PROOF_SIZE} bytes can be used");

        ultraplonk_verifier::verify(&vk, &proof, pubs).map_err(Into::into)
    }

    fn validate_vk(raw_vk: &[u8; VK_SIZE]) -> Result<(), VerifyError> {
        let _vk = ultraplonk_verifier::VerificationKey::try_from(&raw_vk[..]).map_err(|e| {
            log::debug!("Cannot parse verification key: {:?}", e);
            VerifyError::InvalidVerificationKey
        })?;
        Ok(())
    }
}
