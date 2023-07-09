use crate::config::{Args, Conf};
use crate::traits::Name;
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

		let icon = if args.icon {
			self.icon(conf)
		} else {
			String::default()
		};
		let name = if args.align {
			self.aligned_name()
		} else {
			self.name.clone()
		};
		let suffix = if args.suffix {
			self.typ.suffix(conf)
		} else {
			""
		};

		format!("<{icon_directives}>{icon}</> <{text_directives}>{name}{suffix}</>")
	}
}
