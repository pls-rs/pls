use crate::config::EntryConst;
use crate::exc::Exc;
use clap::ValueEnum;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::FileType;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

lazy_static! {
	pub static ref ALL_TYP: Vec<Typ> = Typ::value_variants()
		.iter()
		.copied()
		.filter(|variant| variant != &Typ::All && variant != &Typ::None)
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

	None, // shorthand: no node types
	All,  // shorthand: all node types

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

impl TryFrom<&Path> for Typ {
	type Error = Exc;

	fn try_from(value: &Path) -> Result<Self, Self::Error> {
		value
			.symlink_metadata()
			.map(|meta| meta.file_type().into())
			.map_err(Self::Error::Io)
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
				Typ::None => cleaned.clear(),
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

	/// Get the category of the node from the type of the node.
	///
	/// The category is a broader classification of nodes into either directory
	/// or file.
	pub fn cat(&self) -> Typ {
		match self {
			Typ::Dir => Typ::Dir,
			_ => Typ::File,
		}
	}

	/// Get the style directives associated with the node type.
	///
	/// These directives are combined with directives from other sources to form
	/// the full picture of how the node should be styled.
	pub fn directives<'conf>(&self, entry_const: &'conf EntryConst) -> &'conf String {
		&entry_const.typ.get(self).unwrap().style
	}

	/* Name components */
	/* =============== */

	/// Get the icon associated with the node's type.
	///
	/// Note that this function only returns the name of the icon and not the
	/// glyph itself, to prevent an unnecessary lookup when the fallback is not
	/// needed and to keep the code for the lookups centralised in one place.
	///
	/// This icon is used as a fallback in cases where no other icon is found
	/// for the node from matching specs.
	pub fn icon<'conf>(&self, entry_const: &'conf EntryConst) -> &'conf Option<String> {
		&entry_const.typ.get(self).unwrap().icon
	}

	/// Get the suffix associated with the nodes type.
	///
	/// The suffix is combined with the name and inherits the
	/// [`directives`](Node::directives) that style the node's name.
	///
	/// Note that it is similar to but not the same as the type character,
	/// [`ch`](Typ::ch). Suffixes exist for a subset of types and are symbols.
	///
	/// This function returns a marked-up string.
	pub fn suffix<'conf>(&self, entry_const: &'conf EntryConst) -> &'conf String {
		&entry_const.typ.get(self).unwrap().suffix
	}

	/* Renderables */
	/* =========== */

	/// Get the type character that's displayed in the 'T' column.
	///
	/// Typically the char is a single letter that represents the node's type.
	///
	/// Note that it is similar to but not the same as [`suffix`](Typ::suffix).
	/// Type chars exist for each type and are letters.
	///
	/// This function returns a marked-up string.
	pub fn ch(&self, entry_const: &EntryConst) -> String {
		let ch = &entry_const.typ.get(self).unwrap().ch;
		let directives = self.directives(entry_const);
		format!("<{directives}>{ch}</>")
	}
}

#[cfg(test)]
mod tests {
	use super::Typ;
	use crate::config::EntryConst;

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

	macro_rules! make_name_components_test {
        ( $($name:ident: $typ:expr => $icon:expr, $suffix:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let entry_const = EntryConst::default();
                    assert_eq!($typ.icon(&entry_const), $icon);
                    assert_eq!($typ.suffix(&entry_const), $suffix);
                }
            )*
        };
    }

	make_name_components_test!(
		test_icon_suffix_for_dir: Typ::Dir => &Some(String::from("dir")), "<dimmed>/</>",
		test_icon_suffix_for_symlink: Typ::Symlink => &Some(String::from("symlink")), "<dimmed>@</>",
		test_icon_suffix_for_fifo: Typ::Fifo => &None, "<dimmed>|</>",
		test_icon_suffix_for_socket: Typ::Socket => &None, "<dimmed>=</>",
		test_icon_suffix_for_block_device: Typ::BlockDevice => &None, "",
		test_icon_suffix_for_char_device: Typ::CharDevice => &None, "",
		test_icon_suffix_for_file: Typ::File => &None, "",
	);

	macro_rules! make_renderables_test {
        ( $($name:ident: $typ:expr => $ch:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let entry_const = EntryConst::default();
                    assert_eq!($typ.ch(&entry_const), $ch);
                }
            )*
        };
    }

	make_renderables_test!(
		test_ch_for_dir: Typ::Dir => "<blue>d</>",
		test_ch_for_symlink: Typ::Symlink => "<>l</>",
		test_ch_for_fifo: Typ::Fifo => "<>p</>",
		test_ch_for_socket: Typ::Socket => "<>s</>",
		test_ch_for_block_device: Typ::BlockDevice => "<>b</>",
		test_ch_for_char_device: Typ::CharDevice => "<>c</>",
		test_ch_for_file: Typ::File => "<><dimmed>f</></>",
	);
}
