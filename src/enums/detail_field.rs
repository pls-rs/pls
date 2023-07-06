use serde::{Deserialize, Serialize};

/// This enum contains all the metadata about a node that can be provided by a
/// UNIX-like operating system.
///
/// It also contains the following variants:
/// * shorthands for none, the standard set and all of the details
/// * a name variant that is mandatory as that is the entire point of `pls`
///
/// The `DetailField` variants are closely related to the
/// [`SortField`](crate::enums::SortField) variants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailField {
	Dev,   // device ID
	Ino,   // inode number
	Nlink, // number of hard links
	Typ,   // node type

	Perm, // symbolic permissions
	Oct,  // octal permissions

	User, // user name
	Uid,  // user ID

	Group, // group name
	Gid,   // group ID

	Size, // storage space

	Btime, // created at; "b" for birth
	Ctime, // changed at; originally meant "created at"
	Mtime, // modified at
	Atime, // accessed at

	Git, // git status

	None, // shorthand: no details
	Std,  // shorthand: the standard set of details
	All,  // shorthand: all details

	Name, // node name (not a CLI argument)
}
