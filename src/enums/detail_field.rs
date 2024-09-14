use crate::config::AppConst;
use crate::output::Cell;
use crate::utils::vectors::dedup;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Alignment;
use std::sync::LazyLock;

static STD_FIELDS: LazyLock<Vec<DetailField>> = LazyLock::new(|| {
	["nlink", "typ", "perm", "user", "group", "size", "mtime"]
		.into_iter()
		.filter_map(|item| DetailField::from_str(item, false).ok())
		.collect()
});
static ALL_FIELDS: LazyLock<Vec<DetailField>> = LazyLock::new(|| {
	DetailField::value_variants()
		.iter()
		.copied()
		.filter(|variant| {
			variant != &DetailField::None
				&& variant != &DetailField::Std
				&& variant != &DetailField::All
		})
		.collect()
});

/// This enum contains all the metadata about a node that can be provided by a
/// UNIX-like operating system.
///
/// It also contains the following variants:
/// * shorthands for none, the standard set and all of the details
/// * a name variant that is mandatory as that is the entire point of `pls`
///
/// The `DetailField` variants are closely related to those of
/// [`SortField`](crate::enums::SortField).
#[derive(
	Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ValueEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum DetailField {
	Dev,   // device ID
	Ino,   // inode number
	Nlink, // number of hard links
	Typ,   // node type

	Perm, // symbolic permissions
	Oct,  // octal permissions

	User, // owner user name
	Uid,  // owner user ID

	Group, // owner group name
	Gid,   // owner group ID

	Size,   // storage space
	Blocks, // number of blocks

	// Uses OS-normalized timestamp field
	// [`created`](std::fs::Metadata::created).
	Btime, // created at

	// Uses Unix-specific extension fields
	// [`ctime`](std::os::unix::fs::MetadataExt::ctime) and
	// [`ctime_nsec`](std::os::unix::fs::MetadataExt::ctime_nsec).
	Ctime, // changed at

	// Uses OS-normalized timestamp field
	// [`modified`](std::fs::Metadata::modified).
	Mtime, // modified at

	// Uses OS-normalized timestamp field
	// [`accessed`](std::fs::Metadata::accessed).
	Atime, // accessed at

	Git, // git status

	None, // shorthand: no details
	Std,  // shorthand: the standard set of details
	All,  // shorthand: all details

	#[clap(skip)]
	Name, // node name (not a CLI argument)
}

impl DetailField {
	/// Clean the given input.
	///
	/// This performs the following operations on the input:
	///
	/// * Expand all shorthand values.
	/// * Ensure that `DetailField::Name` is always present.
	/// * Sort values by their order in the enum.
	/// * Remove duplicated values.
	pub fn clean(input: &[Self]) -> Vec<Self> {
		let mut cleaned = vec![];
		for field in input {
			match field {
				DetailField::None => cleaned.clear(),
				DetailField::Std => cleaned.extend_from_slice(&STD_FIELDS),
				DetailField::All => {
					cleaned.clear(); // Reduce sorting and de-duplication burden.
					cleaned.extend_from_slice(&ALL_FIELDS);
				}
				_ => cleaned.push(*field),
			}
		}
		cleaned.push(DetailField::Name);
		cleaned = dedup(cleaned);
		cleaned
	}

	// =======
	// Getters
	// =======

	/// Get the [`Cell`] that should be used to display this field.
	///
	/// This cell is right-aligned for numeric fields, and left-aligned for all
	/// other fields. Fields with uniform width such as octal permissions and
	/// timestamps need not be aligned at all.
	pub fn cell(&self) -> Cell {
		let alignment = match self {
			DetailField::Dev
			| DetailField::Ino
			| DetailField::Nlink
			| DetailField::Oct
			| DetailField::Uid
			| DetailField::Gid
			| DetailField::Size
			| DetailField::Blocks => Alignment::Right,
			_ => Alignment::Left,
		};
		Cell::new(alignment, (0, 1))
	}

	/// Get whether each entry in the list is equally wide.
	///
	/// Computation of max-widths for uniformly wide columns is slightly faster
	/// because it only needs to compute the width of the cell in the first row.
	pub fn uniformly_wide(&self) -> bool {
		matches!(
			self,
			DetailField::Typ
				| DetailField::Oct
				| DetailField::Btime
				| DetailField::Ctime
				| DetailField::Mtime
				| DetailField::Atime
				| DetailField::Git
		)
	}

	// ===========
	// Renderables
	// ===========

	/// Get the name of the detail field to be used in the column header.
	///
	/// This function returns a marked-up string.
	pub fn name(&self, app_const: &AppConst) -> String {
		app_const.table.column_names.get(self).cloned().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::DetailField;

	macro_rules! make_clean_test {
		( $($name:ident: $input:expr => $expected:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(DetailField::clean($input), $expected);
                }
            )*
		};
	}

	make_clean_test!(
		test_expands_shorthand: &[DetailField::Std] => vec![
			DetailField::Nlink,
			DetailField::Typ,
			DetailField::Perm,
			DetailField::User,
			DetailField::Group,
			DetailField::Size,
			DetailField::Mtime,
			DetailField::Name,
		],
		test_none_clears: &[DetailField::Mtime, DetailField::None, DetailField::Gid] => vec![
			DetailField::Gid,
			DetailField::Name,
		],
		test_ensures_name_present: &[] => vec![
			DetailField::Name,
		],
		test_removes_duplicates: &[DetailField::Gid, DetailField::Gid, DetailField::User, DetailField::Gid] => vec![
			DetailField::User,
			DetailField::Gid,
			DetailField::Name,
		],
	);
}
