use crate::enums::{DetailField, Oct, Sym, SymState, Typ};
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use time::format_description::{self, OwnedFormatItem};

#[derive(Serialize, Deserialize)]
pub struct EntryConst {
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
	/// style for magnitude and unit (prefix and base) of node size
	pub size_styles: SizeStyles,
	/// style for the number of blocks occupied by the file
	pub blocks_style: String,
	/// mapping of timestamp fields to the human-readable format
	pub timestamp_formats: HashMap<DetailField, String>,
	/// mapping of symlink state to more symlink state info (including style)
	pub symlink: HashMap<SymState, SymlinkInfo>,

	/// lazily-parsed counterpart of [`timestamp_formats`](Self::timestamp_formats)
	///
	/// Parsing a format description is relatively expensive, so each format is
	/// parsed at most once for the lifetime of the config rather than once per
	/// node per timestamp column.
	#[serde(skip)]
	timestamp_formats_parsed: OnceLock<HashMap<DetailField, OwnedFormatItem>>,
}

impl EntryConst {
	/// Get the parsed format description for the given timestamp field.
	///
	/// The full set of formats is parsed once on first access and cached, so
	/// subsequent timestamps reuse the already-parsed format.
	pub fn timestamp_format(&self, field: DetailField) -> Option<&OwnedFormatItem> {
		self.timestamp_formats_parsed
			.get_or_init(|| {
				self.timestamp_formats
					.iter()
					.filter_map(
						|(&field, fmt)| match format_description::parse_owned::<2>(fmt) {
							Ok(parsed) => Some((field, parsed)),
							Err(err) => {
								warn!("Could not parse timestamp format for {field:?}: {err}");
								None
							}
						},
					)
					.collect()
			})
			.get(&field)
	}

	/// Move each node type's built-in icons behind the user's own choices.
	///
	/// Configs are combined with Figment's *adjoining* merge, which appends
	/// each layer's array to the previous one. Since the built-in defaults form
	/// the base layer, they always end up in front of any user additions —
	/// the opposite of what we want, as the first icon that resolves wins. This
	/// detaches that known default prefix and rotates it to the back, so every
	/// user icon (in config order) precedes the built-ins, which remain as a
	/// fallback. Figment cannot express this "prepend" directly, hence the
	/// post-merge fix-up.
	pub fn prioritize_user_icons(&mut self) {
		let defaults = Self::default();
		for (typ, info) in self.typ.iter_mut() {
			let Some(default_icons) = defaults.typ.get(typ).and_then(|d| d.icons.as_deref()) else {
				continue;
			};
			let Some(icons) = info.icons.as_mut() else {
				continue;
			};
			// Only rotate when the list still leads with the built-in prefix,
			// i.e. the user actually added icons on top of the defaults.
			if icons.len() > default_icons.len() && &icons[..default_icons.len()] == default_icons {
				icons.rotate_left(default_icons.len());
			}
		}
	}
}

impl Default for EntryConst {
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
				(Typ::Dir, "d", "<dimmed>/</>", "dir", "blue"),
				(Typ::Symlink, "l", "<dimmed>@</>", "symlink", ""),
				(Typ::Fifo, "p", "<dimmed>|</>", "fifo", ""),
				(Typ::Socket, "s", "<dimmed>=</>", "socket", ""),
				(Typ::BlockDevice, "b", "", "block_device", ""),
				(Typ::CharDevice, "c", "", "char_device", ""),
				(Typ::File, "<dimmed>f</>", "", "file", ""),
				(Typ::Unknown, "<red>?</>", "", "unknown", ""),
			]
			.into_iter()
			.map(|(k, ch, suffix, icon, style)| {
				(
					k,
					TypInfo {
						ch: ch.to_string(),
						suffix: suffix.to_string(),
						icons: Some(vec![String::from(icon)]),
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
				(SymState::Ok, "󰁔", "magenta", ""), // nf-md-arrow_right
				(SymState::Broken, "󱞣", "red", "strikethrough"), // nf-md-arrow_down_right
				(SymState::Cyclic, "󰑙", "yellow", ""), // nf-md-replay
				(SymState::Error, "󰜺", "red", ""),  // nf-md-cancel
			]
			.into_iter()
			.map(|(k, sep, style, ref_style)| {
				(
					k,
					SymlinkInfo {
						sep: sep.to_string(),
						style: style.to_string(),
						ref_style: ref_style.to_string(),
					},
				)
			})
			.collect(),
			timestamp_formats_parsed: OnceLock::new(),
		}
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

impl NlinkStyles {
	pub fn format(&self, nlink: u64, typ: &Typ) -> String {
		let directives = match (typ, nlink) {
			(Typ::Dir, 1) => &self.dir_sing,
			(Typ::Dir, _) => &self.dir_plur,
			(_, 1) => &self.file_sing,
			_ => &self.file_plur,
		};
		format!("<{directives}>{nlink}</>")
	}
}

#[derive(Serialize, Deserialize)]
pub struct TypInfo {
	/// the character for a node type, used in the 'T' column
	pub ch: String,
	/// the suffix for a node type, placed after the node name
	pub suffix: String,
	/// the fallback icon for the node type, used if no other icon is found
	pub icons: Option<Vec<String>>, // not all node types need to have an icon
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SymlinkInfo {
	/// the separator to show between the node and its target
	pub sep: String,
	/// the style to use for symlinks in a particular symlink state
	pub style: String, // applies to name and separator
	/// the style to use for the symlink reference
	pub ref_style: String, // applies to reference only
}

#[cfg(test)]
mod tests {
	use super::EntryConst;
	use crate::enums::Typ;

	/// Set the merged icon list for a node type, mimicking the state left by
	/// Figment's adjoining merge (built-in prefix followed by user additions).
	fn set_icons(ec: &mut EntryConst, typ: Typ, icons: &[&str]) {
		ec.typ.get_mut(&typ).unwrap().icons = Some(icons.iter().map(|s| s.to_string()).collect());
	}

	fn icons(ec: &EntryConst, typ: Typ) -> Vec<String> {
		ec.typ.get(&typ).unwrap().icons.clone().unwrap()
	}

	#[test]
	fn test_moves_single_default_behind_user_icon() {
		let mut ec = EntryConst::default();
		let default = icons(&ec, Typ::Dir); // built-in prefix
		set_icons(&mut ec, Typ::Dir, &["dir", "theme:_folder"]);
		ec.prioritize_user_icons();

		let mut expected = vec![String::from("theme:_folder")];
		expected.extend(default);
		assert_eq!(icons(&ec, Typ::Dir), expected);
	}

	#[test]
	fn test_preserves_order_of_multiple_user_icons() {
		let mut ec = EntryConst::default();
		set_icons(&mut ec, Typ::Dir, &["dir", "theme:a", "theme:b", "glyph"]);
		ec.prioritize_user_icons();

		assert_eq!(
			icons(&ec, Typ::Dir),
			vec!["theme:a", "theme:b", "glyph", "dir"]
		);
	}

	#[test]
	fn test_leaves_untouched_default_only_list() {
		let mut ec = EntryConst::default();
		let default = icons(&ec, Typ::Dir);
		ec.prioritize_user_icons();
		assert_eq!(icons(&ec, Typ::Dir), default);
	}
}
