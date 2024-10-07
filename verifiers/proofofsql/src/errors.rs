pub struct ErrorWrapper(proof_of_sql_verifier::VerifyError);

impl From<ErrorWrapper> for hp_verifiers::VerifyError {
    fn from(value: ErrorWrapper) -> Self {
        match value.0 {
            proof_of_sql_verifier::VerifyError::InvalidInput => {
                hp_verifiers::VerifyError::InvalidInput
            }
            proof_of_sql_verifier::VerifyError::InvalidProofData => {
                hp_verifiers::VerifyError::InvalidProofData
            }
            proof_of_sql_verifier::VerifyError::VerificationFailed => {
                hp_verifiers::VerifyError::VerifyError
            }
            proof_of_sql_verifier::VerifyError::InvalidVerificationKey => {
                hp_verifiers::VerifyError::InvalidVerificationKey
            }
        }
    }
}

impl From<proof_of_sql_verifier::VerifyError> for ErrorWrapper {
    fn from(value: proof_of_sql_verifier::VerifyError) -> Self {
        Self(value)
    }
}
