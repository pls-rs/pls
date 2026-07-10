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
	resolve("XDG_DATA_HOME", ".local/share")
}

/// Get the directory under which `pls` stores its config, e.g. `pls.yml`.
pub fn config_dir() -> Option<PathBuf> {
	resolve("XDG_CONFIG_HOME", ".config")
}

/// Get the directory under which `pls` stores its cache, e.g. rendered icons.
pub fn cache_dir() -> Option<PathBuf> {
	resolve("XDG_CACHE_HOME", ".cache")
}

// =======
// Private
// =======

/// Compute the `pls` config, data and cache directory.
///
/// # Arguments
///
/// * `var` - the name of the XDG base directory environment variable
/// * `typical` - the typical location of the XDG base directory under `$HOME`
fn resolve(var: &str, typical: &str) -> Option<PathBuf> {
	env::home_dir().map(|home| {
		env::var(var)
			.ok()
			.filter(|dir| !dir.is_empty())
			.map(|dir| PathBuf::from(dir).join("pls"))
			.or_else(|| {
				let typical_xdg = home.join(typical);
				typical_xdg.is_dir().then(|| typical_xdg.join("pls"))
			})
			.unwrap_or_else(|| home.join(".pls"))
	})
}

#[cfg(test)]
// `Jail::expect_with` requires a closure returning the large `figment::Error`.
#[allow(clippy::result_large_err)]
mod tests {
	use super::*;
	use figment::Jail;

	/// Isolate the environment and point `$HOME` at the jail's temp directory.
	///
	/// `env::home_dir` reads `$HOME` on Unix, so overriding it lets us control
	/// where [`resolve`] looks for the typical XDG directories.
	fn setup(jail: &mut Jail) -> PathBuf {
		jail.clear_env();
		let home = jail.directory().to_path_buf();
		jail.set_env("HOME", home.display());
		home
	}

	/// Generate one test per branch of [`resolve`], using a generic env var and
	/// typical directory since `resolve` treats them opaquely.
	///
	/// * `xdg` - the value to set the env var to, or `None` to leave it unset
	/// * `typical_exists` - whether to create the typical directory
	/// * `expected` - a closure mapping `&home` to the expected path
	macro_rules! resolve_tests {
		($($name:ident: xdg = $xdg:expr, typical_exists = $exists:expr, expected = $expected:expr;)*) => {
			$(
				#[test]
				fn $name() {
					Jail::expect_with(|jail| {
						let home = setup(jail);
						let (var, typical) = ("XDG_TEST_HOME", ".typical");
						if $exists {
							jail.create_dir(typical)?;
						}
						if let Some(value) = $xdg {
							jail.set_env(var, value);
						}
						let expected: fn(&PathBuf) -> PathBuf = $expected;
						assert_eq!(resolve(var, typical).unwrap(), expected(&home));
						Ok(())
					});
				}
			)*
		};
	}

	resolve_tests! {
		env_var_takes_precedence_over_typical_dir:
			xdg = Some("/custom/xdg"), typical_exists = true,
			expected = |_| PathBuf::from("/custom/xdg").join("pls");
		empty_env_var_is_ignored:
			xdg = Some(""), typical_exists = true,
			expected = |home| home.join(".typical").join("pls");
		typical_dir_used_when_env_var_unset:
			xdg = None::<&str>, typical_exists = true,
			expected = |home| home.join(".typical").join("pls");
		dot_pls_fallback_when_typical_dir_missing:
			xdg = None::<&str>, typical_exists = false,
			expected = |home| home.join(".pls");
	}

	/// Generate a pair of tests per public directory function, checking that each
	/// reads its own env var and falls back to its own typical directory.
	macro_rules! public_fn_tests {
		($($mod:ident: $func:ident, $var:expr, $typical:expr;)*) => {
			$(
				mod $mod {
					use super::*;

					#[test]
					fn reads_env_var() {
						Jail::expect_with(|jail| {
							setup(jail);
							jail.set_env($var, "/xdg");
							assert_eq!($func().unwrap(), PathBuf::from("/xdg").join("pls"));
							Ok(())
						});
					}

					#[test]
					fn uses_typical_dir() {
						Jail::expect_with(|jail| {
							let home = setup(jail);
							jail.create_dir($typical)?;
							assert_eq!($func().unwrap(), home.join($typical).join("pls"));
							Ok(())
						});
					}
				}
			)*
		};
	}

	public_fn_tests! {
		config: config_dir, "XDG_CONFIG_HOME", ".config";
		data: data_dir, "XDG_DATA_HOME", ".local/share";
		cache: cache_dir, "XDG_CACHE_HOME", ".cache";
	}
}
