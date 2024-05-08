// Copyright 2024, The Horizen Foundation
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

use frame_support::dispatch::{GetDispatchInfo, Pays};
use frame_support::testing_prelude::*;
use hex_literal::hex;
use rstest::rstest;

use sp_core::H256;

use super::Proof;
use crate::mock::*;
use crate::weight::WeightInfo;
use crate::{mock, Error};

include!("proof.rs");
pub static VALID_HASH: [u8; 32] =
    hex!("1e1a36739cf4c3c3b01b5be727125eb435f2c0cefc2ef5ec0f443ca5752b839f");

#[test]
fn valid_proof_passes_verification_and_is_notified() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert!(
            SettlementZksyncPallet::submit_proof(RuntimeOrigin::signed(1), VALID_PROOF.into())
                .is_ok()
        );

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

#[rstest]
#[case::syntactic_valid_proof((730, 0x00), Error::<Test>::VerifyError)]
#[case::syntactic_invalid_proof((0, 0x00), Error::<Test>::InvalidProofData)]
#[case::invalid_pubs((1408, 0xFF), Error::<Test>::InvalidInput)]
#[case::wrong_public_input_value((1439, 0x00), Error::<Test>::VerifyError)]
fn malformed_data_fails_verification_and_is_not_notified(
    #[case] change: (usize, u8),
    #[case] expected_error: Error<Test>,
) {
    new_test_ext().execute_with(|| {
        let mut data: Proof = VALID_PROOF;
        data[change.0] = change.1;

        // Dispatch a signed extrinsic.
        assert_noop!(
            SettlementZksyncPallet::submit_proof(RuntimeOrigin::signed(1), data.into()),
            expected_error
        );
    })
}

#[test]
fn should_use_the_configured_weights() {
    let proof: Proof = VALID_PROOF;
    let info = crate::pallet::Call::<Test>::submit_proof {
        raw_proof: Box::new(proof),
    }
    .get_dispatch_info();

    assert_eq!(info.pays_fee, Pays::Yes);
    assert_eq!(info.weight, MockWeightInfo::submit_proof());
}
