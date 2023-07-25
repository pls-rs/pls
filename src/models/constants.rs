use crate::enums::{DetailField, Oct, Sym, SymState, Typ};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Constants {
	/// style for the device number
	pub dev_style: String,
	/// style for the inode number
	pub ino_style: String,
	/// styles for the number of hard links
	pub nlink_styles: NlinkStyles,
	/// mapping of node type to node type info (including style)
	pub typ: HashMap<Typ, TypInfo>,
	/// mapping of symbolic permission bits to style
	pub perm_styles: HashMap<Sym, String>,
	/// mapping of octal permission bits to style
	pub oct_styles: HashMap<Oct, String>,
	/// styles for the owner user
	pub user_styles: OwnerStyles,
	/// styles for the owner group
	pub group_styles: OwnerStyles,
	/// style for magnitude and unit of node size
	pub size_styles: SizeStyles,
	/// style for the number of blocks occupied by the file
	pub blocks_style: String,
	/// mapping of timestamp fields to the human-readable format
	pub timestamp_formats: HashMap<DetailField, String>,
	/// mapping of symlink state to more symlink state info (including style)
	pub symlink: HashMap<SymState, SymlinkInfo>,
	/// configuration for the table view
	pub table: TableInfo,
	/// pairings of importance levels with styling directives
	pub imp_styles: Vec<(i8, String)>,

	/// mapping of importance levels to styling directives, derived from `imp`
	#[serde(skip)]
	pub imp_map: HashMap<i8, String>,
}

impl Default for Constants {
	fn default() -> Self {
		Self {
			dev_style: String::default(),
			ino_style: String::default(),
			nlink_styles: NlinkStyles {
				file_sing: String::from(""),
				file_plur: String::from("yellow"),
				dir_sing: String::from("yellow"),
				dir_plur: String::from(""),
			},
			typ: [
				(Typ::Dir, "d", "<dimmed>/</>", Some("dir"), "blue"),
				(Typ::Symlink, "l", "<dimmed>@</>", Some("symlink"), ""),
				(Typ::Fifo, "p", "<dimmed>|</>", None, ""),
				(Typ::Socket, "s", "<dimmed>=</>", None, ""),
				(Typ::BlockDevice, "b", "", None, ""),
				(Typ::CharDevice, "c", "", None, ""),
				(Typ::File, "<dimmed>f</>", "", None, ""),
				(Typ::Unknown, "<red>?</>", "", None, ""),
			]
			.into_iter()
			.map(|(k, ch, suffix, icon, style)| {
				(
					k,
					TypInfo {
						ch: ch.to_string(),
						suffix: suffix.to_string(),
						icon: icon.map(String::from),
						style: style.to_string(),
					},
				)
			})
			.collect(),
			perm_styles: [
				(Sym::None, "dimmed"),
				(Sym::Read, "yellow"),
				(Sym::Write, "red"),
				(Sym::Execute, "green"),
				(Sym::Special, "magenta"),
			]
			.into_iter()
			.map(|(k, v)| (k, v.to_string()))
			.collect(),
			oct_styles: [
				(Oct::Special, "magenta"),
				(Oct::User, "blue"),
				(Oct::Group, "blue dimmed"),
				(Oct::Other, "dimmed"),
			]
			.into_iter()
			.map(|(k, v)| (k, v.to_string()))
			.collect(),
			user_styles: OwnerStyles {
				curr: String::from("blue bold"),
				other: String::from("dimmed"),
			},
			group_styles: OwnerStyles {
				curr: String::from("blue"),
				other: String::from("dimmed"),
			},
			size_styles: SizeStyles {
				mag: String::from("bold"),
				prefix: String::default(),
				base: String::from("dimmed"),
			},
			blocks_style: String::default(),
			timestamp_formats: [
				(DetailField::Btime, "green"),
				(DetailField::Ctime, "yellow"),
				(DetailField::Mtime, "yellow"),
				(DetailField::Atime, "blue"),
			]
			.into_iter()
			.map(|(k, v)| {
				(
					k,
					format!(
						"<bold {v}>[year]-[month repr:short]-[day]</> \
						 [hour repr:12]:[minute][period case:lower]"
					),
				)
			})
			.collect(),
			symlink: [
				(SymState::Ok, "󰁔", "magenta"),    // nf-md-arrow_right
				(SymState::Broken, "󱞣", "red"),    // nf-md-arrow_down_right
				(SymState::Cyclic, "󰑙", "yellow"), // nf-md-replay
				(SymState::Error, "󰜺", "red"),     // nf-md-cancel
			]
			.into_iter()
			.map(|(k, sep, style)| {
				(
					k,
					SymlinkInfo {
						sep: sep.to_string(),
						style: style.to_string(),
					},
				)
			})
			.collect(),
			table: TableInfo {
				header_style: String::from("bold italic underline"),
				column_names: [
					(DetailField::Dev, "Device"),
					(DetailField::Ino, "inode"),
					(DetailField::Nlink, "Link#"),
					(DetailField::Typ, "T"),
					(DetailField::Perm, "Permissions"),
					(DetailField::Oct, "SUGO"),
					(DetailField::User, "User"),
					(DetailField::Uid, "UID"),
					(DetailField::Group, "Group"),
					(DetailField::Gid, "GID"),
					(DetailField::Size, "Size"),
					(DetailField::Blocks, "Blocks"),
					(DetailField::Btime, "Created"),
					(DetailField::Ctime, "Changed"),
					(DetailField::Mtime, "Modified"),
					(DetailField::Atime, "Accessed"),
					(DetailField::Git, "Git"),
					(DetailField::Name, "Name"),
				]
				.into_iter()
				.map(|(k, v)| (k, v.to_string()))
				.collect(),
			},
			imp_styles: [(-1, "dimmed"), (1, "italic"), (2, "underline")]
				.into_iter()
				.map(|(k, v)| (k, v.to_string()))
				.collect(),

			imp_map: HashMap::new(), // set in Constants::set_imp_map
		}
	}
}

impl Constants {
	/// Set `imp_map` from `imp` and then use it to clean the latter.
	///
	/// If a level has been defined multiple times, only the last definition
	/// will be retained in `imp_map`. The levels will be sorted by importance
	/// in `imp`.
	pub fn massage_imps(&mut self) {
		self.imp_map = self
			.imp_styles
			.iter()
			.map(|(k, v)| (*k, v.to_string()))
			.collect();

		self.imp_styles = self.imp_map.clone().into_iter().collect();
		self.imp_styles.sort_by_cached_key(|entry| entry.0);
	}

	/// Get the lowest configured importance level, i.e. zeroth index in `imp`.
	pub fn min_imp(&self) -> i8 {
		self.get_imp(0)
	}

	/// Get the highest configured importance level, i.e. last index in `imp`.
	pub fn max_imp(&self) -> i8 {
		self.get_imp(self.imp_styles.len() - 1)
	}

	/// Get the importance level at the given index.
	///
	/// This returns 0 (default for `i8`) if the `imp` vector does not have the
	/// given index.
	fn get_imp(&self, idx: usize) -> i8 {
		self.imp_styles
			.get(idx)
			.map(|row| row.0)
			.unwrap_or_default()
	}
}

#[derive(Serialize, Deserialize)]
pub struct NlinkStyles {
	/// style to use when file has one hard link
	pub file_sing: String,
	/// style to use when file has more than one hard link
	pub file_plur: String,
	/// style to use when directory has one hard link
	pub dir_sing: String,
	/// style to use when directory has more than one hard link
	pub dir_plur: String,
}

#[derive(Serialize, Deserialize)]
pub struct TypInfo {
	/// the character for a node type, used in the 'T' column
	pub ch: String,
	/// the suffix for a node type, placed after the node name
	pub suffix: String,
	/// the fallback icon for the node type, used if no other icon is found
	pub icon: Option<String>, // not all node types need to have an icon
	/// the style to use for nodes of a particular node type
	pub style: String, // applies to name, `ch`, `suffix` and `icon`
}

#[derive(Serialize, Deserialize)]
pub struct OwnerStyles {
	/// style for when the node is owned by the current user/group
	pub curr: String,
	/// style for when the node is owned by a different user/group
	pub other: String,
}

#[derive(Serialize, Deserialize)]
pub struct SizeStyles {
	/// style for the node size magnitude
	pub mag: String,
	/// style for the node size unit prefix
	pub prefix: String,
	/// style for the node size base unit
	pub base: String,
}

#[derive(Serialize, Deserialize)]
pub struct SymlinkInfo {
	/// the separator to show between the node and its target
	pub sep: String,
	/// the style to use for symlinks in a particular symlink state
	pub style: String, // applies to name and `arrow`
}

#[derive(Serialize, Deserialize)]
pub struct TableInfo {
	/// mapping of detail field to column name
	pub column_names: HashMap<DetailField, String>,
	/// the styles to apply to the text in the header row
	pub header_style: String,
}

#[cfg(test)]
mod tests {
	use super::Constants;
	use std::collections::HashMap;

	macro_rules! make_massage_imps_test {
        ( $($name:ident: $imp:expr => $exp_imp:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let imp_styles: Vec<(i8, String)> = $imp
                        .into_iter()
                        .map(|(k, v): (i8, &str)| (k, v.to_string()))
                        .collect();
                    let exp_imp: Vec<(i8, String)> = $exp_imp
                        .into_iter()
                        .map(|(k, v): (i8, &str)| (k, v.to_string()))
                        .collect();
                    let exp_imp_map: HashMap<i8, String> = $exp_imp
                        .into_iter()
                        .map(|(k, v): (i8, &str)| (k, v.to_string()))
                        .collect();

                    let mut constants = Constants {
                        imp_styles,
                        ..Constants::default()
                    };
                    constants.massage_imps();
                    assert_eq!(constants.imp_styles, exp_imp);
                    assert_eq!(constants.imp_map, exp_imp_map);
                }
            )*
        }
    }

	make_massage_imps_test!(
		test_empty: vec![] => vec![],
		test_sorting: vec![
			(2, "underline"),
			(1, "italic"),
		] => vec![(1, "italic"), (2, "underline")],
		test_deduplication: vec![
			(2, "bold"),
			(2, "dimmed"),
			(1, "reversed"),
			(2, "underline"),
			(1, "italic"),
		] => vec![(1, "italic"), (2, "underline")],
	);
}
