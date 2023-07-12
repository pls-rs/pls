use crate::config::{Args, Conf};
use crate::exc::Exc;
use crate::models::Node;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// This enum contains the four states a symlink can be in, out of which one is
/// fine and the rest are problematic.
///
/// This enum is a unitary enum intended only for use as a `HashMap` key when
/// defining the constants in the config.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymState {
	Ok,
	Broken,
	Cyclic,
	Error,
}

impl From<&SymTarget<'_>> for SymState {
	fn from(value: &SymTarget) -> Self {
		match value {
			SymTarget::Ok(_) => SymState::Ok,
			SymTarget::Broken(_) => SymState::Broken,
			SymTarget::Cyclic(_) => SymState::Cyclic,
			SymTarget::Error(_) => SymState::Error,
		}
	}
}

/// This enum is an extension of [`SymState`] that, in addition to the four
/// states of a symlink, also contains additional data relevant to the state.
///
/// * `Ok` contains a [`Node`] instance wrapping the target path.
/// * `Broken` and `Cyclic` contain the target path as a [`PathBuf`] instance.
/// * `Error` contains the raised [`std::io::Error`] instance.
pub enum SymTarget<'node> {
	Ok(Node<'node>), // Valid targets should print like `Node`s.
	Broken(PathBuf), // Invalid targets should be kept as-is.
	Cyclic(PathBuf), // Target is self, so there is nothing to print.
	Error(Exc),      // Target cannot be determined.
}

impl<'node> SymTarget<'node> {
	/// Print the symlink target.
	pub fn print(&self, conf: &Conf, args: &Args) -> String {
		let state = self.into();
		let sym_conf = conf.constants.symlink.get(&state).unwrap();
		let directives = &sym_conf.style;
		let sep = &sym_conf.sep;

		match self {
			SymTarget::Ok(node) => {
				let path = node.display_name(conf, args);
				format!(" <{directives}>{sep}</> {path}")
			}
			SymTarget::Broken(path) | SymTarget::Cyclic(path) => {
				let path = path.to_string_lossy().to_string();
				format!(" <{directives}>{sep} {path}</>")
			}
			SymTarget::Error(exc) => {
				format!(" <{directives}>{sep} {}</>", exc.to_string())
			}
		}
	}
}
