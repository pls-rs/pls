use crate::fmt::{len, render};
use crate::gfx::strip_image;
use std::fmt::Alignment;

/// Represents one cell in the rendered output.
///
/// It provides a convenient way to render text with alignment and padding.
///
/// `Cell` instances are reusable, meaning that once you have a cell for a
/// particular column, you can use it to render any number all rows for that
/// column. This is facilitated by all variable fields being passed as arguments
/// to the [`print`](Cell::print) method and not saved in the struct itself.
pub struct Cell {
	pub alignment: Alignment,
	pub padding: (usize, usize),
}

impl Default for Cell {
	fn default() -> Self {
		Self {
			alignment: Alignment::Left,
			padding: (0, 1),
		}
	}
}

impl Cell {
	/// Create a `Cell` instance with the given alignment and padding.
	pub fn new(alignment: Alignment, padding: (usize, usize)) -> Self {
		Self { alignment, padding }
	}

	/// Return the content of the cell, padded to the given width and aligned
	/// as per the cell's alignment directive.
	///
	/// This function calls render to ensure that markup in the cell contents
	/// is rendered into ANSI escape sequences.
	///
	/// # Arguments
	///
	/// * `text` - the text to print in the cell
	/// * `width` - the width that the cell should span
	/// * `directives` - styles to apply to the entire cell, including padding
	pub fn print<S>(&self, text: S, width: &Option<usize>, directives: Option<String>) -> String
	where
		S: AsRef<str>,
	{
		let text = text.as_ref();
		let text_len = len(strip_image(text)); // This `len` can understand markup.

		let (left, right): (usize, usize) = match width {
			Some(width) if *width > text_len => {
				let pad = width - text_len;
				match self.alignment {
					Alignment::Left => (0, pad),
					Alignment::Center => (pad / 2, pad - (pad / 2)),
					Alignment::Right => (pad, 0),
				}
			}
			_ => (0, 0),
		};
		let (left, right) = (
			" ".repeat(left + self.padding.0),
			" ".repeat(right + self.padding.1),
		);

		let mut content = format!("{left}{text}{right}");

		if let Some(directives) = directives {
			content.insert_str(0, "<>");
			content.insert_str(1, &directives);
			content.push_str("</>");
		}

		render(content)
	}
}

#[cfg(test)]
mod tests {
	use super::Cell;
	use crate::fmt::render;
	use std::fmt::Alignment;

	macro_rules! make_padding_test {
		( $($name:ident: $text:expr, $left:expr, $right:expr => $expected:expr,)* ) => {
			$(
				#[test]
				fn $name() {
					let cell = Cell { padding: ($left, $right), ..Cell::default() };
					assert_eq!(cell.print($text, &None, None), $expected);
				}
			)*
		};
	}

	make_padding_test!(
		test_left_only_padding: "A", 1, 0 => " A",
		test_right_only_padding: "A", 0, 1 => "A ",
		test_left_and_right_padding: "A", 1, 1 => " A ",
	);

	macro_rules! make_print_test {
		( $($name:ident: $text:expr, $alignment:expr, $width:expr => $expected:expr,)* ) => {
			$(
				#[test]
				fn $name() {
					colored::control::set_override(true); // needed when running tests in CLion
					let cell = Cell{ alignment: $alignment, ..Cell::default() };
					assert_eq!(cell.print($text, &$width, None), $expected);
				}
			)*
		};
	}

	make_print_test!(
		test_simple_left: "A", Alignment::Left, Some(5) => "A     ",
		test_simple_right: "A", Alignment::Right, Some(5) => "    A ",
		test_simple_center: "A", Alignment::Center, Some(5) => "  A   ",
		test_unbalanced_center: "A", Alignment::Center, Some(6) => "  A    ",

		test_renders_markup: "<bold>A</>", Alignment::Center, None => render("<bold>A</> "),

		test_excludes_markup_from_len: "<bold>A</>", Alignment::Center, Some(5) => render("  <bold>A</>   "),

		test_handles_missing_width: "A", Alignment::Center, None => "A ",
	);
}
