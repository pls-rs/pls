use crate::config::{AppConst, Conf, EntryConst};
use crate::enums::{Appearance, Collapse, DetailField, Icon, Typ};
use crate::models::{OwnerMan, Spec};
use crate::traits::{Detail, Imp, Name, Sym};
use crate::PLS;
use std::collections::{HashMap, HashSet};
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

	pub appearances: HashSet<Appearance>,

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
			appearances: HashSet::new(),
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
	pub fn solo_file(mut self, name: String) -> Self {
		self.display_name = name;
		self.appearances.insert(Appearance::SoloFile);
		self
	}

	/// Get the `Node` instance with the given name hardcoded.
	///
	/// This function consumes the given `Node` and returns a new instance with
	/// the name hardcoded. It should be used to change the name to something
	/// different from the name derived from the path.
	pub fn symlink(mut self, name: String) -> Self {
		self.display_name = name;
		self.appearances.insert(Appearance::Symlink);
		self
	}

	/// Get the `Node` instance with some tree-drawing characters.
	///
	/// This function consumes the given `Node` and returns a new instance with
	/// the appearance configured with the tree shapes. It is used to make the
	/// node the child of another node.
	pub fn tree_child(mut self) -> Self {
		self.appearances.insert(Appearance::TreeChild);
		self
	}

	/// Get the `Node` instance with children populated.
	pub fn tree_parent(mut self, children: Vec<Node<'pls>>) -> Self {
		self.children = children;
		self.appearances.insert(Appearance::TreeParent);
		self
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
		self.collapse_name = self
			.specs
			.iter()
			.rev()
			.filter_map(|spec| spec.collapse.as_ref())
			.next()
			.map(|collapse| match collapse {
				Collapse::Name(name) => name.clone(),
				Collapse::Ext(ext) if ext.is_empty() => self.stem(),
				Collapse::Ext(ext) => format!("{}.{}", self.stem(), ext),
			});
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
	fn directives(&self, app_const: &AppConst, entry_const: &EntryConst) -> String {
		let mut directives = String::from(self.typ.directives(entry_const));

		if !self.appearances.contains(&Appearance::Symlink) {
			let imp_dir = Imp::directives(self, app_const);
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

	/// Get the icons associated with the node, filtered by the
	/// capabilities of the current terminal.
	///
	/// A node can get its icon from two sources:
	///
	/// * specs associated with the node
	/// * the node's type
	fn icon(&self, conf: &Conf, entry_const: &EntryConst) -> Icon {
		let icon = self
			.specs
			.iter()
			.rev()
			.filter_map(|spec| spec.icons.as_ref())
			.chain(self.typ.icons(entry_const))
			.flatten()
			.find_map(|icon_name| {
				conf.icons
					.get(icon_name.as_str())
					.filter(|icon| !icon.ends_with(".svg") || PLS.supports_gfx)
			});

		match icon {
			Some(icon) => {
				let icon = String::from(icon);
				if icon.ends_with(".svg") {
					Icon::Image(icon)
				} else {
					Icon::Text(icon)
				}
			}
			None => Icon::Text(String::default()),
		}
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
		tree_shapes: &[&str],
	) -> String {
		let text_directives = self.directives(app_const, entry_const);

		let mut parts = String::default();

		// Tree shape
		if self.appearances.contains(&Appearance::TreeChild) {
			let offset = " ".repeat(if PLS.args.align { 3 } else { 2 });
			parts.push_str(&tree_shapes.iter().fold(String::new(), |mut acc, shape| {
				let _ = write!(acc, "{offset}{shape}"); // `write!`-ing into a `String` can never fail.
				acc
			}));
		}

		// Icon
		if PLS.args.icon && !self.appearances.contains(&Appearance::Symlink) {
			let icon = self.icon(conf, entry_const);
			parts.push_str(&icon.render(&text_directives));
		}

		// Name and suffix
		parts.push_str(&format!("<{text_directives}>"));
		if !PLS.args.align
			|| self.appearances.contains(&Appearance::Symlink)
			|| self.appearances.contains(&Appearance::SoloFile)
		{
			parts.push_str(&self.display_name)
		} else {
			parts.push_str(&self.aligned_name())
		}
		if PLS.args.suffix && !self.appearances.contains(&Appearance::Symlink) {
			// Symlink should not have suffix because it should show the path reference without modifications
			parts.push_str(self.typ.suffix(entry_const))
		};
		parts.push_str("</>");

		if PLS.args.sym {
			if let Some(target) = self.target() {
				parts.push_str(&target.print(conf));
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
			DetailField::Size => self.size(entry_const),
			DetailField::Blocks => self.blocks(entry_const),
			DetailField::Git => self.git(entry_const),
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
		tree_shape: &[&str],
	) -> HashMap<DetailField, String> {
		PLS.args
			.details
			.iter()
			.map(|&detail| {
				if detail == DetailField::Name {
					(
						detail,
						self.display_name(conf, app_const, entry_const, tree_shape),
					)
				} else {
					(detail, self.get_value(detail, owner_man, entry_const))
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

		once(self.row(owner_man, conf, app_const, entry_const, &all_shapes))
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
