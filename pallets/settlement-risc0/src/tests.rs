// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frame_support::testing_prelude::*;
use hex_literal::hex;
use rstest::rstest;

use sp_core::H256;

use crate::mock::*;
use crate::{mock, Error};
use crate::{MAX_PROOF_SIZE, MAX_PUBS_SIZE};

include!("data.rs");
pub static VALID_HASH: [u8; 32] =
    hex!("7deb0a79fbc0543267887ed5050f5db13c2a766298511aaaf1a3aff8f42884ad");

#[test]
fn valid_proof_passes_verification_and_is_notified() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert!(SettlementRisc0Pallet::submit_proof(
            RuntimeOrigin::signed(1),
            VALID_VK,
            VALID_PROOF.to_vec(),
            VALID_PUBS.to_vec()
        )
        .is_ok());

        let events = mock::System::events();
        assert_eq!(events.len(), 1);

        mock::System::assert_last_event(
            crate::mock::on_proof_verified::pallet::Event::NewProof {
                value: H256(VALID_HASH),
            }
            .into(),
        );
    });
}

fn modify_data(data: &mut [u8], position: usize, increment: u8) {
    data[position] = data[position] + increment;
}

#[rstest]
#[case::undeserializable_proof((0, 1), (0, 0), Error::<Test>::InvalidProof)]
#[case::undeserializable_pubs((0, 0), (0, 1), Error::<Test>::InvalidPublicInputs)]
fn malformed_data_fails_verification_and_is_not_notified(
    #[case] modify_proof: (usize, u8),
    #[case] modify_pubs: (usize, u8),
    #[case] expected_error: Error<Test>,
) {
    new_test_ext().execute_with(|| {
        let mut proof = VALID_PROOF.clone();
        modify_data(&mut proof, modify_proof.0, modify_proof.1);

        let mut pubs = VALID_PUBS;
        modify_data(&mut pubs, modify_pubs.0, modify_pubs.1);

        // Dispatch a signed extrinsic.
        assert_noop!(
            SettlementRisc0Pallet::submit_proof(
                RuntimeOrigin::signed(1),
                VALID_VK,
                proof.to_vec(),
                pubs.to_vec()
            ),
            expected_error
        );
    })
}

#[rstest]
#[case::false_proof((VALID_PUBS.len() - 1, 1), Error::<Test>::VerifyError)]
fn false_proof_fails_verification_and_is_not_notified(
    #[case] modify_pubs: (usize, u8),
    #[case] expected_error: Error<Test>,
) {
    new_test_ext().execute_with(|| {
        let mut pubs = VALID_PUBS;
        modify_data(&mut pubs, modify_pubs.0, modify_pubs.1);

        // Dispatch a signed extrinsic.
        assert_noop!(
            SettlementRisc0Pallet::submit_proof(
                RuntimeOrigin::signed(1),
                VALID_VK,
                VALID_PROOF.to_vec(),
                pubs.to_vec()
            ),
            expected_error
        );
    })
}

fn extend_data(data: &mut Vec<u8>, size: usize) {
    let extension = vec![0; size];
    data.extend(extension);
}

#[rstest]
#[case::too_big_proof(<u32 as TryInto<usize>>::try_into(MAX_PROOF_SIZE).unwrap() - VALID_PROOF.len() + 1, 0, Error::<Test>::InvalidProofSize)]
#[case::too_big_pubs(0, <u32 as TryInto<usize>>::try_into(MAX_PUBS_SIZE).unwrap() - VALID_PUBS.len() + 1, Error::<Test>::InvalidPublicInputsSize)]
fn excessive_size_data_fails_verification_and_is_not_notified(
    #[case] extend_proof: usize,
    #[case] extend_pubs: usize,
    #[case] expected_error: Error<Test>,
) {
    new_test_ext().execute_with(|| {
        let mut proof = VALID_PROOF.to_vec();
        extend_data(&mut proof, extend_proof);

        let mut pubs = VALID_PUBS.to_vec();
        extend_data(&mut pubs, extend_pubs);

        // Dispatch a signed extrinsic.
        assert_noop!(
            SettlementRisc0Pallet::submit_proof(
                RuntimeOrigin::signed(1),
                VALID_VK,
                proof.to_vec(),
                pubs.to_vec()
            ),
            expected_error
        );
    })
}
