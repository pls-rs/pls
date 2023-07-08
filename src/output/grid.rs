use crate::config::{Args, Conf};
use crate::enums::DetailField;
use crate::fmt::render;
use std::collections::HashMap;

/// The grid view renders the node names in a two dimensional layout to minimise
/// scrolling. It does not support rendering of node metadata.
///
/// The grid view is one of two views supported by `pls`, the other being the
/// [detailed view](crate::output::Table).
///
/// The grid view tries to render all elements in as few lines as possible. Once
/// the number of lines has been minimised, it minimises the column count by
/// making each column take the maximum number of rows.
pub struct Grid {
	pub entries: Vec<String>,
}

impl Grid {
	/// Create a new instance of `Grid`, taking ownership of the given entries.
	pub fn new(entries: Vec<HashMap<DetailField, String>>) -> Self {
		Self {
			entries: entries
				.into_iter()
				.map(|mut entry| entry.remove(&DetailField::Name).unwrap_or_default())
				.collect(),
		}
	}

	/// Render the grid to STDOUT.
	pub fn render(&self, _conf: &Conf, _args: &Args) {
		for entry in &self.entries {
			print!(
				"{} ",
				render(entry.get(&DetailField::Name).unwrap_or(&String::default()))
			);
		}
	}
}
