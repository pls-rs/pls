use crate::config::Conf;
use clap::ValueEnum;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::FileType;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

lazy_static! {
	pub static ref ALL_TYP: Vec<Typ> = Typ::value_variants()
		.iter()
		.copied()
		.filter(|variant| variant != &Typ::All)
		.collect();
}

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
#[derive(
	Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ValueEnum,
)]
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

	#[clap(skip)]
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

impl Typ {
	/// Clean the given input.
	///
	/// This performs the following operations on the input:
	///
	/// * Expand all shorthand forms.
	/// * Sort by their order in the enum.
	/// * Remove duplicated values.
	pub fn clean(input: &[Self]) -> Vec<Self> {
		let mut cleaned = vec![];
		for node_type in input {
			match node_type {
				Typ::All => {
					cleaned.clear();
					cleaned.extend_from_slice(&ALL_TYP);
				}
				_ => cleaned.push(*node_type),
			}
		}
		cleaned.sort(); // Use the order of the `Typ` enum.
		cleaned.dedup(); // Only removes consecutive duplicates, so sort first.
		cleaned
	}

	/* Renderables */
	/* =========== */

	/// Get the node type's character that's used in the 'T' column.
	///
	/// This function returns a marked-up string.
	pub fn ch(&self, conf: &Conf) -> String {
		let ch = &conf.constants.typ[self].ch;
		let directives = &conf.constants.typ[self].style;
		format!("<{directives}>{ch}</>")
	}

	/// Get the node's suffix that placed after the node name.
	///
	/// This function returns a marked-up string.
	pub fn suffix(&self, conf: &Conf) -> String {
		let suffix = &conf.constants.typ[self].suffix;
		let directives = &conf.constants.typ[self].style;
		format!("<{directives}>{suffix}</>")
	}
}

#[cfg(test)]
mod tests {
	use super::Typ;
	use crate::config::Conf;

	macro_rules! make_clean_test {
		( $($name:ident: $input:expr => $expected:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(Typ::clean($input), $expected);
                }
            )*
		};
	}

	make_clean_test!(
		test_expands_shorthand: &[Typ::All] => vec![
			Typ::Dir,
			Typ::Symlink,
			Typ::Fifo,
			Typ::Socket,
			Typ::BlockDevice,
			Typ::CharDevice,
			Typ::File,
		],
		test_sorts_by_enum_order: &[Typ::Fifo, Typ::Dir] => vec![
			Typ::Dir,
			Typ::Fifo,
		],
		test_removes_duplicates: &[Typ::Fifo, Typ::Fifo, Typ::Dir, Typ::Fifo] => vec![
			Typ::Dir,
			Typ::Fifo,
		],
	);

	macro_rules! make_renderables_test {
        ( $($name:ident: $typ:expr => $expected_ch:expr, $expected_suffix:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let conf = Conf::default();
                    assert_eq!($typ.ch(&conf), $expected_ch);
                    assert_eq!($typ.suffix(&conf), $expected_suffix);
                }
            )*
        };
    }

	make_renderables_test!(
		test_dir: Typ::Dir => "<blue>d</>", "<blue><dimmed>/</></>",
		test_symlink: Typ::Symlink => "<>l</>", "<><dimmed>@</></>",
		test_fifo: Typ::Fifo => "<>p</>", "<><dimmed>|</></>",
		test_socket: Typ::Socket => "<>s</>", "<><dimmed>=</></>",
		test_block_device: Typ::BlockDevice => "<>b</>", "<></>",
		test_char_device: Typ::CharDevice => "<>c</>", "<></>",
		test_file: Typ::File => "<><dimmed>f</></>", "<></>",
	);
}
