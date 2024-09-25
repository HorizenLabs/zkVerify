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

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use sp_core::{Get, H256};
use sp_std::vec::Vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn publish_attestations() {
        // Setup code
        // TODO: Define the right limit for this one
        let max_storage_attestations: u32 = 1000;
        let max_proofs_per_attestation: u32 = T::ProofsPerAttestation::get();

        let mut hash: [u8; 32] = [0; 32];
        for i in 0..max_storage_attestations {
            let attestation_id = i as u64;
            let mut proofs = Vec::with_capacity(max_proofs_per_attestation as usize);

            for _ in 0..max_proofs_per_attestation {
                // Update the hash to create a unique proof
                for byte in hash.iter_mut() {
                    *byte = byte.wrapping_add(1);
                    if *byte != 0 {
                        break;
                    }
                }

                proofs.push(H256::from_slice(&hash));
            }

            let bounded_proofs: BoundedVec<H256, T::ProofsPerAttestation> =
                BoundedVec::try_from(proofs).expect("Should not exceed max proofs per attestation");
            AttestationsWithProofsToBePublished::<T>::insert(attestation_id, bounded_proofs);
        }

        NextAttestation::<T>::set((max_storage_attestations - 1) as u64);

        // Verify setup
        assert_eq!(
            AttestationsWithProofsToBePublished::<T>::iter().count(),
            max_storage_attestations as usize
        );

        #[extrinsic_call]
        publish_attestations(RawOrigin::Root);

        // Verification
        assert_eq!(
            AttestationsToBeCleared::<T>::get().len() as u32,
            max_storage_attestations,
            "All attestations should be marked for clearing"
        );
        assert_eq!(
            AttestationsWithProofsToBePublished::<T>::iter().count(),
            max_storage_attestations as usize,
            "Attestations should still be present after publishing"
        );
        assert_eq!(
            NextAttestation::<T>::get(),
            max_storage_attestations as u64,
            "NextAttestation should be incremented"
        );
    }

    #[cfg(test)]
    use crate::Pallet as Poe;
    impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test,);
}
