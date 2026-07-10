use std::fs::read_dir;
use std::path::{Path, PathBuf};

// =====
// Trait
// =====

/// This trait provides a method `sub_dirs` that can be used to get a list of
/// directories under a given path.
pub trait SubDirs {
	/// Get a list of directories under this path.
	///
	/// This function returns an empty vector if the path does not exist or is not
	/// a directory.
	fn sub_dirs(&self) -> Vec<PathBuf>;
}

// ===============
// Implementations
// ===============

impl SubDirs for Path {
	fn sub_dirs(&self) -> Vec<PathBuf> {
		sub_dirs(self)
	}
}

impl SubDirs for PathBuf {
	fn sub_dirs(&self) -> Vec<PathBuf> {
		sub_dirs(self)
	}
}

// =======
// Private
// =======

/// Get a list of directories under `root`.
fn sub_dirs<P>(path: P) -> Vec<PathBuf>
where
	P: AsRef<Path>,
{
	read_dir(path.as_ref())
		.into_iter()
		.flatten()
		.flatten()
		.filter_map(|entry| {
			let path = entry.path();
			path.is_dir().then_some(path)
		})
		.collect()
}
