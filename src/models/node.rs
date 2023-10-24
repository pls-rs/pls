use crate::config::{AppConst, Args, Conf, EntryConst};
use crate::enums::{Appearance, Collapse, DetailField, Typ};
use crate::models::{OwnerMan, Spec};
use crate::traits::{Detail, Imp, Name, Sym};
use std::collections::HashMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::Metadata;
use std::io::Result as IoResult;
use std::iter::once;
use std::path::{Path, PathBuf};

pub struct Node<'pls> {
	/// the name of the node on the file system, determined from the path and
	/// lossily converted into a string
	pub name: String,
	/// the name of the node to show to the user, equal to the `name` unless
	/// overridden by an appearance mode
	pub display_name: String,

	pub path: PathBuf,
	meta: IoResult<Metadata>,
	pub typ: Typ, // `Typ::Unknown` if `meta` is `Err`

	pub appearance: Appearance,

	pub specs: Vec<&'pls Spec>,

	pub collapse_name: Option<String>,
	pub children: Vec<Node<'pls>>,
}

impl<'pls> Node<'pls> {
	// ===========
	// Constructor
	// ===========

	pub fn new(path: &Path) -> Self {
		let name = path
			.file_name()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string();
		let display_name = name.clone();

		let path = path.to_owned();
		let meta = path.symlink_metadata();
		let typ = path.as_path().try_into().unwrap_or(Typ::Unknown);

		Self {
			name,
			display_name,
			path,
			meta,
			typ,
			appearance: Appearance::Normal,
			specs: vec![],
			collapse_name: None,
			children: vec![],
		}
	}

	// ===========
	// Appearances
	// ===========

	/// Get the `Node` instance with the given name hardcoded.
	///
	/// This function consumes the given `Node` and returns a new instance with
	/// the name hardcoded. It should be used to change the name to something
	/// different from the name derived from the path.
	pub fn solo_file(self, name: String) -> Self {
		Self {
			display_name: name,
			appearance: Appearance::SoloFile,
			..self
		}
	}

	/// Get the `Node` instance with the given name hardcoded.
	///
	/// This function consumes the given `Node` and returns a new instance with
	/// the name hardcoded. It should be used to change the name to something
	/// different from the name derived from the path.
	pub fn symlink(self, name: String) -> Self {
		Self {
			display_name: name,
			appearance: Appearance::Symlink,
			..self
		}
	}

	/// Get the `Node` instance with some tree-drawing characters.
	///
	/// This function consumes the given `Node` and returns a new instance with
	/// the appearance configured with the tree shapes. It is used to make the
	/// node the child of another node.
	pub fn tree_child(self) -> Self {
		Self {
			appearance: Appearance::TreeChild,
			..self
		}
	}

	/// Get the `Node` instance with children populated.
	pub fn tree_parent(self, children: Vec<Node<'pls>>) -> Self {
		Self {
			children,
			appearance: Appearance::TreeParent,
			..self
		}
	}

	// =======
	// Getters
	// =======

	/// Get the metadata of the node if it was successfully retrieved.
	pub fn meta_ok(&self) -> Option<&Metadata> {
		self.meta.as_ref().ok()
	}

	// =========
	// Mutations
	// =========

	/// Link the current node with all the specs that apply to it, based on
	/// whether the spec's `pattern` matches with this node's name.
	pub fn match_specs(&mut self, all_specs: &'pls [Spec]) {
		self.specs = all_specs
			.iter()
			.filter(|spec| spec.pattern.is_match(self.name.as_bytes()))
			.collect();
	}

	/// Find the name of the node against which this node will collapse.
	///
	/// If the collapse uses a name, use that name.
	/// If the collapse uses an ext, use this node's stem with that ext.
	pub fn find_collapse(&mut self) {
		let collapse = self
			.specs
			.iter()
			.rev()
			.find(|spec| spec.collapse.is_some())
			.and_then(|spec| spec.collapse.clone());
		self.collapse_name = match collapse {
			Some(Collapse::Name(name)) => Some(name),
			Some(Collapse::Ext(ext)) => Some(format!("{}.{}", self.stem(), ext)),
			None => None,
		}
	}

	// ===========
	// Aggregators
	// ===========

	/// Get all styling directives applicable to the node.
	///
	/// A node can get its style directives from two sources:
	///
	/// * the node's type
	/// * specs associated with the node
	fn directives(&self, app_const: &AppConst, entry_const: &EntryConst, args: &Args) -> String {
		let mut directives = String::from(self.typ.directives(entry_const));

		if self.appearance != Appearance::Symlink {
			let imp_dir = Imp::directives(self, app_const, args);
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

	// ===============
	// Name components
	// ===============

	/// Get the icon associated with the node.
	///
	/// A node can get its icon from two sources:
	///
	/// * specs associated with the node
	/// * the node's type
	fn icon(&self, conf: &Conf, entry_const: &EntryConst) -> String {
		self.specs
			.iter()
			.rev()
			.find(|spec| spec.icon.is_some())
			.and_then(|spec| spec.icon.as_ref())
			.or_else(|| self.typ.icon(entry_const).as_ref())
			.and_then(|icon_name| conf.icons.get(icon_name).cloned())
			.unwrap_or_default()
	}

	// ===========
	// Renderables
	// ===========

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
	pub fn display_name(
		&self,
		conf: &Conf,
		app_const: &AppConst,
		entry_const: &EntryConst,
		args: &Args,
		tree_shapes: &[&str],
	) -> String {
		let text_directives = self.directives(app_const, entry_const, args);
		let icon_directives = text_directives.replace("underline", "");

		let mut parts = String::default();

		// Tree shape
		if Appearance::TreeChild == self.appearance {
			let offset = " ".repeat(if args.align { 3 } else { 2 });
			parts.push_str(&tree_shapes.iter().fold(String::new(), |mut acc, shape| {
				let _ = write!(acc, "{offset}{shape}"); // `write!`-ing into a `String` can never fail.
				acc
			}));
		}

		// Icon
		if args.icon && self.appearance != Appearance::Symlink {
			parts.push_str(&format!(
				"<{icon_directives}>{:<1}</> ",
				self.icon(conf, entry_const),
			));
		}

		// Name and suffix
		parts.push_str(&format!("<{text_directives}>"));
		match self.appearance {
			Appearance::Symlink | Appearance::SoloFile => parts.push_str(&self.display_name),
			_ if args.align => parts.push_str(&self.aligned_name()),
			_ => parts.push_str(&self.display_name),
		}
		if args.suffix && self.appearance != Appearance::Symlink {
			// Symlink should not have suffix because it should show the path reference without modifications
			parts.push_str(self.typ.suffix(entry_const))
		};
		parts.push_str("</>");

		if args.sym {
			if let Some(target) = self.target() {
				parts.push_str(&target.print(conf, args));
			}
		}

		parts
	}

	// =============
	// Printer entry
	// =============

	fn get_value(
		&self,
		detail: DetailField,
		owner_man: &mut OwnerMan,
		entry_const: &EntryConst,
		args: &Args,
	) -> String {
		let val = match detail {
			// `Detail` trait
			DetailField::Dev => self.dev(entry_const),
			DetailField::Ino => self.ino(entry_const),
			DetailField::Nlink => self.nlink(entry_const),
			DetailField::Perm => self.perm(entry_const),
			DetailField::Oct => self.oct(entry_const),
			DetailField::User => self.user(owner_man, entry_const),
			DetailField::Uid => self.uid(owner_man, entry_const),
			DetailField::Group => self.group(owner_man, entry_const),
			DetailField::Gid => self.gid(owner_man, entry_const),
			DetailField::Btime => self.time(detail, entry_const),
			DetailField::Mtime => self.time(detail, entry_const),
			DetailField::Ctime => self.time(detail, entry_const),
			DetailField::Atime => self.time(detail, entry_const),
			DetailField::Size => self.size(entry_const, args),
			DetailField::Blocks => self.blocks(entry_const),
			// `Typ` enum
			DetailField::Typ => Some(self.typ.ch(entry_const)),
			_ => Some(String::default()),
		};
		val.unwrap_or_default()
	}

	/// Get a mapping of detail fields to their values.
	///
	/// This information is used to render the table row for a node.
	pub fn row(
		&self,
		owner_man: &mut OwnerMan,
		conf: &Conf,
		app_const: &AppConst,
		entry_const: &EntryConst,
		args: &Args,
		tree_shape: &[&str],
	) -> HashMap<DetailField, String> {
		args.details
			.iter()
			.map(|&detail| {
				if detail == DetailField::Name {
					(
						detail,
						self.display_name(conf, app_const, entry_const, args, tree_shape),
					)
				} else {
					(detail, self.get_value(detail, owner_man, entry_const, args))
				}
			})
			.collect()
	}

	/// Get a vector of mapping of detail fields to their values.
	///
	/// Each entry in the vector is a row that can be used to render a table.
	///
	#[allow(clippy::too_many_arguments)]
	pub fn entries(
		&self,
		owner_man: &mut OwnerMan,
		conf: &Conf,
		app_const: &AppConst,
		entry_const: &EntryConst,
		args: &Args,
		parent_shapes: &[&str],  // list of shapes inherited from the parent
		own_shape: Option<&str>, // shape to show just before the current node
	) -> Vec<HashMap<DetailField, String>> {
		// list of parent shapes to pass to the children
		let mut child_parent_shapes = parent_shapes.to_vec();

		// the complete set of shapes to print for the current node
		let mut all_shapes = parent_shapes.to_vec();

		if let Some(more_shape) = own_shape {
			child_parent_shapes.push(if more_shape == app_const.tree.tee_dash {
				// Current node is not the last of its parent, so child nodes
				// will have a pipe for continuity.
				&app_const.tree.pipe_space
			} else {
				// Current node is the last of its parent, so child nodes will
				// not have a pipe, but rather spaces for padding.
				&app_const.tree.space_space
			});
			all_shapes.push(more_shape);
		}

		once(self.row(owner_man, conf, app_const, entry_const, args, &all_shapes))
			.chain(self.children.iter().enumerate().flat_map(|(idx, child)| {
				let child_own_shape = if idx == self.children.len() - 1 {
					&app_const.tree.bend_dash
				} else {
					&app_const.tree.tee_dash
				};

				child.entries(
					owner_man,
					conf,
					app_const,
					entry_const,
					args,
					&child_parent_shapes,
					Some(child_own_shape),
				)
			}))
			.collect()
	}
}

impl Display for Node<'_> {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.name)
	}
}
