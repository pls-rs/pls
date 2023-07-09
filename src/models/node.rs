use crate::config::{Args, Conf};
use crate::enums::Typ;
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
	pub fn display_name(&self, conf: &Conf, args: &Args) -> String {
		self.name.clone()
	}
}
