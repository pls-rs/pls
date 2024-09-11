use crate::args::input::Input;
use crate::enums::DetailField;
use crate::exc::Exc;
use crate::models::{Node, OwnerMan};
use crate::traits::Imp;
use crate::PLS;
use log::debug;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::os::unix::ffi::OsStrExt;

// ======
// Models
// ======

/// Represents a group that renders children of the specified directory.
#[derive(Debug)]
pub struct DirGroup {
	pub input: Input,
}

// ===============
// Implementations
// ===============

impl DirGroup {
	// ===========
	// Constructor
	// ===========

	pub fn new(input: Input) -> Self {
		Self { input }
	}

	// ======
	// Public
	// ======

	/// Convert this directory's children into entries for the output layout.
	///
	/// Since nodes can be nested, the function uses the flattened output of
	/// each node's [`Node::entries`].
	pub fn entries(
		&self,
		owner_man: &mut OwnerMan,
	) -> Result<Vec<HashMap<DetailField, String>>, Exc> {
		let mut nodes = self.nodes()?;
		if PLS.args.collapse {
			nodes = Self::make_tree(nodes);
		}
		Self::re_sort(&mut nodes, owner_man);

		let entries = nodes
			.iter()
			.flat_map(|node| {
				node.entries(
					owner_man,
					&self.input.conf,
					&self.input.conf.app_const,
					&self.input.conf.entry_const,
					&[],
					None,
				)
			})
			.collect();
		Ok(entries)
	}

	// =======
	// Private
	// =======

	/// Convert the directory entry into a [`Node`] instance.
	///
	/// This option converts the directory entry into a `Node` instance,
	/// associates it with the right set of specs and then returns it if the
	/// entry matches the following criteria:
	///
	/// * passes the name-based `--only` and `--exclude` filters
	/// * is of a type accepted by the `--typ` filter
	/// * is above the minimum importance cutoff for visibility
	///
	/// If any criteria is not met, the node is not to be rendered and `None` is
	/// returned.
	fn node(&self, entry: DirEntry) -> Option<Node> {
		let name = entry.file_name();
		debug!("Checking visibility of name {name:?}.");
		let haystack = name.as_bytes();

		let include = PLS
			.args
			.only
			.as_ref()
			.map_or(true, |pat| pat.is_match(haystack));
		if !include {
			debug!("Name {name:?} did not match `--only`.");
			return None;
		}

		let exclude = PLS
			.args
			.exclude
			.as_ref()
			.map_or(false, |pat| pat.is_match(haystack));
		if exclude {
			debug!("Name {name:?} matched `--exclude`.");
			return None;
		}

		let mut node = Node::new(&entry.path());

		debug!("Checking visibility of typ {:?}.", node.typ);
		if !PLS.args.typs.contains(&node.typ) {
			return None;
		}

		node.match_specs(&self.input.conf.specs);

		if !node.is_visible(&self.input.conf) {
			return None;
		}

		Some(node)
	}

	/// Get a list of all nodes that are a children of this directory.
	///
	/// Unlike [`FilesGroup`](crate::args::files_group::FilesGroup), this
	/// function filters out nodes based on visibility.
	fn nodes(&self) -> Result<Vec<Node>, Exc> {
		let entries = self.input.path.read_dir().map_err(Exc::Io)?;

		let entries = entries
			.filter_map(|entry| entry.ok().and_then(|entry| self.node(entry)))
			.collect();
		Ok(entries)
	}

	// ======
	// Static
	// ======

	/// Recursively sort the given list of nodes and their children.
	///
	/// This function iterates over all the sort bases and sorts the given list
	/// of nodes. It is invoked both from the top-level and from each parent
	/// node to sort its children.
	fn re_sort(nodes: &mut [Node], owner_man: &mut OwnerMan) {
		if nodes.len() <= 1 {
			return;
		}
		PLS.args.sort_bases.iter().rev().for_each(|field| {
			nodes.sort_by(|a, b| field.compare(a, b, owner_man));
		});
		for node in nodes {
			Self::re_sort(&mut node.children, owner_man);
		}
	}

	/// Recursively move children nodes into their parent nodes.
	fn re_make_node<'a>(
		node: Node<'a>,
		child_map: &mut HashMap<String, Vec<Node<'a>>>,
	) -> Node<'a> {
		let mut children = vec![];
		if let Some((_id, child_nodes)) = child_map.remove_entry(&node.name) {
			for child_node in child_nodes {
				children.push(Self::re_make_node(child_node.tree_child(), child_map));
			}
		}
		if children.is_empty() {
			node
		} else {
			node.tree_parent(children)
		}
	}

	/// Move children nodes into their parent nodes and return only top-level nodes.
	///
	/// Currently, this is specifically tailored to the collapse feature and not a
	/// generic tree implementation.
	fn make_tree(nodes: Vec<Node>) -> Vec<Node> {
		if nodes.len() <= 1 {
			return nodes;
		}

		let nodes: Vec<_> = nodes
			.into_iter()
			.map(|mut node| {
				node.find_collapse();
				node
			})
			.collect();

		let mut roots = vec![];
		let mut child_map: HashMap<String, Vec<Node>> = HashMap::new();
		nodes.into_iter().for_each(|node| {
			if let Some(collapse) = node.collapse_name.clone() {
				match child_map.entry(collapse) {
					Entry::Occupied(mut entry) => {
						let children = entry.get_mut();
						children.push(node);
					}
					Entry::Vacant(entry) => {
						let children = vec![node];
						entry.insert(children);
					}
				};
			} else {
				roots.push(node);
			}
		});

		if child_map.is_empty() {
			return roots;
		}

		roots = roots
			.into_iter()
			.map(|root| Self::re_make_node(root, &mut child_map))
			.collect();
		roots.extend(child_map.into_values().flatten());
		roots
	}
}
