use crate::config::AppConst;
use crate::enums::DetailField;
use crate::output::{Cell, Rendered};
use crate::PLS;
use rayon::prelude::*;
use std::fmt::Alignment;
use std::io::{self, BufWriter, Write};

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
	pub entries: Vec<Rendered>,
}

impl Grid {
	/// Create a new instance of `Grid`, rendering and measuring the name column
	/// of the given entries up front.
	pub fn new(entries: Vec<Vec<String>>) -> Self {
		// The grid renders only the name column; find its position in the
		// configured detail fields and pull that value out of each row.
		let name_idx = PLS
			.args
			.details
			.iter()
			.position(|det| *det == DetailField::Name);
		Self {
			entries: entries
				.into_par_iter()
				.map(|mut entry| match name_idx {
					Some(idx) if idx < entry.len() => entry.swap_remove(idx),
					_ => String::default(),
				})
				.map(|markup| Rendered::new(&markup))
				.collect(),
		}
	}

	/// Render the grid to STDOUT.
	pub fn render(&self, _app_const: &AppConst) {
		let mut max_width = self.entries.iter().map(|e| e.width).max();
		let max_cols = self.columns(max_width);

		let entry_len = self.entries.len();
		if entry_len == 0 {
			// Nothing to render, so we exit.
			return;
		}

		let rows = (entry_len as f64 / max_cols as f64).ceil() as usize;
		let cols = (entry_len as f64 / rows as f64).ceil() as usize;

		if cols == 1 {
			// If there is only one column, we don't need to equalise width.
			max_width = None;
		}

		if cols > 1 && PLS.args.down {
			self.print(&self.down(rows), cols, max_width);
		} else {
			let entries: Vec<&Rendered> = self.entries.iter().collect();
			self.print(&entries, cols, max_width);
		};
	}

	/// Print the entries to the screen.
	///
	/// This prints the entries in the specified number of columns, each cell
	/// padded to span the given max-width.
	fn print(&self, entries: &[&Rendered], cols: usize, max_width: Option<usize>) {
		let entry_len = self.entries.len();

		let cell = Cell::new(Alignment::Left, (0, 2));
		let end_cell = Cell::new(Alignment::Left, (0, 0));

		// Buffer the whole grid behind a single stdout lock so that each cell
		// does not incur its own lock acquisition and write syscall. Writing
		// stops at the first error (e.g. a closed pipe) instead of formatting
		// the remaining cells for output that nobody will read.
		let mut out = BufWriter::new(std::io::stdout().lock());
		let _: io::Result<()> = (|| {
			for (idx, cell_content) in entries.iter().enumerate() {
				if idx % cols == cols - 1 || idx == entry_len - 1 {
					writeln!(out, "{}", end_cell.print(cell_content, &max_width))?;
				} else {
					write!(out, "{}", cell.print(cell_content, &max_width))?;
				}
			}
			out.flush()
		})();
	}

	/// Shuffle the entries to enable printing down instead of across.
	///
	/// Since terminals can only print row-by-row, we split the entries into
	/// columns and then pick one cell per column, going in cycles till all
	/// cells are exhausted.
	fn down(&self, rows: usize) -> Vec<&Rendered> {
		let chunks: Vec<_> = self.entries.chunks(rows).collect();
		(0..rows)
			.flat_map(|row_idx| chunks.iter().filter_map(move |chunk| chunk.get(row_idx)))
			.collect()
	}

	/// Get the number of columns that can be accommodated on the screen.
	///
	/// If the terminal width cannot be determined, such as when piping to a
	/// file, the output will be laid out in a single column.
	fn columns(&self, max_width: Option<usize>) -> u16 {
		match (Self::term_width(), max_width) {
			(Some(term_width), Some(item_width)) => {
				let cols = (term_width + 2) / (item_width as u16 + 2);
				cols.max(1)
			}
			_ => 1,
		}
	}

	/// Get the terminal width.
	///
	/// The terminal width is determined from two sources:
	///
	/// * the `PLS_COLUMNS` environment variable, if it is set
	/// * the result of an ioctl call, if it succeeds
	fn term_width() -> Option<u16> {
		std::env::var("PLS_COLUMNS") // development hack
			.ok()
			.and_then(|width_str| width_str.parse::<u16>().ok())
			.or_else(|| PLS.window.as_ref().map(|win| win.ws_col))
	}
}
