//! This module provides a trait [`Abs`], that can be used to extend `Path` and
//! `PathBuf` with a method `abs` that converts a path to an absolute path.

use std::env::current_dir;
use std::path::{Path, PathBuf};

// =====
// Trait
// =====

/// This trait provides a method `abs` that can be used to convert a path
/// to an absolute path.
pub trait Abs {
	/// Convert the given path to an absolute path.
	///
	/// This function is appends the path to the current working directory if it
	/// is not already absolute and if the current working directory can be
	/// determined. In all other cases, the path will be returned as-is.
	fn abs(&self) -> PathBuf;
}

// ===============
// Implementations
// ===============

impl Abs for Path {
	fn abs(&self) -> PathBuf {
		abs(self)
	}
}

impl Abs for PathBuf {
	fn abs(&self) -> PathBuf {
		abs(self)
	}
}

// =======
// Private
// =======

fn abs<P>(path: P) -> PathBuf
where
	P: AsRef<Path>,
{
	let path = path.as_ref();
	if !path.is_absolute() {
		if let Ok(cwd) = current_dir() {
			return cwd.join(path);
		}
	}
	path.to_path_buf()
}
