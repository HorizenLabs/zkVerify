#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_runtime_interface::pass_by::PassByCodec;
use sp_runtime_interface::runtime_interface;

#[derive(PassByCodec, Encode, Decode)]
pub enum VerifyError {
    InvalidInput,
    InvalidProofData,
    VerifyError,
}

pub const ZKSYNC_PUBS_SIZE: usize = 32;
pub const ZKSYNC_PROOF_SIZE: usize = 44 * 32;

#[runtime_interface]
pub trait ZksyncVerify {
    fn verify(
        proof_bytes: &[u8; ZKSYNC_PROOF_SIZE],
        pubs_bytes: [u8; ZKSYNC_PUBS_SIZE],
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
            pubs_bytes[ZKSYNC_PUBS_SIZE - 1],
            proof_bytes[0],
            proof_bytes[ZKSYNC_PROOF_SIZE - 1]
        );
        proof.inputs = vec![pubs];
        zksync_era_verifier::verify(&zksync_era_verifier::default_eth_vk(), &proof)
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| VerifyError::VerifyError)
            .and_then(|verified| verified.then_some(()).ok_or(VerifyError::VerifyError))
            .map(|_| log::trace!("verified"))
    }
}

#[cfg(feature = "std")]
pub use zksync_verify::HostFunctions as ZksyncVerifierHostFunctions;

#[cfg(feature = "std")]
pub type HLNativeHostFunctions = (ZksyncVerifierHostFunctions,);
