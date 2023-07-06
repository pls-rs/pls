use serde::{Deserialize, Serialize};

/// This enum contains all the different ways two nodes can be compared to
/// determine the sorting order.
///
/// It also contains, for every variant, a corresponding variant with trailing
/// underscore that sorts in the opposite direction.
///
/// The `SortField` variants are closely related to the
/// [`DetailField`](crate::enums::DetailField) variants.
///
/// The normal sort order for alphabetical fields is A to Z. The natural sort
/// order for numeric fields is 0 to 9. Sort fields with trailing underscore
/// have the opposite of this behaviour.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
	Dev,   // device ID
	Ino,   // inode number
	Nlink, // number of hard links
	Typ,   // node type
	Cat,   // node category (directory or file)

	User, // user name
	Uid,  // user ID

	Group, // group name
	Gid,   // group ID

	Size, // storage space

	Btime, // created at; "b" for birth
	Ctime, // changed at; originally meant "created at"
	Mtime, // modified at
	Atime, // accessed at

	Name,  // node name
	Cname, // canonical name (name in lower case with leading symbols stripped)
	Ext,   // file extension

	// Reversed sort by the field
	Inode_,
	Nlinks_,
	Typ_,
	Cat_,
	User_,
	Uid_,
	Group_,
	Gid_,
	Size_,
	Btime_,
	Ctime_,
	Mtime_,
	Atime_,
	Name_,
	Cname_,
	Ext_,
}
