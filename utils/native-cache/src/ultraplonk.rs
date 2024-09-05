#![cfg(feature = "ultraplonk")]

use std::path::{Path, PathBuf};

use crate::{
    is_dyn_or_static_lib, log, Boxed, Dependency, DependencyImpl, ProfileLibFilesDependencyBuilder,
};

struct UltraplonkDependencyImpl;

impl DependencyImpl for UltraplonkDependencyImpl {
    fn validate_lib_file(p: &Path) -> bool {
        log!("validate_lib_file? {}", p.display());
        is_dyn_or_static_lib(p)
    }

    fn source_libs_folder(folder: &Path) -> PathBuf {
        log!("source_libs_folder? {}", folder.display());
        folder.join("out").join("build").join("lib")
    }

    fn is_target_lib_candidate(p: &str) -> bool {
        log!("is_target_lib_candidate? {}", p);
        p.starts_with("ultraplonk_verifier-")
    }
}

/// Define a ultraplonk-verifier dependency cache
pub fn ultraplonk(cache_root: impl AsRef<Path>, profile: impl AsRef<str>) -> Box<dyn Dependency> {
    ProfileLibFilesDependencyBuilder::<UltraplonkDependencyImpl>::new(
        "ultraplonk",
        "BARRETENBERG_LIB_DIR",
    )
    .with_env("production", "BARRETENBERG_LIB_DIR_RELEASE")
    .with_env("release", "BARRETENBERG_LIB_DIR_RELEASE")
    .with_env("debug", "BARRETENBERG_LIB_DIR_DEBUG")
    .build(cache_root, profile)
    .boxed()
}
