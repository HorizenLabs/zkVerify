use std::path::Path;

/// The trait that define how to cache a dependency.
pub trait Dependency {
    /// The path where to cache the dependency files.
    fn default_cache_path(&self) -> &Path;
    /// The environment key value.
    fn env_key(&self) -> &str;
    /// Check if the cache folder is still valid.
    fn is_valid_cache(&self, cache: &Path) -> bool;
    /// Return true if the given `path`` is a valid source folder for the dependency.
    fn folder_match(&self, path: &Path) -> bool;
    /// Cache the dependency files in the `source` folder .
    fn cache_files(&self, source: &Path, dest: &Path) -> Result<(), std::io::Error>;
    /// Emit the rerun rules for this dependency
    fn rerun_if(&self, cache: &Path);
}

/// Blanket implementation for `Box<dyn Dependency>`
impl<T: Dependency + ?Sized> Dependency for Box<T> {
    fn default_cache_path(&self) -> &Path {
        self.as_ref().default_cache_path()
    }

    fn env_key(&self) -> &str {
        self.as_ref().env_key()
    }

    fn is_valid_cache(&self, cache: &Path) -> bool {
        self.as_ref().is_valid_cache(cache)
    }

    fn folder_match(&self, path: &Path) -> bool {
        self.as_ref().folder_match(path)
    }

    fn cache_files(&self, source: &Path, dest: &Path) -> Result<(), std::io::Error> {
        self.as_ref().cache_files(source, dest)
    }

    fn rerun_if(&self, cache: &Path) {
        self.as_ref().rerun_if(cache)
    }
}

/// Useful to box a [`Dependency`].
pub trait Boxed {
    fn boxed(self) -> Box<dyn Dependency + 'static>;
}

impl<D: Dependency + 'static> Boxed for D {
    fn boxed(self) -> Box<dyn Dependency + 'static> {
        Box::new(self)
    }
}
