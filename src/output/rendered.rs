use crate::fmt::render_and_measure;
use rayon::prelude::*;

/// A cell whose markup has already been converted to ANSI escape codes, paired
/// with its rendered display width.
///
/// Rendering markup (parsing tags, applying styles) and measuring its width
/// both require a pass over the string. Doing that work once up front — instead
/// of repeatedly while computing column widths and again while printing — keeps
/// the hot output path free of markup parsing.
pub struct Rendered {
	/// the cell content, already rendered from markup into ANSI escape codes
	pub text: String,
	/// the display width of [`text`](Self::text) in terminal columns, excluding
	/// ANSI escape codes and terminal-graphics sequences
	pub width: usize,
}

impl Rendered {
	/// Render and measure the given markup string into a `Rendered` cell.
	pub fn new(markup: &str) -> Self {
		let (text, width) = render_and_measure(markup);
		Self { text, width }
	}
}

/// Render and measure every cell of every row.
///
/// This is the single point at which a group's markup rows are converted into
/// printable, pre-measured cells. Rendering a cell is pure, independent string
/// work, so the rows are rendered in parallel across the global thread pool;
/// `collect` preserves the original row order.
pub fn render_rows(rows: Vec<Vec<String>>) -> Vec<Vec<Rendered>> {
	rows.into_par_iter()
		.map(|row| row.into_iter().map(|cell| Rendered::new(&cell)).collect())
		.collect()
}
