use crate::config::{Args, Conf};
use crate::enums::{DetailField, Typ};
use crate::models::OwnerMan;
use crate::traits::{Detail, Name};
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::{Path, PathBuf};

pub struct Node {
	pub name: String, // lossy

	pub path: PathBuf,
	pub meta: Metadata,
	pub typ: Typ,
}

impl Node {
	pub fn new(path: &Path) -> Self {
		let name = path
			.file_name()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string();

		let path = path.to_owned();
		let meta = path.symlink_metadata().unwrap();
		let typ = meta.file_type().into();

		Self {
			name,
			path,
			meta,
			typ,
		}
	}

	/* Aggregators */
	/* =========== */

	/// Get all styling directives applicable to the node.
	///
	/// This function aggregates the `directive` function across all traits.
	fn directives<'conf>(&self, conf: &'conf Conf, _args: &Args) -> &'conf String {
		self.typ.directives(conf)
	}

	/* Name components */
	/* =============== */

	/// Get the icon associated with the node.
	///
	/// A node can get its icon from two sources:
	///
	/// * specs associated with the node
	/// * the node's type
	fn icon(&self, conf: &Conf) -> String {
		if let Some(icon_name) = self.typ.icon(conf) {
			return conf.icons.get(icon_name).cloned().unwrap_or_default();
		}

		String::default()
	}

	/* Renderables */
	/* =========== */

	/// Get the display name of the node.
	///
	/// The display name of a node consists of the following parts:
	///
	/// * icon, based on the `--icons` CLI argument
	/// * actual name, aligned based on the `--align` CLI argument
	/// * suffix, based on the `--suffix` CLI argument
	/// * symlink target, based on the `--symlink` CLI argument
	///
	/// Additionally, the display name is marked up with the appropriate
	/// directives obtained from configuration values.
	fn display_name(&self, conf: &Conf, args: &Args) -> String {
		let text_directives = self.directives(conf, args);
		let icon_directives = self.directives(conf, args).replace("underline", "");

		let mut parts = String::default();

		// Icon
		if args.icon {
			parts.push_str(&format!("<{icon_directives}>{:<1}</> ", self.icon(conf)));
		}

		// Name and suffix
		parts.push_str(&format!("<{text_directives}>"));
		if args.align {
			parts.push_str(&self.aligned_name());
		} else {
			parts.push_str(&self.name);
		};
		if args.suffix {
			parts.push_str(self.typ.suffix(conf))
		};
		parts.push_str("</>");

		// TODO: Symlink target

		parts
	}

	/* Printer entry */
	/* ============= */

	fn get_value(
		&self,
		detail: DetailField,
		owner_man: &mut OwnerMan,
		conf: &Conf,
		args: &Args,
	) -> String {
		match detail {
			// `Detail` trait
			DetailField::Dev => self.dev(conf),
			DetailField::Ino => self.ino(conf),
			DetailField::Nlink => self.nlink(conf),
			DetailField::Perm => self.perm(conf),
			DetailField::Oct => self.oct(conf),
			DetailField::User => self.user(owner_man, conf),
			DetailField::Uid => self.uid(owner_man, conf),
			DetailField::Group => self.group(owner_man, conf),
			DetailField::Gid => self.gid(owner_man, conf),
			DetailField::Btime => self.time(detail, conf),
			DetailField::Mtime => self.time(detail, conf),
			DetailField::Ctime => self.time(detail, conf),
			DetailField::Atime => self.time(detail, conf),
			DetailField::Size => self.size(conf, args),
			DetailField::Blocks => self.blocks(conf),
			// `Typ` enum
			DetailField::Typ => self.typ.ch(conf),
			// `Node` struct
			DetailField::Name => self.display_name(conf, args),
			_ => String::default(),
		}
	}

	/// Get a mapping of detail fields to their values.
	///
	/// This information is used to render the table row for a node.
	pub fn row(
		&self,
		owner_man: &mut OwnerMan,
		conf: &Conf,
		args: &Args,
	) -> HashMap<DetailField, String> {
		args.details
			.iter()
			.map(|&detail| (detail, self.get_value(detail, owner_man, conf, args)))
			.collect()
	}
}
