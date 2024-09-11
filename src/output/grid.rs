use crate::config::AppConst;
use crate::enums::DetailField;
use crate::fmt::len;
use crate::output::Cell;
use crate::PLS;
use std::collections::HashMap;
use std::fmt::Alignment;
use terminal_size::{terminal_size, Width};

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
	pub fn render(&self, _app_const: &AppConst) {
		let max_width = self.entries.iter().map(len).max();
		let max_cols = self.columns(max_width);

		let entry_len = self.entries.len();
		let rows = (entry_len as f64 / max_cols as f64).ceil() as usize;
		let cols = (entry_len as f64 / rows as f64).ceil() as usize;

		if PLS.args.down {
			self.print(&self.down(rows), cols, max_width);
		} else {
			self.print(&self.entries, cols, max_width);
		};
	}

	/// Print the entries to the screen.
	///
	/// This prints the entries in the specified number of columns, each cell
	/// padded to span the given max-width.
	fn print<S>(&self, entries: &[S], cols: usize, max_width: Option<usize>)
	where
		S: AsRef<str>,
	{
		let entry_len = self.entries.len();

		let cell = Cell::new(Alignment::Left, (0, 2));
		let end_cell = Cell::new(Alignment::Left, (0, 0));
		for (idx, text) in entries.iter().enumerate() {
			if idx % cols == cols - 1 || idx == entry_len - 1 {
				println!("{}", &end_cell.print(text, &max_width, None));
			} else {
				print!("{}", &cell.print(text, &max_width, None));
			}
		}
	}

	/// Shuffle the entries to enable printing down instead of across.
	///
	/// Since terminals can only print row-by-row, we split the entries into
	/// columns and then pick one cell per column, going in cycles till all
	/// cells are exhausted.
	fn down(&self, rows: usize) -> Vec<&String> {
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
	/// If the `PLS_COLUMNS` environment variable is set, the value of that
	/// variable is used as the terminal width. Otherwise, the terminal width is
	/// determined using the `terminal_size` crate.
	fn term_width() -> Option<u16> {
		std::env::var("PLS_COLUMNS") // development hack
			.ok()
			.and_then(|width_str| width_str.parse::<u16>().ok())
			.or_else(|| terminal_size().map(|(Width(term_width), _)| term_width))
	}
}
