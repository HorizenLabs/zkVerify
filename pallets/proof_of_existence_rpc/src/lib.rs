use std::sync::Arc;

use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;

pub use proof_of_existence_rpc_runtime_api::PoEApi as PoERuntimeApi;
use proof_of_existence_rpc_runtime_api::{AttestationPathRequestError, MerkleProof};

#[rpc(client, server)]
pub trait PoEApi<BlockHash, ResponseType> {
    #[method(name = "poe_proofPath")]
    fn get_proof_path(
        &self,
        attestation_id: u64,
        proof_hash: H256,
        at: Option<BlockHash>,
    ) -> RpcResult<ResponseType>;
}

// Provides RPC methods to query a dispatchable's class, weight and fee.
pub struct PoE<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> PoE<C, P> {
    // Creates a new instance of the PoE Rpc helper.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

// Error type of this RPC api.
pub enum Error {
    /// The transaction was not decodable.
    DecodeError,
    /// The call to runtime failed.
    RuntimeError,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::RuntimeError => 1,
            Error::DecodeError => 2,
        }
    }
}

impl<C, Block> PoEApiServer<<Block as BlockT>::Hash, MerkleProof> for PoE<C, Block>
where
    Block: BlockT,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: PoERuntimeApi<Block>,
{
    fn get_proof_path(
        &self,
        attestation_id: u64,
        proof_hash: sp_core::H256,
        at: Option<Block::Hash>,
    ) -> RpcResult<MerkleProof> {
        let api = self.client.runtime_api();
        let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

        fn map_err(error: impl ToString, desc: &'static str) -> CallError {
            CallError::Custom(ErrorObject::owned(
                Error::RuntimeError.into(),
                desc,
                Some(error.to_string()),
            ))
        }

        let res = api
            .get_proof_path(at_hash, attestation_id, proof_hash)
            .map_err(|e| map_err(e, "Unable to query dispatch info."))
            .and_then(|r| r.map_err(convert_attestation_error))?;
        Ok(res)
    }
}

fn convert_attestation_error(e: AttestationPathRequestError) -> CallError {
    match e {
        AttestationPathRequestError::AttestationIdNotPublished(id) => {
            CallError::Custom(ErrorObject::owned(
                2,
                "Attestation not published yet",
                Some(format!("Attestation {id} not published yet")),
            ))
        }
        AttestationPathRequestError::ProofNotFound(id, h) => CallError::Custom(ErrorObject::owned(
            1,
            "Proof not found",
            Some(format!(
                "Proof {h} not found in Storage for attestation id {id}"
            )),
        )),
    }
}
