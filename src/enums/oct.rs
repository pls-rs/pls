use serde::{Deserialize, Serialize};

/// This enum contains different groups of permissions defined on nodes, in a
/// UNIX-like operating system, as they would appear in the octal notation.
///
/// Note that while the values of `Special` cause changes to user, group and
/// other permissions, they are all stored in fourth digit of the octal number.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Oct {
	Special,
	User,
	Group,
	Other,
}
