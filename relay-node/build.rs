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

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use std::{env, path::PathBuf};

use native_cache::rocksdb;

fn main() {
    substrate_build_script_utils::generate_cargo_keys();
    // For the node/worker version check, make sure we always rebuild the node and binary workers
    // when the version changes.
    substrate_build_script_utils::rerun_if_git_head_changed();

    let profile = env::var("PROFILE").expect("Should have a PROFILE environment variable");
    let cache_root: PathBuf = env::current_dir().unwrap().join("../deps");
    native_cache::handle_dependencies("../target", &rocksdb(&cache_root, &profile), &profile)
        .expect("Cannot handle cache");
}
