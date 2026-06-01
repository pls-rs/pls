use crate::args::input::Input;
use crate::enums::{DetailField, SortField};
use crate::exc::Exc;
use crate::models::{Node, OwnerMan, Owners};
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
	pub fn entries(&self, owner_man: &mut OwnerMan) -> Result<Vec<Vec<String>>, Exc> {
		let mut nodes = self.nodes()?;
		if PLS.args.collapse {
			nodes = Self::make_tree(nodes);
		}

		// Owners are looked up through an immutable, shareable view rather than
		// the mutable manager. When any owner column or owner-based sort is in
		// play, resolve every owner up front and then hand out that view.
		if owners_needed() {
			Self::resolve_owners(&nodes, owner_man);
		}
		let owners = owner_man.owners();

		Self::re_sort(&mut nodes, owners);

		let entries = nodes
			.iter()
			.flat_map(|node| {
				node.entries(
					owners,
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
	fn node(&self, entry: DirEntry) -> Option<Node<'_>> {
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
			.is_some_and(|pat| pat.is_match(haystack));
		if exclude {
			debug!("Name {name:?} matched `--exclude`.");
			return None;
		}

		let mut node = Node::from_entry(&entry);

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
	fn nodes(&self) -> Result<Vec<Node<'_>>, Exc> {
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
	fn re_sort(nodes: &mut [Node], owners: Owners) {
		if nodes.len() <= 1 {
			return;
		}
		PLS.args.sort_bases.iter().rev().for_each(|field| {
			nodes.sort_by(|a, b| field.compare(a, b, owners));
		});
		for node in nodes {
			Self::re_sort(&mut node.children, owners);
		}
	}

	/// Resolve the owning user and group of every node in the given forest.
	///
	/// This populates the [`OwnerMan`] cache up front so that rendering can look
	/// owners up through an immutable [`Owners`] view without touching the cache.
	fn resolve_owners(nodes: &[Node], owner_man: &mut OwnerMan) {
		for node in nodes {
			if let Some(meta) = node.meta_ok() {
				use std::os::unix::fs::MetadataExt;
				owner_man.user(meta.uid());
				owner_man.group(meta.gid());
			}
			Self::resolve_owners(&node.children, owner_man);
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
		nodes.into_iter().for_each(|mut node| {
			if let Some(collapse) = node.collapse_name.take() {
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

/// Whether the current invocation needs owner information resolved.
///
/// Owner resolution is only worth its (serial) cost when an owner column is
/// displayed or the nodes are sorted by owner; other views never consult the
/// owner cache.
pub(crate) fn owners_needed() -> bool {
	use DetailField::{Gid, Group, Uid, User};
	use SortField::{
		Group as GroupSort, Group_ as GroupSortRev, User as UserSort, User_ as UserSortRev,
	};
	PLS.args
		.details
		.iter()
		.any(|field| matches!(field, User | Uid | Group | Gid))
		|| PLS
			.args
			.sort_bases
			.iter()
			.any(|field| matches!(field, UserSort | UserSortRev | GroupSort | GroupSortRev))
}
