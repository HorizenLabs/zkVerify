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

use regex::Regex;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};
use toml::*;
use walkdir::WalkDir;

fn main() {
    let cargo_config = PathBuf::from(env!("CARGO_HOME")).join("config.toml");
    println!("cargo::rerun-if-changed={:?}", cargo_config);
    println!("cargo:rustc-link-arg=-fuse-ld=lld");

    generate_cargo_keys();

    rerun_if_git_head_changed();

    let libs_cached = cache_cpp_libs();

    set_env_paths(!libs_cached);
}

fn cache_cpp_libs() -> bool {
    let rocksdb_lib = "librocksdb.a";
    let snappy_lib = "libsnappy.a";

    let target_paths = [
        "../target/production/build",
        "../target/release/build",
        "../target/debug/build",
    ];
    let mut libs = [
        // (pattern, copied)
        (
            format!("{}{}", "librocksdb-sys-.*/out/", rocksdb_lib),
            false,
        ),
        (format!("{}{}", "librocksdb-sys-.*/out/", snappy_lib), false),
    ];
    let destination_path = "../deps/";

    if fs::metadata(PathBuf::from(destination_path).join(rocksdb_lib)).is_err()
        || fs::metadata(PathBuf::from(destination_path).join(snappy_lib)).is_err()
    {
        for target_path in target_paths {
            for entry in WalkDir::new(target_path).into_iter().flatten() {
                let path = entry.path();
                for lib in libs.iter_mut() {
                    let regex_pattern = Regex::new(&lib.0).unwrap();
                    if path.is_file() && regex_pattern.is_match(path.to_str().unwrap()) {
                        let destination_file =
                            PathBuf::from(destination_path).join(path.file_name().unwrap());
                        fs::create_dir_all(destination_path).expect("Unable to create directories");
                        fs::copy(path, destination_file).expect("Unable to copy file");
                        lib.1 = true;
                        break;
                    }
                }
                if libs[0].1 && libs[1].1 {
                    return true;
                }
            }
        }
    } else {
        return true;
    }

    libs[0].1 && libs[1].1
}

fn set_env_paths(reset: bool) {
    let libs_path: PathBuf = env::current_dir().unwrap().join("../deps");
    let libs_path = libs_path.to_str().unwrap();
    let cargo_config = PathBuf::from(env!("CARGO_HOME")).join("config.toml");

    if !Path::new(&cargo_config).exists() {
        File::create(cargo_config.clone()).expect("Unable to create file");
    }

    let mut file = File::open(cargo_config.clone()).expect("Unable to open file");
    let mut contents = String::new();
    File::read_to_string(&mut file, &mut contents).expect("Unable to read file");

    let mut main_table = contents.parse::<Table>().unwrap();
    if let Some(env) = main_table.get("env") {
        let env_table = env.as_table();
        if let Some(env_table) = env_table {
            let mut env_table = env_table.to_owned();
            if !reset {
                env_table.insert("ROCKSDB_LIB_DIR".to_owned(), libs_path.into());
                env_table.insert("SNAPPY_LIB_DIR".to_owned(), libs_path.into());
            } else {
                env_table.remove("ROCKSDB_LIB_DIR");
                env_table.remove("SNAPPY_LIB_DIR");
            }

            main_table.insert("env".to_owned(), toml::Value::Table(env_table));
        }
    } else if !reset {
        let mut env_table = Table::new();
        env_table.insert("ROCKSDB_LIB_DIR".to_owned(), libs_path.into());
        env_table.insert("SNAPPY_LIB_DIR".to_owned(), libs_path.into());

        main_table.insert("env".to_owned(), toml::Value::Table(env_table));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cargo_config.clone())
        .unwrap();
    file.write_all(main_table.to_string().as_bytes()).unwrap();
}
