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

use std::env;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};
use toml::*;
use walkdir::WalkDir;

#[allow(unused_macros)]
macro_rules! log {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../Cargo.lock");
    let cargo_config = PathBuf::from(env!("CARGO_HOME")).join("config.toml");
    println!("cargo::rerun-if-changed={:?}", cargo_config);

    let profile = env::var("PROFILE").expect("Should have a PROFILE environment variable");

    let cache_root: PathBuf = env::current_dir().unwrap().join("../deps");

    let ultraplonk = UltraplonkDependency::new(&profile, cache_root);
    let libs_cached = cache_cpp_libs(&ultraplonk, &profile);

    set_env_paths(&ultraplonk, !libs_cached);
}

struct UltraplonkDependency {
    cache_root: PathBuf,
    profile: String,
}

impl UltraplonkDependency {
    fn new(profile: &str, cache_root: PathBuf) -> Self {
        Self { profile: profile.to_owned(), cache_root }
    }

    fn is_valid_lib_file(&self, p: &Path) -> bool {
        if let Some((name, ext)) = p.file_name()
            .and_then(OsStr::to_str)
            .and_then(|n| p.extension().map(|e| (n, e) )) {
            ext == "a" && name.starts_with("lib")
        } else {
            false
        }
    }

    fn is_valid_libs_folder(&self, folder: &Path) -> bool {
        if let Ok(walker) = fs::read_dir(folder) {
            walker.filter_map(Result::ok).map(|d| d.path())
                .any(|p| self.is_valid_lib_file(&p))
        } else {
            false
        }
        
    }

    fn is_valid_source_folder(&self, ultraplonk_dir: &Path) -> bool {
        self.is_valid_libs_folder(&self.source_folder(ultraplonk_dir))
    }

    fn source_folder(&self, ultraplonk_dir: &Path) -> PathBuf {
        ultraplonk_dir.join("out").join("build").join("lib")
    }
}

impl Dependency for UltraplonkDependency {
    fn cache_path(&self) -> PathBuf {
        self.cache_root.join("ultraplonk").join(&self.profile)
    }

    fn env_key(&self) -> &str {
        match self.profile.as_str() {
            "production" => "BARRETENBERG_LIB_DIR_RELEASE",
            "release" => "BARRETENBERG_LIB_DIR_RELEASE",
            "debug" => "BARRETENBERG_LIB_DIR_DEBUG",
            _ => "BARRETENBERG_LIB_DIR",
        }
    }

    fn env_value(&self) -> String {
        self.cache_path().canonicalize().expect("Should create absolute path").display().to_string()
    }

    fn is_valid_cache(&self) -> bool {
        let cache = self.cache_path();
        fs::metadata(&cache).is_ok() && self.is_valid_libs_folder(&cache)
    }

    fn folder_match(&self, path: &Path) -> bool {
        matches!(path.file_name().and_then(OsStr::to_str) , 
            Some(p) if 
                p.starts_with("ultraplonk_verifier-") && 
                self.source_folder(path).is_dir() &&
                self.is_valid_source_folder(path))
    }

    fn cache(&self, source: &Path) -> Result<(), std::io::Error> {
        let dest = self.cache_path();
        std::fs::create_dir_all(&dest)?;
        for entry in fs::read_dir(self.source_folder(source))? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let path  = entry.path();
            if ty.is_file() && self.is_valid_lib_file(&path) {
                fs::copy(&path, dest.join(entry.file_name()))?;
            }
    }
        Ok(())
    }
}

trait Dependency {
    fn cache_path(&self) -> PathBuf;
    fn env_key(&self) -> &str;
    fn env_value(&self) -> String;
    fn is_valid_cache(&self) -> bool;
    fn folder_match(&self, path: &Path) -> bool;
    fn cache(&self, source: &Path) -> Result<(), std::io::Error>;
}

fn cache_cpp_libs(dependency: &impl Dependency, profile: &str) -> bool {
    let target_path = format!("../target/{profile}/build");

    if !dependency.is_valid_cache() {
        // We ignore the error because doesn't matter if the cache folder doesn't exist.
        let _ = fs::remove_dir_all(dependency.cache_path());
        for entry in WalkDir::new(target_path).max_depth(1).into_iter().flatten() {
            let path = entry.path();
            if dependency.folder_match(path) {
                dependency
                    .cache(path)
                    .expect("Unable to copy dependency");
                return true;
            }
        }
        false
    } else {
        true
    }
}

#[derive(Default)]
struct EnvMap(Table);

impl From<&Table> for EnvMap {
    fn from(value: &Table) -> Self {
        Self(value.to_owned())
    }
}

impl EnvMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, dependency: &impl Dependency) {
        self.0.insert(
            dependency.env_key().to_owned(),
            dependency.env_value().into(),
        );
    }

    pub fn remove(&mut self, dependency: &impl Dependency) {
        self.0.remove(dependency.env_key());
    }

    pub fn store(self, table: &mut Table) {
        table.insert("env".to_owned(), toml::Value::Table(self.0));
    }
}

fn set_env_paths(dependency: &impl Dependency, reset: bool) {
    let cargo_config = PathBuf::from(env!("CARGO_HOME")).join("config.toml");

    if !Path::new(&cargo_config).exists() {
        File::create(cargo_config.clone()).expect("Unable to create file");
    }

    let mut file = File::open(cargo_config.clone()).expect("Unable to open file");
    let mut contents = String::new();
    File::read_to_string(&mut file, &mut contents).expect("Unable to read file");

    let mut main_table = contents.parse::<Table>().unwrap();
    let mut env_table = main_table
        .get("env")
        .and_then(Value::as_table)
        .map(EnvMap::from)
        .unwrap_or_default();
    if reset {
        env_table.remove(dependency);
    } else {
        env_table.add(dependency);
    }
    env_table.store(&mut main_table);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cargo_config.clone())
        .unwrap();
    file.write_all(main_table.to_string().as_bytes()).unwrap();
}
