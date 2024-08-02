use crate::VerifyError;
use sp_runtime_interface::runtime_interface;

#[cfg(feature = "std")]
impl From<foo_verifier::VerifyError> for VerifyError {
    fn from(value: foo_verifier::VerifyError) -> Self {
        match value {
            foo_verifier::VerifyError::Failure => VerifyError::VerifyError,
        }
    }
}

#[runtime_interface]
pub trait FooVerify {
    fn verify(vk: [u8; 32], proof: &[u8; 512], pubs: &[u8; 32]) -> Result<(), VerifyError> {
        foo_verifier::verify(vk.into(), *proof, *pubs)
            .inspect_err(|_| log::debug!("Cannot verify foo proof"))
            .map_err(Into::into)
            .map(|_| log::trace!("verified"))
    }
}
