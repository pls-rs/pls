use crate::config::{Args, Conf};
use crate::enums::DetailField;
use crate::fmt::render;
use std::collections::HashMap;

/// The detailed renders node names, and optionally, chosen node metadata in
/// a tabular layout with one row per node.
///
/// The detailed view is one of two views supported by `pls`, the other being
/// the [grid view](crate::output::Grid).
#[derive(Default)]
pub struct Table {
	pub entries: Vec<HashMap<DetailField, String>>,
}

impl Table {
	/// Create a new instance of `Table`, taking ownership of the given entries.
	pub fn new(entries: Vec<HashMap<DetailField, String>>) -> Self {
		Self { entries }
	}

	/// Render the table to STDOUT.
	pub fn render(&self, _conf: &Conf, args: &Args) {
		for entry in &self.entries {
			for det in &args.details {
				print!("{} ", render(entry.get(det).unwrap_or(&String::default())));
			}
			println!();
		}
	}
}
