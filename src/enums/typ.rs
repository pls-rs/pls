use serde::{Deserialize, Serialize};

/// This enum contains different types of nodes that can be found on UNIX-like
/// operating systems.
///
/// It also contains the following variants:
/// * a shorthand that can be used to refer to any and all other variants
/// * an unknown variant that is used when a node type is unrecognised
///
/// The names for the variants are used in accordance with the naming scheme of
/// the [`std::fs::FileType`] struct.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Typ {
	Dir,         // regular folder
	Symlink,     // symbolic link
	Fifo,        // named pipe
	Socket,      // file-based socket
	BlockDevice, // block special device file
	CharDevice,  // character special device file
	File,        // regular file

	All, // shorthand: all types

	Unknown, // unrecognised type (not a CLI argument)
}
