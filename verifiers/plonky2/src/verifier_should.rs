#![cfg(test)]

use super::*;
use frame_support::assert_ok;
use rstest::*;

include!("resources.rs");

#[fixture]
fn valid_test_data() -> TestData {
    get_valid_test_data()
}

#[rstest]
fn verify_valid_proof(valid_test_data: TestData) {
    assert_ok!(Plonky2::verify_proof(
        &valid_test_data.vk,
        &valid_test_data.proof,
        &valid_test_data.pubs
    ));
}

mod reject {
    use frame_support::assert_err;
    use hp_verifiers::VerifyError;

    use super::*;

    #[rstest]
    fn should_fail_on_invalid_pubs(valid_test_data: TestData) {
        let TestData {
            vk,
            proof,
            mut pubs,
        } = valid_test_data;

        pubs[0] = pubs.first().unwrap().wrapping_add(1);

        assert_err!(
            Plonky2::verify_proof(&vk, &proof, &pubs),
            VerifyError::VerifyError
        );
    }

    #[rstest]
    fn should_not_verify_false_proof(valid_test_data: TestData) {
        let TestData {
            vk,
            mut proof,
            pubs,
        } = valid_test_data;

        let len = proof.len();
        proof[len - 1] = pubs.last().unwrap().wrapping_add(1);

        assert_err!(
            Plonky2::verify_proof(&vk, &proof, &pubs),
            VerifyError::VerifyError
        );
    }

    #[rstest]
    fn should_not_validate_corrupted_vk(valid_test_data: TestData) {
        let mut vk = valid_test_data.vk.clone();

        let len = vk.len();
        vk[len - 1] = vk.last().unwrap().wrapping_add(1);

        assert_err!(
            Plonky2::validate_vk(&vk),
            VerifyError::InvalidVerificationKey
        );
    }
}
