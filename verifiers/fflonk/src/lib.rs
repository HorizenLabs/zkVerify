#![cfg_attr(not(feature = "std"), no_std)]

use hp_verifiers::{Cow, Verifier, VerifyError};

pub const PUBS_SIZE: usize = 32;
pub const PROOF_SIZE: usize = 24 * 32;
pub type Pubs = [u8; PUBS_SIZE];
pub type Proof = [u8; PROOF_SIZE];

pub struct Fflonk;

impl Verifier for Fflonk {
    type Proof = Proof;

    type Pubs = Pubs;

    type Vk = ();

    fn hash_context_data() -> &'static [u8] {
        b"fflonk"
    }

    fn verify_proof(
        _vk: &Self::Vk,
        raw_proof: &Self::Proof,
        raw_pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        let pubs: fflonk_verifier::Public = (*raw_pubs)
            .try_into()
            .map_err(|e| log::error!("Cannot extract public inputs: {:?}", e))
            .map_err(|_| VerifyError::InvalidInput)?;

        let proof = fflonk_verifier::Proof::try_from(raw_proof)
            .map_err(|e| log::debug!("Cannot extract raw proof data: {:?}", e))
            .map_err(|_| VerifyError::InvalidProofData)?;
        log::trace!(
            "Extracted public inputs [{:?}...{:?}] and proof data [{:?}...{:?}]",
            &raw_pubs[0],
            &raw_pubs[PUBS_SIZE - 1],
            &raw_proof[0],
            &raw_proof[PROOF_SIZE - 1]
        );

        proof
            .verify(pubs)
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| VerifyError::VerifyError)
    }

    fn validate_vk(_vk: &Self::Vk) -> Result<(), hp_verifiers::VerifyError> {
        Ok(())
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        Cow::Borrowed(pubs)
    }
}
