// Copyright 2024, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use native_cache::rocksdb;
use std::env;
use std::path::PathBuf;
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    let cargo_config = PathBuf::from(env!("CARGO_HOME")).join("config.toml");
    println!("cargo::rerun-if-changed={:?}", cargo_config);
    println!("cargo:rustc-link-arg=-fuse-ld=lld");

    generate_cargo_keys();

    rerun_if_git_head_changed();

    let profile = env::var("PROFILE").expect("Should have a PROFILE environment variable");
    let cache_root: PathBuf = env::current_dir().unwrap().join("../deps");
    native_cache::handle_dependencies("../target", &rocksdb(&cache_root, &profile), &profile)
        .expect("Cannot handle cache");
}
