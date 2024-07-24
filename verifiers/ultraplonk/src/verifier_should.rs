// Copyright 2024, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg(test)]

use serial_test::serial;

use super::*;
include!("resources.rs");

#[test]
#[serial]
fn verify_valid_proof() {
    let vk = vk_key();
    let proof = VALID_PROOF;
    let pi = public_input();

    assert!(Ultraplonk::verify_proof(&vk, &proof, &pi).is_ok());
}

mod reject {
    use super::*;

    #[test]
    #[serial]
    fn invalid_pubs() {
        let vk = vk_key();
        let mut invalid_pubs = public_input();
        invalid_pubs[0][0] = 0x10;

        assert_eq!(
            Ultraplonk::verify_proof(&vk, &VALID_PROOF, &invalid_pubs),
            Err(VerifyError::InvalidInput)
        );
    }

    #[test]
    #[serial]
    fn invalid_proof() {
        let vk = vk_key();
        let pi = public_input();
        let mut invalid_proof: Proof = VALID_PROOF;
        // last byte changed from '0x06' to '0x00' (public inputs)
        invalid_proof[invalid_proof.len() - 1] = 0x00;

        assert_eq!(
            Ultraplonk::verify_proof(&vk, &invalid_proof, &pi),
            Err(VerifyError::VerifyError)
        );
    }

    #[test]
    #[serial]
    fn invalid_vk() {
        let mut vk = vk_key();
        let pi = public_input();

        vk[10] = vk[10].wrapping_add(1);

        assert_eq!(
            Ultraplonk::verify_proof(&vk, &VALID_PROOF, &pi),
            Err(VerifyError::VerifyError)
        );
    }

    #[test]
    #[serial]
    fn reject_malformed_proof() {
        let vk = vk_key();
        let pi = public_input();
        let mut malformed_proof: Proof = VALID_PROOF;
        // first byte changed from '0x17' to '0x07' (raw proof data)
        malformed_proof[0] = 0x07;

        assert_eq!(
            Ultraplonk::verify_proof(&vk, &malformed_proof, &pi),
            Err(VerifyError::VerifyError)
        );
    }
}
