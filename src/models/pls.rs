use crate::config::{Args, Conf, ConfMan};
use crate::exc::Exc;
use crate::fmt::render;
use crate::models::{Node, OwnerMan};
use crate::output::{Grid, Table};
use crate::traits::Imp;
use log::{debug, info};
use std::fs::DirEntry;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// Represents the entire application state.
#[derive(Default)]
pub struct Pls {
	/// configuration manager for `.pls.yml` files
	conf_man: ConfMan,
	/// command-line arguments
	args: Args,
}

impl Pls {
	/// Create a node from the given entry of a directory.
	///
	/// This function performs filtering of nodes based on the following
	/// criteria, returning `None` if the node is to be filtered out.
	///
	/// * name (using the args `--only` and `--exclude`)
	/// * type (using the arg `--types`)
	/// * importance (using the arg `--imp`)
	fn get_node<'pls>(&'pls self, entry: DirEntry, conf: &'pls Conf) -> Option<Node> {
		let name = entry.file_name();
		debug!("Checking visibility of name {name:?}.");
		let haystack = name.as_bytes();
		let is_visible = self
			.args
			.only
			.as_ref()
			.map_or(true, |pat| pat.is_match(haystack))
			&& self
				.args
				.exclude
				.as_ref()
				.map_or(true, |pat| !pat.is_match(haystack));
		if !is_visible {
			return None;
		}

		let mut node = Node::new(&entry.path());
		debug!("Checking visibility of typ {:?}.", node.typ);
		if !self.args.typs.contains(&node.typ) {
			return None;
		}

		node.match_specs(&conf.specs);
		if !node.is_visible(conf, &self.args) {
			return None;
		}

		Some(node)
	}

	/// Get the list of nodes for a given path.
	///
	/// If the path is a directory, the list will consist of the immediate
	/// contents of that directory. If the path is a file, the list will consist
	/// of just that file.
	///
	/// We do not perform visibility checks when a single file is to be listed
	/// because it goes against the users expectations to see a blank output
	/// when wanting to see information about a specific file.
	fn get_contents<'pls>(&'pls self, path: &Path, conf: &'pls Conf) -> Result<Vec<Node>, Exc> {
		if path.is_dir() {
			let entries = path.read_dir().map_err(Exc::IoError)?;
			let nodes = entries
				.into_iter()
				.filter_map(|entry| entry.ok().and_then(|entry| self.get_node(entry, conf)))
				.collect();
			Ok(nodes)
		} else {
			let mut node = Node::new(path);
			node.match_specs(&conf.specs);
			Ok(vec![node])
		}
	}

	/// List the given path.
	///
	/// This function contains the core logic of the application, while `run`,
	/// which calls this function, contains the logic for iterating over the
	/// paths to be listed.
	///
	/// Note that a lot of operations in this function can be done in parallel.
	/// There is scope for considerable performance improvements here.
	fn list(&self, path: &Path) -> Result<(), Exc> {
		let path_buf = path.canonicalize().map_err(Exc::IoError)?;

		// Create the configuration specific to this path.
		let mut conf = self.conf_man.get(Some(path))?;
		conf.constants.massage_imps();

		// Get all nodes corresponding to this path. This list is already
		// filtered by all filtering criteria.
		let mut nodes = self.get_contents(&path_buf, &conf)?;

		// Create the ownership manager. This instance caches user and
		// membership information, so it should be reused for both sorting and
		// detail fields.
		let mut owner_man = OwnerMan::default();

		// Sort the nodes using the sort bases. This is in reverse order because
		// the first listed base should be the main sorting factor.
		if nodes.len() > 1 {
			self.args.sort_bases.iter().rev().for_each(|field| {
				nodes.sort_by(|a, b| field.compare(a, b, &mut owner_man));
			});
		}

		// Convert each node into a row that becomes an entry for a printer.
		let entries: Vec<_> = nodes
			.iter()
			.map(|node| node.row(&mut owner_man, &conf, &self.args))
			.collect();

		// Create the printer and render the entries to STDOUT.
		if self.args.grid {
			let grid = Grid::new(entries);
			grid.render(&conf, &self.args);
		} else {
			let table = Table::new(entries);
			table.render(&conf, &self.args);
		}

		Ok(())
	}

	/// Run `pls`.
	///
	/// This is the entrypoint of the `Pls` class, and once control is passed
	/// to it from `main`, it handles everything.
	pub fn run(&self) {
		for (idx, path) in self.args.paths.iter().enumerate() {
			if idx >= 1 {
				println!(); // Leave a line between each listed path.
			}
			if self.args.paths.len() > 1 {
				println!("{}", render(format!("<bold>{}:</>", path.display())));
			}

			match self.list(path) {
				Ok(()) => info!("All OK!"),
				Err(exc) => println!("{exc}"),
			}
		}
	}
}
