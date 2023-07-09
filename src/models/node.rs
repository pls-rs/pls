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

	/* Renderables */
	/* =========== */

	/// Get the display name of the node.
	pub fn display_name(&self, conf: &Conf, args: &Args) -> String {
		self.name.clone()
	}
}
