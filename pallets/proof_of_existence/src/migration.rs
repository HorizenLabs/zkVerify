use super::*;

use frame_support::pallet_prelude::{StorageVersion, Weight};
use frame_support::traits::GetStorageVersion;
use frame_support::BoundedVec;
use sp_core::Get;
use sp_core::H256;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

pub fn migrate_to_new_storage<T: Config>() -> Weight {
    let onchain_version = Pallet::<T>::on_chain_storage_version();

    // Only proceed with migration if the on-chain storage version is less than the new version.
    if onchain_version < 2 {
        let mut reads: u64 = 0;
        let mut writes: u64 = 0;

        // Step 1: Collect all entries from the old storage `Values<T>` into a map.
        let mut attestation_map: BTreeMap<u64, Vec<H256>> = BTreeMap::new();
        let mut total_entries: u64 = 0;

        for (attestation_id, proof_hash, _) in Values::<T>::iter() {
            attestation_map
                .entry(attestation_id)
                .or_default()
                .push(proof_hash);
            reads += 1;
            total_entries += 1;
        }

        // Step 2: Migrate data to the new storage `AttestationsWithProofsToBePublished<T>`.
        for (attestation_id, proof_hashes) in attestation_map {
            // Convert the Vec<H256> into a BoundedVec<H256, T::ProofsPerAttestation>.
            let bounded_proofs = BoundedVec::<H256, T::ProofsPerAttestation>::try_from(
                proof_hashes.clone(),
            )
            .expect("Number of proofs per attestation should not exceed T::ProofsPerAttestation");
            // Insert the bounded vector into the new storage.
            AttestationsWithProofsToBePublished::<T>::insert(attestation_id, bounded_proofs);
            writes += 1;
        }

        // Step 3: Remove all entries from the old storage `Values<T>`.
        let _ = Values::<T>::clear(u32::MAX, None);
        writes += total_entries; // Each removed entry counts as a write.

        // Step 4: Update the storage version to prevent the migration from running again.
        StorageVersion::new(2).put::<Pallet<T>>();
        writes += 1; // Writing the new storage version is a write.

        // Calculate and return the total weight based on reads and writes.
        T::DbWeight::get().reads_writes(reads, writes)
    } else {
        // If the on-chain storage version is already updated, no migration is needed.
        Weight::zero()
    }
}
