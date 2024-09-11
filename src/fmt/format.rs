use colored::{Color, ColoredString, Colorize};
use regex::Regex;
use std::sync::LazyLock;

static TRUE_COLOR: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(
		r"(?x)(?-u)^
        rgb\(
            (?P<red>\d{1,3}),\s?
            (?P<green>\d{1,3}),\s?
            (?P<blue>\d{1,3})
        \)
    $",
	)
	.unwrap()
});

/// Format the given string using the given list of directives.
///
/// Each of these directives can be a style or a color.
///
/// Styles can be one of a predefined list, namely 'blink', 'bold', 'dimmed',
/// 'hidden', 'italic', 'reversed', 'strikethrough', 'underline', or 'clear'.
/// Using the 'clear' style removes all existing styles from the string.
///
/// Colors can be one of the following:
///
/// * 8 ANSI colors
/// * 8 bright versions of the ANSI colors (with the 'bright_' prefix)
/// * true RGB colors (using the 'rgb(<red>,<green>,<blue>)' notation)
///
/// These colors are applied to the foreground text by default, but can be
/// applied to the background instead (using the 'bg:' prefix).
///
/// For more information, refer to the documentation for the
/// [colored](https://docs.rs/colored) crate.
///
/// # Arguments
///
/// * `text` - the string to format according to the style directives
/// * `directives` - the formatting directives to apply to the string
pub fn fmt<S, T>(text: S, directives: &[T]) -> String
where
	S: AsRef<str>,
	T: AsRef<str>,
{
	let mut string = ColoredString::from(text.as_ref());
	for directive in directives {
		string = apply_directive(string, directive.as_ref())
	}
	string.to_string()
}

/// Apply a single directive to a `ColoredString` instance, consuming it and
/// returning a new `ColoredString` instance with that directive applied.
fn apply_directive(string: ColoredString, directive: &str) -> ColoredString {
	// Handle blank directives fast.
	if directive.is_empty() {
		return string;
	};

	let is_bg = directive.starts_with("bg:");
	let directive = directive.replace("bg:", "").replace("bright_", "bright ");

	let string = match directive.as_str() {
		"clear" => return string.clear(), // no style
		"blink" => return string.blink(),
		"bold" => return string.bold(),
		"dimmed" => return string.dimmed(),
		"hidden" => return string.hidden(), // This shouldn't be reachable.
		"italic" => return string.italic(),
		"reversed" => return string.reversed(),
		"strikethrough" => return string.strikethrough(),
		"underline" => return string.underline(),
		_ => string,
	};

	let mut color: Option<Color> = None;
	let caps = TRUE_COLOR.captures(&directive);
	if let Some(caps) = caps {
		// RGB true colors
		let channels: Vec<_> = vec!["red", "green", "blue"]
			.into_iter()
			.filter_map(|x| caps[x].parse::<u8>().ok())
			.collect();
		if channels.len() == 3 {
			color = Some(Color::TrueColor {
				r: channels[0],
				g: channels[1],
				b: channels[2],
			});
		}
	} else {
		// Named ANSI colors
		color = directive.parse().ok()
	}

	match color {
		Some(col) if is_bg => string.on_color(col),
		Some(col) => string.color(col),
		None => string,
	}
}

/// You can see the comprehensive list of escape codes for
/// [ANSI colours on Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors).
#[cfg(test)]
mod tests {
	use super::fmt;

	macro_rules! make_test {
		( $($name:ident: $styles:expr => $prefix:expr, $suffix:expr,)* ) => {
			$(
				#[test]
				fn $name() {
					colored::control::set_override(true); // needed when running tests in CLion
					let text = fmt("Hello, World!", $styles);
					assert_eq!(text, format!("{}{}{}", $prefix, "Hello, World!", $suffix));
				}
			)*
		};
	}

	make_test!(
		test_fmt_applies_ansi_code_for_single_style: &["bold"] => "\x1b[1m", "\x1b[0m",
		test_fmt_applies_ansi_code_for_multiple_styles: &["bold", "italic"] => "\x1b[1;3m", "\x1b[0m",

		test_fmt_handles_reversed_colors: &["reversed"] => "\x1b[7m", "\x1b[0m",

		test_fmt_applies_regular_text_color: &["blue"] => "\x1b[34m", "\x1b[0m",
		test_fmt_applies_bright_text_color: &["bright_blue"] => "\x1b[94m", "\x1b[0m",
		test_fmt_ignores_invalid_text_color: &["invalid"] => "", "",

		test_fmt_applies_rgb_text_color: &["rgb(77,77,77)"] => "\x1b[38;2;77;77;77m", "\x1b[0m",
		test_fmt_ignores_out_of_bounds_rgb_text_color: &["rgb(256,256,256)"] => "", "",

		test_fmt_applies_regular_background_color: &["bg:blue"] => "\x1b[44m", "\x1b[0m",
		test_fmt_applies_bright_background_color: &["bg:bright_blue"] => "\x1b[104m", "\x1b[0m",
		test_fmt_ignores_invalid_background_color: &["bg:invalid"] => "", "",

		test_fmt_applies_rgb_background_color: &["bg:rgb(77,77,77)"] => "\x1b[48;2;77;77;77m", "\x1b[0m",
		test_fmt_ignores_out_of_bounds_rgb_background_color: &["bg:rgb(256,256,256)"] => "", "",

		test_fmt_handles_clear_directive: &["bold", "italic", "clear"] => "", "", // no prefix, no suffix
	);
}
