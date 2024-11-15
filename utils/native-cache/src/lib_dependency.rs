use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::Dependency;

/// This trait define a concrete [`LibFilesDependency`] implementation.
/// Here you should define:
///  - How to identify a target lib candidate
///  - How to validate a the library file.to cache
///  - Where search the library files
pub trait DependencyImpl {
    /// Return if the given path is a target lib candidate or not.
    fn is_target_lib_candidate(p: &str) -> bool;
    /// Checks if the given path is a valid library file to cache.
    fn validate_lib_file(p: &Path) -> bool;
    /// Return the path where the source libraries are located starting
    /// from the target folder candidate..
    fn source_libs_folder(folder: &Path) -> PathBuf;
}

/// A concrete implementation of [`Dependency`] trait designed for
/// the cases where you would cache your lib files dependency. `
pub struct LibFilesDependency<I: DependencyImpl> {
    cache_path: PathBuf,
    env_key: String,
    _dependency_impl: PhantomData<I>,
}

/// This builder can be used to build [`LibFilesDependency`] by define
/// a profile to env-folder tuple map. After the builder is defined you can
/// create a [`LibFilesDependency`] instance given the cache root and the
/// active profile.
pub struct ProfileLibFilesDependencyBuilder<I: DependencyImpl> {
    cache_path: PathBuf,
    default_env: String,
    map_envs: HashMap<String, String>,
    _dependency_impl: PhantomData<I>,
}

impl<I: DependencyImpl> ProfileLibFilesDependencyBuilder<I> {
    /// Create a new builder with given cache path and default env.
    /// - `cache_path`: the relative path to the cache folder used for this dependency.
    /// - `default_env`: the default environment variable name.
    pub fn new(cache_path: impl AsRef<Path>, default_env: &str) -> Self {
        Self {
            cache_path: cache_path.as_ref().to_owned(),
            default_env: default_env.to_owned(),
            map_envs: HashMap::new(),
            _dependency_impl: PhantomData,
        }
    }

    /// Build a [`LibFilesDependency`] instance given the cache root and the active profile.
    pub fn build(
        &self,
        cache_root: impl AsRef<Path>,
        profile: impl AsRef<str>,
    ) -> LibFilesDependency<I> {
        let cache_root = cache_root.as_ref();
        let profile = profile.as_ref();
        LibFilesDependency {
            cache_path: cache_root.join(self.cache_path.as_path()).join(profile),
            env_key: self.env_key(profile).to_owned(),
            _dependency_impl: PhantomData,
        }
    }

    /// Add a profile to env mapping.
    pub fn with_env(mut self, profile: &str, env_name: &str) -> Self {
        self.map_envs
            .insert(profile.to_lowercase(), env_name.to_owned());
        self
    }

    fn env_key(&self, profile: &str) -> &str {
        self.map_envs.get(profile).unwrap_or(&self.default_env)
    }
}

impl<I: DependencyImpl> LibFilesDependency<I> {
    pub fn new(cache_path: &Path, env_key: &str) -> Self {
        let cache_path = cache_path.to_owned();
        let env_key = env_key.to_owned();
        Self {
            cache_path,
            env_key,
            _dependency_impl: PhantomData,
        }
    }

    fn is_valid_lib_file(&self, p: &Path) -> bool {
        I::validate_lib_file(p)
    }

    fn is_valid_libs_folder(&self, folder: &Path) -> bool {
        if let Ok(walker) = std::fs::read_dir(folder) {
            walker
                .filter_map(Result::ok)
                .map(|d| d.path())
                .any(|p| self.is_valid_lib_file(&p))
        } else {
            false
        }
    }

    fn is_valid_source_folder(&self, folder: &Path) -> bool {
        self.is_valid_libs_folder(&self.source_folder(folder))
    }

    fn source_folder(&self, folder: &Path) -> PathBuf {
        I::source_libs_folder(folder)
    }
}

impl<I: DependencyImpl> Dependency for LibFilesDependency<I> {
    fn default_cache_path(&self) -> &Path {
        &self.cache_path
    }

    fn env_key(&self) -> &str {
        &self.env_key
    }

    fn is_valid_cache(&self, cache: &Path) -> bool {
        fs::metadata(cache).is_ok() && self.is_valid_libs_folder(cache)
    }

    fn cache_files(&self, source: &Path, dest: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(self.source_folder(source))? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let path = entry.path();
            if ty.is_file() && self.is_valid_lib_file(&path) {
                fs::copy(&path, dest.join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    fn folder_match(&self, path: &Path) -> bool {
        matches!(path.file_name().and_then(OsStr::to_str) ,
        Some(p) if I::is_target_lib_candidate(p) &&
            self.source_folder(path).is_dir() &&
            self.is_valid_source_folder(path))
    }

    fn rerun_if(&self, cache: &Path) {
        println!("cargo::rerun-if-changed={}", cache.display());
        println!("cargo::rerun-if-env-changed={}", self.env_key());
    }
}
