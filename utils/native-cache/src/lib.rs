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

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use config::Config;
use walkdir::WalkDir;

// Reexport
pub use dependency::{Boxed, Dependency};
pub use helpers::{is_dyn_or_static_lib, is_name};
pub use lib_dependency::{DependencyImpl, ProfileLibFilesDependencyBuilder};
#[cfg(feature = "rocksdb")]
pub use rocksdb::rocksdb;
#[cfg(feature = "ultraplonk")]
pub use ultraplonk::ultraplonk;

mod config;
mod dependency;
mod helpers;
mod lib_dependency;
mod rocksdb;
mod ultraplonk;

/// Enable logging.
pub const ENABLE_LOGS: bool = false;

/// Handle a single dependency
/// - `target_root` is the path to the target directory where cargo places build artifacts.
/// - `dependency` that should define how to cache by implement [`Dependency`].
/// - `profile` is the compilation profile; In your build script you can use `PROFILE`
///    environment variable that cargo set: `let profile = env::var("PROFILE").unwrap();`
pub fn handle_dependency(
    target_root: impl AsRef<Path>,
    dependency: &impl Dependency,
    profile: &str,
) -> anyhow::Result<()> {
    let valid = cache_dependency(&target_root, dependency, profile)?;
    if valid {
        dependency.rerun_if();
    }
    set_env_paths(dependency, !valid)
}

/// Handle a set of dependencies
/// - `target_root` is the path to the target directory where cargo places build artifacts.
/// - `dependencies` a set of dependencies to be cached; every item should implement [`Dependency`] trait.
/// - `profile` is the compilation profile; In your build script you can use `PROFILE`
///    environment variable that cargo set: `let profile = env::var("PROFILE").unwrap();`
pub fn handle_dependencies<'a>(
    target_root: impl AsRef<Path>,
    dependencies: impl IntoIterator<Item = &'a Box<dyn Dependency>>,
    profile: &str,
) -> anyhow::Result<()> {
    for dependency in dependencies {
        handle_dependency(target_root.as_ref(), dependency, profile)?
    }
    Ok(())
}

fn cache_dependency(
    target_root: impl AsRef<Path>,
    dependency: &impl Dependency,
    profile: &str,
) -> anyhow::Result<bool> {
    let target_path = target_root
        .as_ref()
        .to_path_buf()
        .join(profile)
        .join("build");

    if !dependency.is_valid_cache() {
        // We ignore the error because doesn't matter if the cache folder doesn't exist.
        let _ = fs::remove_dir_all(dependency.cache_path());
        log!("Rebuild from {}", target_path.display());
        for entry in WalkDir::new(target_path).max_depth(1).into_iter().flatten() {
            let path = entry.path();
            log!("folder {}", path.display());
            if dependency.folder_match(path) {
                log!("folder {} MATCH", path.display());
                dependency
                    .cache_files(path)
                    .context("Unable to copy dependency")?;
                return Ok(true);
            }
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

fn set_env_paths(dependency: &impl Dependency, reset: bool) -> anyhow::Result<()> {
    let cargo_config = PathBuf::from(env!("CARGO_HOME")).join("config.toml");

    let mut config = Config::load(cargo_config.clone())?;

    if reset {
        config.remove(dependency);
    } else {
        config.add(dependency);
    }
    config.store()
}
