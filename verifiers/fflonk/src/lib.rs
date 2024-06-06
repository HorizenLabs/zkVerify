#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;

use frame_support::weights::Weight;
use hp_verifiers::{Cow, Verifier, VerifyError};

pub mod benchmarking;
mod verifier_should;
mod vk;
mod weight;

pub const PUBS_SIZE: usize = 32;
pub const PROOF_SIZE: usize = 24 * 32;
pub type Pubs = [u8; PUBS_SIZE];
pub type Proof = [u8; PROOF_SIZE];

#[pallet_verifiers::verifier]
pub struct Fflonk;

impl Verifier for Fflonk {
    type Proof = Proof;

    type Pubs = Pubs;

    type Vk = vk::Vk;

    fn hash_context_data() -> &'static [u8] {
        b"fflonk"
    }

    fn verify_proof(
        vk: &Self::Vk,
        raw_proof: &Self::Proof,
        raw_pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        let vk: fflonk_verifier::VerificationKey = vk
            .clone()
            .try_into()
            .map_err(|e| log::debug!("Invalid Vk: {:?}", e))
            .map_err(|_| VerifyError::InvalidVerificationKey)?;
        let pubs: fflonk_verifier::Public = (*raw_pubs).into();
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

        fflonk_verifier::verify(&vk, &proof, &pubs)
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| VerifyError::VerifyError)
    }

    fn validate_vk(vk: &Self::Vk) -> Result<(), hp_verifiers::VerifyError> {
        let _: fflonk_verifier::VerificationKey = vk
            .clone()
            .try_into()
            .map_err(|e| log::debug!("Invalid Vk: {:?}", e))
            .map_err(|_| VerifyError::InvalidVerificationKey)?;
        Ok(())
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        Cow::Borrowed(pubs)
    }
}

/// The struct to use in runtime pallet configuration to map the weight computed by this crate
/// benchmarks to the weight needed by the `pallet-verifiers`.
/// In this case the implementation doesn't depends from the kind of proof or public input and
/// the crate's benchmarks are mapped 1-1 to the `pallet-verifiers`'s one.
pub struct FflonkWeight<W: weight::WeightInfo, T: frame_system::Config>(W, PhantomData<T>);

impl<W: weight::WeightInfo, T: frame_system::Config> pallet_verifiers::WeightInfo<Fflonk>
    for FflonkWeight<W, T>
{
    fn submit_proof(
        _proof: &<Fflonk as hp_verifiers::Verifier>::Proof,
        _pubs: &<Fflonk as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof()
    }

    fn submit_proof_with_vk_hash(
        _proof: &<Fflonk as hp_verifiers::Verifier>::Proof,
        _pubs: &<Fflonk as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof_with_vk_hash()
    }

    fn register_vk(_vk: &<Fflonk as hp_verifiers::Verifier>::Vk) -> Weight {
        W::register_vk()
    }
}
