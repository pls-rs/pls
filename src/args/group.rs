use crate::args::dir_group::DirGroup;
use crate::args::files_group::FilesGroup;
use crate::args::input::Input;
use crate::config::{Args, Conf, ConfMan};
use crate::enums::{DetailField, Typ};
use crate::exc::Exc;
use crate::fmt::render;
use crate::models::OwnerMan;
use crate::output::{Grid, Table};
use std::collections::HashMap;

// ======
// Models
// ======

/// Represents a set, possibly singleton, of paths entered in the CLI.
///
/// Each group generates one UI block, table or grid, in the final output. This
/// is done so that individual files provided as arguments can be displayed
/// more compactly as a collection.
#[derive(Debug)]
pub enum Group {
	/// represents one directory path entered on the CLI
	Dir(DirGroup),
	/// represents all individual file paths entered on the CLI
	Files(FilesGroup),
}

// ===============
// Implementations
// ===============

impl Group {
	/// Partition the given inputs into groups.
	///
	/// Each directory becomes its own group, denoted by [`DirGroup`], while
	/// all files are collected into a single group denoted by [`FilesGroup`].
	/// This separation is an implementation detail.
	pub fn partition(inputs: Vec<Input>, conf_man: &ConfMan) -> Vec<Self> {
		let mut groups = vec![];
		let mut files = vec![];
		for input in inputs {
			if input.typ == Typ::Dir {
				groups.push(Self::Dir(DirGroup::new(input)));
			} else {
				files.push(input);
			}
		}
		if !files.is_empty() {
			groups.insert(0, Self::Files(FilesGroup::new(files, conf_man)));
		}
		groups
	}

	pub fn render(
		&self,
		show_title: bool,
		owner_man: &mut OwnerMan,
		args: &Args,
	) -> Result<(), Exc> {
		if show_title {
			if let Self::Dir(group) = self {
				println!(
					"\n{}",
					render(format!("<bold>{}:</bold>", group.input.path.display()))
				);
			}
		}

		let entries = self.entries(owner_man, args)?;

		if args.grid {
			let grid = Grid::new(entries);
			grid.render(&self.conf().app_const, args);
		} else {
			let table = Table::new(entries, matches!(self, Self::Files(_)));
			table.render(&self.conf().app_const, args);
		}

		Ok(())
	}

	/// Get the config for this group.
	///
	/// For a directory, the config file inside the directory is used. For a
	/// group of files, the config file in the common ancestor directory is
	/// used.
	fn conf(&self) -> &Conf {
		match self {
			Self::Dir(group) => &group.input.conf,
			Self::Files(group) => &group.parent_conf,
		}
	}

	/// Convert this group into a vector of entries that can be passed into the
	/// layout to be rendered.
	pub fn entries(
		&self,
		owner_man: &mut OwnerMan,
		args: &Args,
	) -> Result<Vec<HashMap<DetailField, String>>, Exc> {
		match self {
			Self::Dir(group) => group.entries(owner_man, args),
			Self::Files(group) => Ok(group.entries(owner_man, args)),
		}
	}
}
