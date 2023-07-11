use serde::{Deserialize, Serialize};

/// This enum contains the four states a symlink can be in, out of which one is
/// fine and the rest are problematic.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymState {
	Ok,
	Broken,
	Cyclic,
	Error,
}
