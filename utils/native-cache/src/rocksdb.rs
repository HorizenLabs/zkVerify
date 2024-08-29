#![cfg(feature = "rocksdb")]

use std::path::{Path, PathBuf};

use crate::{is_name, log, Boxed, Dependency, DependencyImpl, ProfileLibFilesDependencyBuilder};

pub fn rocksdb(cache_root: impl AsRef<Path>, profile: impl AsRef<str>) -> Vec<Box<dyn Dependency>> {
    vec![
        ProfileLibFilesDependencyBuilder::<RocksDbDependencyImpl>::new(
            "rocksdb",
            "ROCKSDB_LIB_DIR",
        )
        .build(cache_root.as_ref(), profile.as_ref())
        .boxed(),
        ProfileLibFilesDependencyBuilder::<SnappyDependencyImpl>::new("snappy", "SNAPPY_LIB_DIR")
            .build(cache_root, profile)
            .boxed(),
    ]
}

struct RocksDbDependencyImpl;

impl DependencyImpl for RocksDbDependencyImpl {
    fn validate_lib_file(p: &Path) -> bool {
        is_name(p, "librocksdb.a")
    }

    fn source_libs_folder(folder: &Path) -> PathBuf {
        folder.join("out")
    }

    fn is_target_lib_candidate(p: &str) -> bool {
        p.starts_with("librocksdb-sys-")
    }
}

struct SnappyDependencyImpl;

impl DependencyImpl for SnappyDependencyImpl {
    fn validate_lib_file(p: &Path) -> bool {
        log!("validate_lib_file? {}", p.display());
        is_name(p, "libsnappy.a")
    }

    fn source_libs_folder(folder: &Path) -> PathBuf {
        log!("source_libs_folder? {}", folder.display());
        folder.join("out")
    }

    fn is_target_lib_candidate(p: &str) -> bool {
        log!("is_target_lib_candidate? {}", p);
        p.starts_with("librocksdb-sys-")
    }
}
