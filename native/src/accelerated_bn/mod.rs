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

#![warn(missing_docs)]

use codec::{Decode, Encode};
use sp_runtime_interface::pass_by::PassByCodec;

#[cfg(feature = "bn254")]
pub mod bn254;

#[cfg(feature = "bn254")]
mod utils;

/// Errors that can occur during cryptographic operations.
#[derive(Debug, PartialEq, PassByCodec, Encode, Decode)]
pub enum BnCryptoError {
    /// Decoding failed.
    DecodeError,
    /// Unequal length of bases and scalars.
    BaseScalarLengthMismatch,
    /// Final Exponentiation failed.
    FinalExponError,
}
