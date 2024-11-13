#![cfg(not(doc))]

use frame_support::{
    migrations::VersionedMigration, storage_alias, traits::UncheckedOnRuntimeUpgrade,
};
use hp_verifiers::Verifier;
use sp_core::Get;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

use crate::Config;

mod v0 {
    use frame_support::Identity;
    use sp_core::H256;

    use super::*;

    /// V0 type for [`crate::Vks`].
    #[storage_alias]
    pub type Vks<T: Config<I>, I: 'static>
    where
        I: Verifier,
    = StorageMap<crate::Pallet<T, I>, Identity, H256, <I as Verifier>::Vk>;
}

/// Implements [`UncheckedOnRuntimeUpgrade`], migrating the state of this pallet from V0 to V1.
///
/// In V0 of the template, the value of the [`crate::Vks`] `StorageMap` is just a `Vk`. In V1,
/// it has been upgraded to contain the struct [`crate::VkEntry`].
///
/// For simplicity, old registered vks are just discarded during the migration.
pub struct InnerMigrateV0ToV1<T, I>(core::marker::PhantomData<(T, I)>);

impl<T: Config<I>, I: 'static> UncheckedOnRuntimeUpgrade for InnerMigrateV0ToV1<T, I>
where
    I: Verifier,
{
    /// Migrate the storage from V0 to V1.
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        let n = v0::Vks::<T, I>::drain().count() as u64;
        T::DbWeight::get().reads_writes(n, n)
    }

    /// Verifies the storage was migrated correctly.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
        frame_support::ensure!(crate::Vks::<T, I>::iter().count() == 0, "error");
        Ok(())
    }
}

/// [`UncheckedOnRuntimeUpgrade`] implementation [`InnerMigrateV0ToV1`] wrapped in a
/// [`VersionedMigration`](frame_support::migrations::VersionedMigration), which ensures that:
/// - The migration only runs once when the on-chain storage version is 0
/// - The on-chain storage version is updated to `1` after the migration executes
/// - Reads/Writes from checking/settings the on-chain storage version are accounted for
pub type MigrateV0ToV1<T, I> = VersionedMigration<
    0, // The migration will only execute when the on-chain storage version is 0
    1, // The on-chain storage version will be set to 1 after the migration is complete
    InnerMigrateV0ToV1<T, I>,
    crate::Pallet<T, I>,
    <T as frame_system::Config>::DbWeight,
>;

#[cfg(any(all(feature = "try-runtime", test), doc))]
mod test {
    use self::InnerMigrateV0ToV1;
    use super::*;
    use crate::mock::*;
    use crate::tests::*;
    use frame_support::assert_ok;
    use frame_support::weights::RuntimeDbWeight;
    use sp_core::H256;

    #[test]
    fn successful_migration() {
        test_ext().execute_with(|| {
            // Populate `Vks` storage map with some entries
            let num_entries = 5;
            for i in 0..num_entries {
                v0::Vks::<Test, FakeVerifier>::insert(H256::from_low_u64_be(i), Box::new(i));
            }
            assert_eq!(
                v0::Vks::<Test, FakeVerifier>::iter().count() as u64,
                num_entries
            );

            // Check that `pre_upgrade` is successful
            let bytes = InnerMigrateV0ToV1::<Test, FakeVerifier>::pre_upgrade()
                .map_err(|e| format!("pre_upgrade failed: {:?}", e))
                .unwrap();

            // Perform runtime upgrade
            let weight = InnerMigrateV0ToV1::<Test, FakeVerifier>::on_runtime_upgrade();

            // Check that `post_upgrade` is successful
            assert_ok!(InnerMigrateV0ToV1::<Test, FakeVerifier>::post_upgrade(
                bytes
            ));

            // Check that weight are as expected
            assert_eq!(
                weight,
                <<Test as frame_system::Config>::DbWeight as Get<RuntimeDbWeight>>::get()
                    .reads_writes(num_entries, num_entries)
            );

            // Check that `Vks` is empty
            assert_eq!(crate::Vks::<Test, FakeVerifier>::iter().count(), 0);
        })
    }
}
