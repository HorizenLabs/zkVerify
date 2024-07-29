#![cfg(test)]

use super::*;

struct Mock;

pub const SOME_PARAMETER_CONST: u8 = 1;

impl Config for Mock {
    type SomeParameter = ConstU8<SOME_PARAMETER_CONST>; // arbitrary value for tests
}

include!("resources.rs");

#[test]
fn verify_valid_proof() {
    assert!(Foo::<Mock>::verify_proof(&VALID_VK, &VALID_PROOF, &VALID_PUBS).is_ok());
}

mod reject {
    use hp_verifiers::VerifyError;

    use super::*;

    #[test]
    fn invalid_proof() {
        let mut invalid_pubs = VALID_PUBS.clone();
        invalid_pubs[0] = SOME_PARAMETER_CONST
            .saturating_sub(VALID_VK[0])
            .saturating_sub(VALID_PROOF[0]);

        assert_eq!(
            Foo::<Mock>::verify_proof(&VALID_VK, &VALID_PROOF, &invalid_pubs),
            Err(VerifyError::VerifyError)
        )
    }
}
