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

pub const PUBS_SIZE: usize = 32;
pub const PROOF_SIZE: usize = 44 * 32;

#[runtime_interface]
pub trait ZksyncVerify {
    fn verify(
        proof_bytes: &[u8; PROOF_SIZE],
        pubs_bytes: [u8; PUBS_SIZE],
    ) -> Result<(), VerifyError> {
        let pubs = zksync_era_verifier_deserialize::fr(&pubs_bytes)
            .map_err(|e| log::error!("Cannot extract public inputs: {:?}", e))
            .map_err(|_| VerifyError::InvalidInput)?;
        let mut proof = zksync_era_verifier::deserialize_eth_proof(proof_bytes)
            .map_err(|e| log::debug!("Cannot extract raw proof data: {:?}", e))
            .map_err(|_| VerifyError::InvalidProofData)?;
        log::trace!(
            "Extracted public inputs [{:?}...{:?}] and proof data [{:?}...{:?}]",
            pubs_bytes[0],
            pubs_bytes[PUBS_SIZE - 1],
            proof_bytes[0],
            proof_bytes[PROOF_SIZE - 1]
        );
        proof.inputs = vec![pubs];
        zksync_era_verifier::verify(&zksync_era_verifier::default_eth_vk(), &proof)
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| VerifyError::VerifyError)
            .and_then(|verified| verified.then_some(()).ok_or(VerifyError::VerifyError))
            .map(|_| log::trace!("verified"))
    }
}
