use super::*;

use frame_support::pallet_prelude::{StorageVersion, Weight};
use frame_support::traits::GetStorageVersion;
use sp_core::Get;

pub fn migrate_to_v2<T: Config>() -> Weight {
    let onchain_version = Pallet::<T>::on_chain_storage_version();
    if onchain_version < 2 {
        let mut total_reads: u64 = 0;
        let mut total_writes: u64 = 0;

        // migrate OldestAttestation to OldestAttestationV2
        OldestAttestationV2::<T>::insert(
            COMMON_ATTESTATION_CHAIN_ID,
            OldestAttestation::<T>::get(),
        );
        total_writes += 1;

        // migrate NextAttestation to NextAttestationV2
        NextAttestationV2::<T>::insert(COMMON_ATTESTATION_CHAIN_ID, NextAttestation::<T>::get());
        total_writes += 1;

        // migrate FirstInsertionTime to FirstInsertionTimeV2
        FirstInsertionTimeV2::<T>::insert(
            COMMON_ATTESTATION_CHAIN_ID,
            FirstInsertionTime::<T>::get().unwrap(),
        );
        total_writes += 1;

        // migrate Values to ValuesV2
        for (att_id, proof_hash, value) in Values::<T>::drain() {
            let att_id_with_common_attestation_chain_id = (att_id, COMMON_ATTESTATION_CHAIN_ID);
            ValuesV2::<T>::insert(att_id_with_common_attestation_chain_id, proof_hash, value);
        }
        let values_reads: u64 = Values::<T>::iter().count().try_into().unwrap();
        let values_writes: u64 = ValuesV2::<T>::iter().count().try_into().unwrap();
        total_reads += values_reads;
        total_writes += values_writes;

        StorageVersion::new(2).put::<Pallet<T>>();

        T::DbWeight::get().reads_writes(total_reads, total_writes)
    } else {
        Weight::zero()
    }
}
