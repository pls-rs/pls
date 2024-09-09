use crate::args::input::Input;
use crate::config::{Conf, ConfMan};
use crate::enums::DetailField;
use crate::models::{Node, OwnerMan, Pls};
use crate::utils::paths::common_ancestor;
use log::debug;
use std::collections::HashMap;
use std::path::PathBuf;

// ======
// Models
// ======

/// Represents a group that renders the given collection of individual files.
///
/// A group of files will use the UI configuration of their common ancestor
/// while still using their individual configurations for their entry in the
/// layout.
pub struct FilesGroup {
	pub inputs: Vec<Input>,

	pub common_ancestor: Option<PathBuf>,
	pub parent_conf: Conf,
}

// ===============
// Implementations
// ===============

impl FilesGroup {
	// ===========
	// Constructor
	// ===========

	pub fn new(inputs: Vec<Input>, conf_man: &ConfMan) -> Self {
		let abs: Vec<_> = inputs.iter().map(|input| input.abs.as_path()).collect();
		let common_ancestor = common_ancestor(&abs);
		let mut conf = conf_man.get(common_ancestor.as_ref()).unwrap_or_default();
		conf.app_const.massage_imps();

		Self {
			inputs,
			common_ancestor,
			parent_conf: conf,
		}
	}

	// ======
	// Public
	// ======

	/// Convert this list of files into entries for the output layout.
	///
	/// Since individual nodes are not nested, the function uses each node's
	/// [`Node::row`] instead of the flattened output of each node's
	/// [`Node::entries`].
	pub fn entries(
		&self,
		owner_man: &mut OwnerMan,
		pls: &Pls,
	) -> Vec<HashMap<DetailField, String>> {
		self.nodes()
			.iter()
			.map(|(node, conf)| {
				node.row(
					owner_man,
					conf,
					&self.parent_conf.app_const,
					&conf.entry_const,
					pls,
					&[],
				)
			})
			.collect()
	}

	// =======
	// Private
	// =======

	/// Get a list of nodes from the individual files in this group.
	///
	/// Unlike [`DirGroup`](crate::args::dir_group::DirGroup), this function
	/// does not filter out nodes based on their visibility. This is because the
	/// files in this group have been explicitly provided by the user and should
	/// be rendered regardless of their visibility.
	fn nodes(&self) -> Vec<(Node, &Conf)> {
		self.inputs
			.iter()
			.map(|input| {
				let display_name = input.path.to_string_lossy().to_string();
				let mut node = Node::new(&input.path).solo_file(display_name);
				debug!("Currently {} specs", input.conf.specs.len());
				node.match_specs(&input.conf.specs);
				(node, &input.conf)
			})
			.collect()
	}
}

impl std::fmt::Debug for FilesGroup {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("FilesGroup")
			.field("inputs", &self.inputs)
			.field("common_ancestor", &self.common_ancestor)
			.finish()
	}
}
