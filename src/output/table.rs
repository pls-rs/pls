use crate::config::AppConst;
use crate::fmt::len;
use crate::output::{render_rows, Rendered};
use crate::PLS;
use std::io::{BufWriter, Write};
use std::iter::once;

/// The detailed renders node names, and optionally, chosen node metadata in
/// a tabular layout with one row per node.
///
/// The detailed view is one of two views supported by `pls`, the other being
/// the [grid view](crate::output::Grid).
#[derive(Default)]
pub struct Table {
	pub entries: Vec<Vec<Rendered>>,
	pub is_solo: bool,
}

impl Table {
	/// Create a new instance of `Table`, rendering and measuring the given
	/// markup entries up front so the layout never re-parses markup.
	pub fn new(entries: Vec<Vec<String>>, is_solo: bool) -> Self {
		Self {
			entries: render_rows(entries),
			is_solo,
		}
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

		// Buffer the whole table behind a single stdout lock so that each cell
		// does not incur its own lock acquisition and write syscall.
		let mut out = BufWriter::new(std::io::stdout().lock());

		if PLS.args.header {
			let header_style = app_const.table.header_style.as_str();
			for (width, det, cell) in &iter_basis {
				let name = det.name(app_const);
				let _ = write!(
					out,
					"{}",
					cell.print_markup(name, width, Some(header_style))
				);
			}
			let _ = writeln!(out);
		}

		for entry in &self.entries {
			for ((width, _det, cell), value) in iter_basis.iter().zip(entry) {
				let _ = write!(out, "{}", cell.print(value, width));
			}
			let _ = writeln!(out);
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
					.filter_map(|entry| entry.get(det_idx).map(|cell| cell.width))
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
