// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Migration code for the parachain's DB.

#![cfg(feature = "full-node")]

use super::{columns, other_io_error, DatabaseKind, LOG_TARGET};
use std::{
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use polkadot_node_core_approval_voting::approval_db::{
    common::{Config as ApprovalDbConfig, Result as ApprovalDbResult},
    v2::migration_helpers::v1_to_latest,
    v3::migration_helpers::v2_to_latest,
};
use polkadot_node_subsystem_util::database::{
    kvdb_impl::DbAdapter as RocksDbAdapter, paritydb_impl::DbAdapter as ParityDbAdapter, Database,
};
type Version = u32;

/// Version file name.
const VERSION_FILE_NAME: &'static str = "parachain_db_version";

/// Current db version.
/// Version 4 changes approval db format for `OurAssignment`.
/// Version 5 changes approval db format to hold some additional
/// information about delayed approvals.
pub(crate) const CURRENT_VERSION: Version = 5;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error when reading/writing the version")]
    Io(#[from] io::Error),
    #[error("The version file format is incorrect")]
    CorruptedVersionFile,
    #[error("Parachains DB has a future version (expected {current:?}, found {got:?})")]
    FutureVersion { current: Version, got: Version },
    #[error("Parachain DB migration failed")]
    MigrationFailed,
    #[error("Parachain DB migration would take forever")]
    MigrationLoop,
}

impl From<Error> for io::Error {
    fn from(me: Error) -> io::Error {
        match me {
            Error::Io(e) => e,
            _ => super::other_io_error(me.to_string()),
        }
    }
}

/// Try upgrading parachain's database to a target version.
pub(crate) fn try_upgrade_db(
    db_path: &Path,
    db_kind: DatabaseKind,
    target_version: Version,
) -> Result<(), Error> {
    // Ensure we don't loop forever below because of a bug.
    const MAX_MIGRATIONS: u32 = 30;

    #[cfg(test)]
    remove_file_lock(&db_path);

    // Loop migrations until we reach the target version.
    for _ in 0..MAX_MIGRATIONS {
        let version = try_upgrade_db_to_next_version(db_path, db_kind)?;

        #[cfg(test)]
        remove_file_lock(&db_path);

        if version == target_version {
            return Ok(());
        }
    }

    Err(Error::MigrationLoop)
}

/// Try upgrading parachain's database to the next version.
/// If successful, it returns the current version.
pub(crate) fn try_upgrade_db_to_next_version(
    db_path: &Path,
    db_kind: DatabaseKind,
) -> Result<Version, Error> {
    let is_empty = db_path.read_dir().map_or(true, |mut d| d.next().is_none());

    let new_version = if !is_empty {
        match get_db_version(db_path)? {
            // 0 -> 1 migration
            Some(0) => unreachable!(), //migrate_from_version_0_to_1(db_path, db_kind)?,
            // 1 -> 2 migration
            Some(1) => unreachable!(), //migrate_from_version_0_to_1(db_path, db_kind)?,
            // 2 -> 3 migration
            Some(2) => unreachable!(),
            Some(3) => unreachable!(),
            Some(4) => unreachable!(),
            // Already at current version, do nothing.
            Some(CURRENT_VERSION) => CURRENT_VERSION,
            // This is an arbitrary future version, we don't handle it.
            Some(v) => {
                return Err(Error::FutureVersion {
                    current: CURRENT_VERSION,
                    got: v,
                })
            }
            None => unreachable!(),
        }
    } else {
        CURRENT_VERSION
    };

    update_version(db_path, new_version)?;
    Ok(new_version)
}

/// Reads current database version from the file at given path.
/// If the file does not exist returns `None`, otherwise the version stored in the file.
fn get_db_version(path: &Path) -> Result<Option<Version>, Error> {
    match fs::read_to_string(version_file_path(path)) {
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
        Ok(content) => u32::from_str(&content)
            .map(|v| Some(v))
            .map_err(|_| Error::CorruptedVersionFile),
    }
}

/// Writes current database version to the file.
/// Creates a new file if the version file does not exist yet.
fn update_version(path: &Path, new_version: Version) -> Result<(), Error> {
    fs::create_dir_all(path)?;
    fs::write(version_file_path(path), new_version.to_string()).map_err(Into::into)
}

/// Returns the version file path.
fn version_file_path(path: &Path) -> PathBuf {
    let mut file_path = path.to_owned();
    file_path.push(VERSION_FILE_NAME);
    file_path
}

// This currently clears columns which had their configs altered between versions.
// The columns to be changed are constrained by the `allowed_columns` vector.
fn paritydb_fix_columns(
    path: &Path,
    options: parity_db::Options,
    allowed_columns: Vec<u32>,
) -> io::Result<()> {
    // Figure out which columns to delete. This will be determined by inspecting
    // the metadata file.
    if let Some(metadata) = parity_db::Options::load_metadata(&path)
        .map_err(|e| other_io_error(format!("Error reading metadata {:?}", e)))?
    {
        let columns_to_clear = metadata
            .columns
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| allowed_columns.contains(&(*idx as u32)))
            .filter_map(|(idx, opts)| {
                let changed = opts != options.columns[idx];
                if changed {
                    gum::debug!(
                        target: LOG_TARGET,
                        "Column {} will be cleared. Old options: {:?}, New options: {:?}",
                        idx,
                        opts,
                        options.columns[idx]
                    );
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if columns_to_clear.len() > 0 {
            gum::debug!(
                target: LOG_TARGET,
                "Database column changes detected, need to cleanup {} columns.",
                columns_to_clear.len()
            );
        }

        for column in columns_to_clear {
            gum::debug!(target: LOG_TARGET, "Clearing column {}", column,);
            parity_db::clear_column(path, column.try_into().expect("Invalid column ID"))
                .map_err(|e| other_io_error(format!("Error clearing column {:?}", e)))?;
        }

        // Write the updated column options.
        options
            .write_metadata(path, &metadata.salt)
            .map_err(|e| other_io_error(format!("Error writing metadata {:?}", e)))?;
    }

    Ok(())
}

/// Database configuration for version 3.
pub(crate) fn paritydb_version_3_config(path: &Path) -> parity_db::Options {
    let mut options =
        parity_db::Options::with_columns(&path, super::columns::v4::NUM_COLUMNS as u8);
    for i in columns::v4::ORDERED_COL {
        options.columns[*i as usize].btree_index = true;
    }

    options
}



/// Remove the lock file. If file is locked, it will wait up to 1s.
#[cfg(test)]
pub fn remove_file_lock(path: &std::path::Path) {
    use std::{io::ErrorKind, thread::sleep, time::Duration};

    let mut lock_path = std::path::PathBuf::from(path);
    lock_path.push("lock");

    for _ in 0..10 {
        let result = std::fs::remove_file(lock_path.as_path());
        match result {
            Err(error) => match error.kind() {
                ErrorKind::WouldBlock => {
                    sleep(Duration::from_millis(100));
                    continue;
                }
                _ => return,
            },
            Ok(_) => {}
        }
    }

    unreachable!(
        "Database is locked, waited 1s for lock file: {:?}",
        lock_path
    );
}

#[cfg(test)]
mod tests {
    use super::{
        columns::{v2::COL_SESSION_WINDOW_DATA, v4::*},
        *,
    };
    use kvdb_rocksdb::{Database, DatabaseConfig};
    use polkadot_node_core_approval_voting::approval_db::{
        v2::migration_helpers::v1_fill_test_data,
        v3::migration_helpers::{v1_to_latest_sanity_check, v2_fill_test_data},
    };
    use polkadot_node_subsystem_util::database::kvdb_impl::DbAdapter;
    use test_helpers::dummy_candidate_receipt;

}
