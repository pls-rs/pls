use crate::config::{Conf, ConfMan};
use crate::enums::Typ;
use crate::exc::Exc;
use crate::ext::Abs;
use log::debug;
use std::path::{Path, PathBuf};

// ======
// Models
// ======

/// Represents one path entered in the CLI.
///
/// The path entered in the CLI can be a file or a directory. The path may be a
/// symlink, which should not be resolved, and treated as a file, even if it
/// points to a directory.
pub struct Input {
	/// the path as entered in the CLI
	pub path: PathBuf,

	/// the absolute version of the path;
	/// This version prefixes the CWD if necessary and resolves `.` and `..`
	pub abs: PathBuf,
	pub typ: Typ,

	/// the config associated with this path
	pub conf: Conf,
}

// ===============
// Implementations
// ===============

impl std::fmt::Debug for Input {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Input")
			.field("path", &self.path)
			.field("abs", &self.abs)
			.field("typ", &self.typ)
			.finish()
	}
}

impl Input {
	pub fn new(path: &Path, conf_man: &ConfMan) -> Result<Self, Exc> {
		let path_buf = path.to_path_buf();
		let abs = path.abs();

		let typ = path.try_into()?;

		let mut conf = conf_man.get(Some(&path))?;
		debug!("{path:?} {:?}", conf.specs);
		conf.app_const.massage_imps();

		Ok(Self {
			path: path_buf,
			abs,
			typ,
			conf,
		})
	}
}

#[cfg(test)]
mod tests {
	use crate::enums::Typ;
	use std::path::PathBuf;

	#[test]
	fn test_relative() {
		let path = PathBuf::from("README.md");
		let typ: Typ = path.as_path().try_into().unwrap_or(Typ::Unknown);

		assert_eq!(typ, Typ::File);
	}
}
