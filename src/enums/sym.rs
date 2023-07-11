use serde::{Deserialize, Serialize};

/// This enum contains the three states a symlink can be in, out of which one is
/// fine and two are problematic.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymState {
	Ok,
	Broken,
	Cyclic,
}
