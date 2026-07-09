//! This module contains code for locating `pls`'s config, data and cache
//! directories.
//!
//! The logic is consistently applied as follows:
//!
//! * If the `XDG_CONFIG_HOME`, `XDG_DATA_HOME` or `XDG_CACHE_HOME` env vars are
//!   set, we will use them directly, which is very respectful to the user.
//! * If the env vars are not set, but their conventional locations exist, we
//!   will use them, and assume the user forgot to set the env vars.
//! * If neither of the above is true, we will fall back to `~/.pls`, which is
//!   our last resort. The home folder is usually full of these things.
//!
//! The public interface of the module consists of:
//!
//! * [`config_dir`]
//! * [`data_dir`]
//! * [`cache_dir`]

use std::env;
use std::path::PathBuf;

/// Get the directory under which `pls` stores its data, e.g. icon packs.
pub fn data_dir() -> Option<PathBuf> {
	resolve(xdg_dir("XDG_DATA_HOME", ".local/share"), env::home_dir())
}

/// Get the directory under which `pls` stores its config, e.g. `pls.yml`.
pub fn config_dir() -> Option<PathBuf> {
	resolve(xdg_dir("XDG_CONFIG_HOME", ".config"), env::home_dir())
}

/// Get the directory under which `pls` stores its cache, e.g. rendered icons.
pub fn cache_dir() -> Option<PathBuf> {
	resolve(xdg_dir("XDG_CACHE_HOME", ".cache"), env::home_dir())
}

// =======
// Private
// =======

/// Resolve an XDG base directory from an env var, with a home-relative default.
///
/// The default is only used if it actually exists on disk, matching the
/// convention that an implicit XDG location is honoured only on systems that
/// follow it.
///
/// # Arguments
///
/// * `var` - the name of the env var to read
/// * `home_default` - the fallback path, relative to the home directory
fn xdg_dir(var: &str, home_default: &str) -> Option<PathBuf> {
	match env::var(var) {
		Ok(dir) if !dir.is_empty() => Some(PathBuf::from(dir)),
		_ => {
			let default = env::home_dir()?.join(home_default);
			default.is_dir().then_some(default)
		}
	}
}

/// Compute the `pls` directory from an XDG base and the home directory.
///
/// Returns `<xdg>/pls` when an XDG base is given, else `<home>/.pls`. Split out
/// from [`data_dir`] and [`config_dir`] so the resolution is testable without
/// touching the real environment.
///
/// # Arguments
///
/// * `xdg` - the XDG base directory, if available
/// * `home` - the home directory, if available
fn resolve(xdg: Option<PathBuf>, home: Option<PathBuf>) -> Option<PathBuf> {
	match xdg {
		Some(dir) => Some(dir.join("pls")),
		None => Some(home?.join(".pls")),
	}
}

#[cfg(test)]
mod tests {
	use super::resolve;
	use std::path::PathBuf;

	#[test]
	fn test_prefers_xdg_dir() {
		let dir = resolve(Some("/xdg/data".into()), Some(PathBuf::from("/home/u")));
		assert_eq!(dir, Some(PathBuf::from("/xdg/data/pls")));
	}

	#[test]
	fn test_falls_back_to_home_pls() {
		let dir = resolve(None, Some(PathBuf::from("/home/u")));
		assert_eq!(dir, Some(PathBuf::from("/home/u/.pls")));
	}

	#[test]
	fn test_no_home_no_xdg_is_none() {
		assert_eq!(resolve(None, None), None);
	}
}
