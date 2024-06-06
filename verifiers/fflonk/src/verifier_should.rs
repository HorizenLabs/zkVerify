#![cfg(test)]

use sp_core::U256;

use super::*;
include!("resources.rs");

#[test]
fn verify_valid_proof() {
    let vk = cdk_key();
    let proof = VALID_PROOF;
    let pubs = VALID_PUBS;

    assert!(Fflonk::verify_proof(&vk, &proof, &pubs).is_ok());
}

#[test]
fn return_the_same_bytes_as_public_inputs() {
    let data: [u8; 32] = VALID_PROOF[0..32].try_into().unwrap();
    assert_eq!(Fflonk::pubs_bytes(&data).as_ref(), &data);
}

#[test]
fn validate_valid_vk() {
    let vk = cdk_key();
    assert!(Fflonk::validate_vk(&vk).is_ok())
}

#[test]
fn reject_malformed_vk() {
    let mut vk = cdk_key();

    *vk.mut_c0_x() = U256::zero();

    assert_eq!(
        Fflonk::validate_vk(&vk),
        Err(VerifyError::InvalidVerificationKey)
    );
}

mod reject {
    use sp_core::U256;

    use super::*;

    #[test]
    fn invalid_pubs() {
        let vk = cdk_key();
        let proof = VALID_PROOF;
        let mut pubs = VALID_PUBS;
        pubs[0] = pubs[0].wrapping_add(1);

        assert_eq!(
            Fflonk::verify_proof(&vk, &proof, &pubs),
            Err(VerifyError::VerifyError)
        );
    }

    #[test]
    fn invalid_proof() {
        let vk = cdk_key();
        let mut invalid_proof: Proof = VALID_PROOF;
        // last byte changed from '0x06' to '0x00' (public inputs)
        invalid_proof[invalid_proof.len() - 1] = 0x00;
        let pubs = VALID_PUBS;

        assert_eq!(
            Fflonk::verify_proof(&vk, &invalid_proof, &pubs),
            Err(VerifyError::VerifyError)
        );
    }

    #[test]
    fn invalid_vk() {
        let mut vk = cdk_key();
        let proof: Proof = VALID_PROOF;
        let pubs = VALID_PUBS;

        *vk.mut_k1() = U256::zero();

        assert_eq!(
            Fflonk::verify_proof(&vk, &proof, &pubs),
            Err(VerifyError::VerifyError)
        );
    }

    #[test]
    fn reject_malformed_proof() {
        let vk = cdk_key();
        let mut malformed_proof: Proof = VALID_PROOF;
        // first byte changed from '0x17' to '0x07' (raw proof data)
        malformed_proof[0] = 0x07;
        let pubs = VALID_PUBS;

        assert_eq!(
            Fflonk::verify_proof(&vk, &malformed_proof, &pubs),
            Err(VerifyError::InvalidProofData)
        );
    }
}
