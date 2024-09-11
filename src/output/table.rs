use crate::config::AppConst;
use crate::enums::DetailField;
use crate::fmt::len;
use crate::PLS;
use std::collections::HashMap;
use std::iter::once;

/// The detailed renders node names, and optionally, chosen node metadata in
/// a tabular layout with one row per node.
///
/// The detailed view is one of two views supported by `pls`, the other being
/// the [grid view](crate::output::Grid).
#[derive(Default)]
pub struct Table {
	pub entries: Vec<HashMap<DetailField, String>>,
	pub is_solo: bool,
}

impl Table {
	/// Create a new instance of `Table`, taking ownership of the given entries.
	pub fn new(entries: Vec<HashMap<DetailField, String>>, is_solo: bool) -> Self {
		Self { entries, is_solo }
	}

	/// Render the table to STDOUT.
	pub fn render(&self, app_const: &AppConst) {
		let max_widths = self.max_widths(app_const);

		let iter_basis: Vec<_> = PLS
			.args
			.details
			.iter()
			.enumerate()
			.map(|(idx, det)| {
				let mut cell = det.cell();
				if idx == PLS.args.details.len() - 1 {
					cell.padding = (0, 0); // Remove right padding from the last column.
				}
				(max_widths[idx], det, cell)
			})
			.collect();

		if PLS.args.header {
			for (width, det, cell) in &iter_basis {
				let name = det.name(app_const);
				let directives = app_const.table.header_style.clone();
				print!("{}", &cell.print(name, width, Some(directives)));
			}
			println!();
		}

		for entry in &self.entries {
			for (width, det, cell) in &iter_basis {
				print!("{}", &cell.print(entry.get(det).unwrap(), width, None));
			}
			println!();
		}
	}

	/// Get mapping of detail field to the maximum width of the cells in that
	/// column.
	fn max_widths(&self, app_const: &AppConst) -> Vec<Option<usize>> {
		PLS.args
			.details
			.iter()
			.enumerate()
			.map(|(det_idx, det)| {
				if det_idx == PLS.args.details.len() - 1 {
					return None;
				}
				let end_lim = if self.entries.is_empty() {
					// If there are no entries, the limit must be zero.
					0
				} else if !self.is_solo && det.uniformly_wide() {
					// For uniform columns, only compare the header and row #1.
					1
				} else {
					// For non-uniform columns, compare the header and every row.
					// This is much slower as makes two passes over every cell.
					self.entries.len()
				};
				self.entries[0..end_lim]
					.iter()
					.filter_map(|entry| entry.get(det).map(len))
					.chain(once(if PLS.args.header {
						len(det.name(app_const))
					} else {
						0
					}))
					.max()
			})
			.collect()
	}
}
