use crate::config::{Args, Conf};
use crate::fmt::render;
use crate::models::{Node, OwnerMan};
use crate::output::{Grid, Table};
use std::path::Path;

/// Represents the entire application state.
#[derive(Default)]
pub struct Pls {
	/// configuration from `.pls.yml` files
	conf: Conf,
	/// command-line arguments
	args: Args,
}

impl Pls {
	/// Get the list of nodes for a given path.
	///
	/// If the path is a directory, the list will consist of the immediate
	/// contents of that directory. If the path is a file, the list will consist
	/// of just that file.
	fn get_contents(&self, path: &Path) -> Result<Vec<Node>, ()> {
		if path.is_dir() {
			let entries = match path.read_dir() {
				Ok(entries) => entries,
				Err(_) => return Err(()),
			};
			let nodes = entries
				.into_iter()
				.filter_map(|entry| entry.ok().map(|entry| Node::new(&entry.path())))
				.collect();
			Ok(nodes)
		} else {
			Ok(vec![Node::new(path)])
		}
	}

	/// List the given path.
	///
	/// This function contains the core logic of the application, while `run`,
	/// which calls this function, contains the logic for iterating over the
	/// paths to be listed.
	fn list(&self, path: &Path) -> Result<(), ()> {
		let path_buf = match path.canonicalize() {
			Ok(path_buf) => path_buf,
			Err(_) => return Err(()),
		};

		let mut nodes = match self.get_contents(&path_buf) {
			Ok(nodes) => nodes,
			Err(_) => return Err(()),
		};

		let mut owner_man = OwnerMan::default();

		self.args.sort_bases.iter().rev().for_each(|field| {
			nodes.sort_by(|a, b| field.compare(a, b, &mut owner_man));
		});

		let entries: Vec<_> = nodes
			.iter()
			.map(|node| node.row(&mut owner_man, &self.conf, &self.args))
			.collect();

		if self.args.grid {
			let grid = Grid::new(entries);
			grid.render(&self.conf, &self.args);
		} else {
			let table = Table::new(entries);
			table.render(&self.conf, &self.args);
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

			if self.list(path).is_err() {
				println!("Error occurred");
			}
		}
	}
}
