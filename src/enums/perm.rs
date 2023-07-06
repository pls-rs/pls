use serde::{Deserialize, Serialize};

/// This enum contains different types of permissions defined on nodes, in a
/// UNIX-like operating system, as they would appear in the symbolic notation.
///
/// Note that in a symbolic triplet, `Execute` and `Special` are both expressed
/// combined in the third character.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Perm {
	None, // no permissions
	Read,
	Write,
	Execute,
	Special, // setuid, setgid or sticky
}
