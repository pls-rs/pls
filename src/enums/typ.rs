use serde::{Deserialize, Serialize};
use std::fs::FileType;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

/// This enum contains different types of nodes that can be found on UNIX-like
/// operating systems.
///
/// It also contains the following variants:
/// * a shorthand that can be used to refer to any and all other variants
/// * an unknown variant that is used when a node type is unrecognised
///
/// The names for the variants are used in accordance with the naming scheme of
/// the [`FileType`] struct. A variant of this enum can be created using `into`
/// on `FileType`.
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

impl From<FileType> for Typ {
	fn from(value: FileType) -> Self {
		match value {
			_ if value.is_dir() => Typ::Dir,
			_ if value.is_symlink() => Typ::Symlink,
			_ if value.is_fifo() => Typ::Fifo,
			_ if value.is_socket() => Typ::Socket,
			_ if value.is_block_device() => Typ::BlockDevice,
			_ if value.is_char_device() => Typ::CharDevice,
			_ if value.is_file() => Typ::File,
			_ => Typ::Unknown,
		}
	}
}
