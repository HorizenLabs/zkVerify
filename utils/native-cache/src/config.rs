use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use toml::{Table, Value};

use crate::{log, Dependency};

/// Struct to handle cargo config.toml file
pub struct Config {
    path: PathBuf,
    dirty: bool,
    body: Table,
}

impl Config {
    /// Load the cargo config.toml file
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        if !path.as_ref().exists() {
            log!("creating config file at: {}", path.as_ref().display());
            File::create(path.as_ref())?;
        }
        let mut file = File::open(path.as_ref()).context("Cannot open config file")?;
        let mut contents = String::new();
        File::read_to_string(&mut file, &mut contents).context("Cannot read config file")?;

        let body = contents
            .parse::<Table>()
            .context("Cannot parse config file")?;

        Ok(Config {
            path: path.as_ref().to_path_buf(),
            dirty: false,
            body,
        })
    }

    fn envs(&self) -> Option<&Table> {
        self.body
            .get("env")
            .map(|e| e.as_table().expect("[env] if exists should be a table"))
    }

    fn envs_mut(&mut self) -> &mut Table {
        self.body
            .entry("env")
            .or_insert_with(|| {
                log!("inserting envs table if it doesn't exist");
                self.dirty = true;
                Table::default().into()
            })
            .as_table_mut()
            .expect("[env] if exists should be a table")
    }

    fn inner_add(envs: &mut Table, dependency: &impl Dependency, value: impl AsRef<str>) -> bool {
        let k = dependency.env_key().to_owned();
        let v: Value = value.as_ref().to_owned().into();

        if Some(&v) != envs.get(&k) {
            log!("inserting envs value changed: {:?} -> {}", envs.get(&k), v);
            envs.insert(k, v);
            true
        } else {
            false
        }
    }

    /// Store the config.toml file.
    pub fn store_force(&mut self) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path.as_path())
            .context("Cannot open the config file to save it")?;
        file.write_all(self.body.to_string().as_bytes())
            .context("Cannot save config file")?;

        self.dirty = false;
        Ok(())
    }

    /// Add a dependency to the cargo config.toml file.
    pub fn get(&self, dependency: &impl Dependency) -> Option<String> {
        self.envs()
            .and_then(|envs| envs.get(dependency.env_key()))
            .map(ToString::to_string)
    }

    /// Add a dependency to the cargo config.toml file.
    pub fn add(&mut self, dependency: &impl Dependency, value: impl AsRef<str>) {
        self.dirty |= Self::inner_add(self.envs_mut(), dependency, value);
    }

    /// Remove a dependency to the cargo config.toml file.
    pub fn remove(&mut self, dependency: &impl Dependency) {
        let old = self.dirty;
        self.dirty |= self.envs_mut().remove(dependency.env_key()).is_some();
        if self.dirty != old {
            log!("Removed env: {}", dependency.env_key());
        }
    }

    /// Store the config.toml file if and only if is dirty.
    pub fn store(&mut self) -> anyhow::Result<()> {
        if self.dirty {
            self.store_force()?;
        }
        Ok(())
    }
}
