use crate::enums::{DetailField, Oct, Sym, Typ};
use std::collections::HashMap;

pub struct Constants {
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
	/// configuration for the table view
	pub table: TableInfo,
}

impl Default for Constants {
	fn default() -> Self {
		Self {
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
			table: TableInfo {
				header_style: String::from("bold italic"),
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
		}
	}
}

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

pub struct OwnerStyles {
	/// style for when the node is owned by the current user/group
	pub curr: String,
	/// style for when the node is owned by a different user/group
	pub other: String,
}

pub struct SizeStyles {
	/// style for the node size magnitude
	pub mag: String,
	/// style for the node size unit prefix
	pub prefix: String,
	/// style for the node size base unit
	pub base: String,
}

pub struct TableInfo {
	/// mapping of detail field to column name
	pub column_names: HashMap<DetailField, String>,
	/// the styles to apply to the text in the header row
	pub header_style: String,
}
