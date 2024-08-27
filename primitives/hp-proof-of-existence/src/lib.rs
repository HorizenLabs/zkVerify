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

#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use sp_core::H256;
use sp_inherents::{InherentIdentifier, IsFatalError};

/// The identifier for the `proof-of-existence0` inherent.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"PoE-0000";

/// The type of the inherent.
pub type InherentType = Attestation;

/// Timestamp wrapper that represents a proof-of-existence0 inherent.
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Attestation;

/// Errors that can occur while checking the timestamp inherent.
#[derive(Encode, Decode, snafu::Snafu, sp_runtime::RuntimeDebug)]
#[cfg_attr(feature = "std", derive())]
pub enum InherentError {
    /// The time between the blocks is too short.
    #[snafu(
        display("The time since the last published root is lower than the minimum period and not enough proofs.")
    )]
    TooEarlyForASmallTree,
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        match self {
            InherentError::TooEarlyForASmallTree => true,
        }
    }
}

impl InherentError {
    /// Try to create an instance ouf of the given identifier and data.
    #[cfg(feature = "std")]
    pub fn try_from(id: &InherentIdentifier, mut data: &[u8]) -> Option<Self> {
        if id == &INHERENT_IDENTIFIER {
            <InherentError as codec::Decode>::decode(&mut data).ok()
        } else {
            None
        }
    }
}

/// Provide the timestamp.
#[cfg(feature = "std")]
#[derive(Default)]
pub struct InherentDataProvider {
    attestation: Attestation,
}

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        inherent_data.put_data(INHERENT_IDENTIFIER, &self.attestation)
    }

    async fn try_handle_error(
        &self,
        identifier: &InherentIdentifier,
        error: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        Some(Err(sp_inherents::Error::Application(Box::from(
            InherentError::try_from(identifier, error)?,
        ))))
    }
}

/// Trait used by proof verifier pallets (e.g. pallet-settlement-fflonk) to signal that a successful proof verification
/// happened.
/// This must be implemented by proof storage pallets (e.g. pallet-attestation) to subscribe to proof verification events.
#[impl_trait_for_tuples::impl_for_tuples(10)]
pub trait OnProofVerified {
    fn on_proof_verified(pubs_hash: H256);
}
