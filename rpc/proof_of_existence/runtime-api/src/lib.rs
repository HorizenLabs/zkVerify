#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
pub use pallet_poe::AttestationPathRequestError;
use scale_info::TypeInfo;
use sp_runtime::{Deserialize, SaturatedConversion, Serialize};
use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
    #[api_version(4)]
    pub trait PoEApi
    {
        // Returns the Merkle path for the given attestation id and proof hash
        fn get_proof_path(attestation_id: u64, proof_hash: sp_core::H256) -> Result<MerkleProof, AttestationPathRequestError>;
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct MerkleProof {
    pub root: sp_core::H256,
    pub proof: Vec<sp_core::H256>,
    pub number_of_leaves: u32,
    pub leaf_index: u32,
    pub leaf: sp_core::H256,
}

impl From<binary_merkle_tree::MerkleProof<sp_core::H256, sp_core::H256>> for MerkleProof {
    fn from(value: binary_merkle_tree::MerkleProof<sp_core::H256, sp_core::H256>) -> Self {
        MerkleProof {
            root: value.root,
            proof: value.proof,
            number_of_leaves: value.number_of_leaves.saturated_into(),
            leaf_index: value.leaf_index.saturated_into(),
            leaf: value.leaf,
        }
    }
}
