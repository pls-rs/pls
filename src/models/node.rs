use crate::config::{Args, Conf};
use crate::enums::{Appearance, DetailField, Typ};
use crate::models::{OwnerMan, Spec};
use crate::traits::{Detail, Imp, Name, Sym};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::Metadata;
use std::path::{Path, PathBuf};

pub struct Node<'pls> {
	pub name: String, // lossy

	pub path: PathBuf,
	pub meta: Metadata,
	pub typ: Typ,

	pub appearance: Appearance,

	pub specs: Vec<&'pls Spec>,
}

impl<'pls> Node<'pls> {
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
			appearance: Appearance::Normal,
			specs: vec![],
		}
	}

	/// Get the `Node` instance with the given name hardcoded.
	///
	/// This function consumes the given `Node` and returns a new instance with
	/// the name hardcoded. It should be used to change the name to something
	/// different from the name derived from the path.
	pub fn symlink(self, name: String) -> Self {
		Self {
			name,
			appearance: Appearance::Symlink,
			..self
		}
	}

	/* Mutations */
	/* ========= */

	/// Link the current node with all the specs that apply to it, based on
	/// whether the spec's `pattern` matches with this node's name.
	pub fn match_specs(&mut self, all_specs: &'pls [Spec]) {
		self.specs = all_specs
			.iter()
			.filter(|spec| spec.pattern.is_match(self.name.as_bytes()))
			.collect();
	}

	/* Aggregators */
	/* =========== */

	/// Get all styling directives applicable to the node.
	///
	/// A node can get its style directives from two sources:
	///
	/// * the node's type
	/// * specs associated with the node
	fn directives(&self, conf: &Conf, args: &Args) -> String {
		let mut directives = String::from(self.typ.directives(conf));

		if self.appearance != Appearance::Symlink {
			let imp_dir = Imp::directives(self, conf, args);
			if let Some(directive) = imp_dir {
				directives.push(' ');
				directives.push_str(&directive);
			}
		}

		for &spec in &self.specs {
			if let Some(style) = &spec.style {
				directives.push(' ');
				directives.push_str(style);
			}
		}

		directives
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
		self.specs
			.iter()
			.rev()
			.find(|spec| spec.icon.is_some())
			.and_then(|spec| spec.icon.as_ref())
			.or_else(|| self.typ.icon(conf).as_ref())
			.and_then(|icon_name| conf.icons.get(icon_name).cloned())
			.unwrap_or_default()
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
	pub fn display_name(&self, conf: &Conf, args: &Args) -> String {
		let text_directives = self.directives(conf, args);
		let icon_directives = text_directives.replace("underline", "");

		let mut parts = String::default();

		// Icon
		if args.icon && self.appearance != Appearance::Symlink {
			parts.push_str(&format!("<{icon_directives}>{:<1}</> ", self.icon(conf)));
		}

		// Name and suffix
		parts.push_str(&format!("<{text_directives}>"));
		if args.align && self.appearance != Appearance::Symlink {
			parts.push_str(&self.aligned_name());
		} else {
			parts.push_str(&self.name);
		};
		if args.suffix {
			parts.push_str(self.typ.suffix(conf))
		};
		parts.push_str("</>");

		if args.sym {
			if let Some(target) = self.target() {
				parts.push_str(&target.print(conf, args));
			}
		}

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

impl Display for Node<'_> {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.name)
	}
}
