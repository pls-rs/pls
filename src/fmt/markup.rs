use crate::fmt::format::fmt;
use crate::gfx::strip_image;
use std::cell::RefCell;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

const ESCAPE: u8 = b'\\';
const TAG_OPEN: u8 = b'<';
const TAG_CLOSE: u8 = b'>';

thread_local! {
	/// Per-thread cache of the ANSI prefix/suffix for a set of formatting
	/// directives.
	///
	/// The set of distinct directive combinations is tiny (it is drawn from the
	/// config vocabulary), so memoising the affixes lets the hot path avoid
	/// rebuilding a `ColoredString` for every styled run while still producing
	/// byte-identical output to the underlying `colored` crate.
	static AFFIX_CACHE: RefCell<HashMap<String, (String, String)>> = RefCell::new(HashMap::new());
}

/// A sentinel that cannot appear in user text, used to recover the prefix and
/// suffix that `colored` wraps around a styled run.
const AFFIX_SENTINEL: &str = "\u{1}";

/// Compute the ANSI prefix and suffix that `colored` applies for `directives`.
///
/// `colored` always wraps text as `{prefix}{text}{suffix}` with the affixes
/// independent of the text itself, so rendering a sentinel and splitting on it
/// recovers them exactly.
fn compute_affixes(directives: &[&str]) -> (String, String) {
	let rendered = fmt(AFFIX_SENTINEL, directives);
	match rendered.split_once(AFFIX_SENTINEL) {
		Some((prefix, suffix)) => (prefix.to_string(), suffix.to_string()),
		None => (String::new(), String::new()),
	}
}

/// Append `text` to `out`, wrapped in the ANSI affixes for `directives`.
fn write_run(out: &mut String, directives: &[&str], text: &str) {
	if directives.is_empty() {
		out.push_str(text);
		return;
	}
	AFFIX_CACHE.with(|cache| {
		let mut cache = cache.borrow_mut();
		let (prefix, suffix) = cache
			.entry(directives.join("\u{1f}"))
			.or_insert_with(|| compute_affixes(directives));
		out.push_str(prefix);
		out.push_str(text);
		out.push_str(suffix);
	});
}

/// Count the display width (in graphemes) of a styled run, excluding any
/// embedded terminal-graphics escape sequences.
fn run_width(text: &str) -> usize {
	// Terminal-graphics escapes always begin with the ESC byte, so when none is
	// present the (relatively costly) image strip can be skipped entirely.
	if text.as_bytes().contains(&0x1b) {
		strip_image(text).graphemes(true).count()
	} else {
		text.graphemes(true).count()
	}
}

/// Reduce the textual parts of a markup string.
///
/// This function reduces the text parts of a markup string by passing the
/// following data to a reducer function and updating the accumulator to the
/// returned value.
///
/// * the currently active formatting tags
/// * the last continuous block of text
/// * the current accumulator value
///
/// # Arguments
///
/// * `markup` - the marked-up string to be reduced
/// * `init` - the initial value of the accumulator
/// * `reducer` - the function that works on the aforementioned data
fn reduce_markup<'a, T, F>(markup: &'a str, init: T, reducer: F) -> T
where
	F: Fn(&[Vec<&'a str>], &mut String, T) -> T,
{
	let mut stack: Vec<Vec<&'a str>> = Vec::new(); // the list of currently active tags
	let mut curr: String = String::default(); // the current continuous text block

	let mut acc = init; // initialise the accumulator

	// The delimiters (`<`, `>`, `\`) are all ASCII, so they never appear inside
	// a multi-byte UTF-8 sequence. This lets us scan and slice on byte offsets
	// and borrow tag tokens directly from the input instead of allocating.
	let bytes = markup.as_bytes();
	let mut i = 0;
	while i < bytes.len() {
		match bytes[i] {
			ESCAPE => {
				i += 1; // Consume `BACKSLASH`.
				if bytes.get(i) == Some(&TAG_OPEN) {
					curr.push(TAG_OPEN as char);
					i += 1; // Consume `TAG_OPEN`.
				} else {
					curr.push(ESCAPE as char);
				}
			}
			TAG_OPEN => {
				// Handle the current run of continuous text.
				acc = reducer(&stack, &mut curr, acc);

				i += 1; // Consume `TAG_OPEN`.
				let start = i;
				while i < bytes.len() && bytes[i] != TAG_CLOSE {
					i += 1;
				}
				let tag = &markup[start..i];
				if tag == "/" {
					stack.pop();
				} else {
					stack.push(tag.split(' ').collect());
				}
				if i < bytes.len() {
					i += 1; // Consume `TAG_CLOSE`.
				}
			}
			_ => {
				let start = i;
				while i < bytes.len() && bytes[i] != TAG_OPEN && bytes[i] != ESCAPE {
					i += 1;
				}
				curr.push_str(&markup[start..i]);
			}
		}
	}
	// Call the handler for any uncleared text at the end of the markup.
	reducer(&stack, &mut curr, acc)
}

/// Render the given markup string into ANSI escape codes.
///
/// This function converts the the markup tags into ANSI formatting codes so
/// that if printed to the console, the output matches the tags. This conversion
/// is not reversible as it flattens any nested tags.
///
/// # Arguments
///
/// * `markup` - the marked-up string to be rendered
pub fn render<S>(markup: S) -> String
where
	S: AsRef<str>,
{
	reduce_markup(
		markup.as_ref(),
		String::default(),
		|stack, curr, mut acc| {
			if !curr.is_empty() {
				let directives: Vec<&str> = stack.iter().flatten().copied().collect();
				if !directives.contains(&"hidden") {
					write_run(&mut acc, &directives, curr);
				}
				curr.clear();
			}
			acc
		},
	)
}

/// Render the given markup string into ANSI escape codes and measure its
/// rendered width in a single pass.
///
/// This is equivalent to calling [`render`] and [`len`] separately but parses
/// the markup only once, which matters because every rendered cell needs both
/// its ANSI form (to print) and its display width (to align columns).
///
/// # Arguments
///
/// * `markup` - the marked-up string to be rendered and measured
pub fn render_and_measure<S>(markup: S) -> (String, usize)
where
	S: AsRef<str>,
{
	reduce_markup(
		markup.as_ref(),
		(String::new(), 0_usize),
		|stack, curr, (mut text, mut width)| {
			if !curr.is_empty() {
				let directives: Vec<&str> = stack.iter().flatten().copied().collect();
				if !directives.contains(&"hidden") {
					width += run_width(curr);
					write_run(&mut text, &directives, curr);
				}
				curr.clear();
			}
			(text, width)
		},
	)
}

/// Get the true length of a markup string.
///
/// This counts the number of graphemes (not characters, not bytes) and excludes
/// markup tags from the count. This length can be used to align tables.
///
/// # Arguments
///
/// * `markup` - the marked-up string to be measured
pub fn len<S>(markup: S) -> usize
where
	S: AsRef<str>,
{
	reduce_markup(markup.as_ref(), 0, |stack, curr, acc| {
		let count = if curr.is_empty() || stack.iter().flatten().any(|tag| *tag == "hidden") {
			0
		} else {
			curr.graphemes(true).count()
		};
		curr.clear();
		acc + count
	})
}

#[cfg(test)]
mod tests {
	use super::{len, render, render_and_measure};

	macro_rules! make_render_test {
		( $($name:ident: $markup:expr => $rendered:expr,)* ) => {
			$(
				#[test]
				fn $name() {
					colored::control::set_override(true); // needed when running tests in CLion
					let rendered = render($markup);
					assert_eq!(rendered, $rendered);
				}
			)*
		};
	}

	make_render_test!(
		test_render_formats_single_style: "<bold>bold</>" => "\x1b[1mbold\x1b[0m",
		test_render_formats_multiple_styles: "<bold italic>bold italic</>" => "\x1b[1;3mbold italic\x1b[0m",

		test_render_formats_reversed_colors: "<reversed>reversed</>" => "\x1b[7mreversed\x1b[0m",

		test_render_formats_text_color: "<blue>blue</>" => "\x1b[34mblue\x1b[0m",
		test_render_formats_background_color: "<bg:blue>bg:blue</>" => "\x1b[44mbg:blue\x1b[0m",

		test_render_handles_unclosed_tags: "<bold>bold" => "\x1b[1mbold\x1b[0m",
		test_render_handles_trailing_text: "<bold>bold</> trailing" => "\x1b[1mbold\x1b[0m trailing",
		test_render_handles_nested_tags: "<blue><italic>blue italic</> blue</>" => "\x1b[3;34mblue italic\x1b[0m\x1b[34m blue\x1b[0m",
		test_render_drops_hidden_tags: "<blue>blue<hidden>hidden</></>" => "\x1b[34mblue\x1b[0m",

		test_render_ignores_escaped_tags: "\\<bold>\\bold" => "<bold>\\bold",
		test_render_keeps_backslash_in_text: "some\\ text" => "some\\ text",
		test_render_keeps_backslash_in_tag: "<bold \\ italic>bold italic</>" => "\x1b[1;3mbold italic\x1b[0m",
		test_render_keeps_trailing_backslash: "hello\\" => "hello\\",

		// Note that while the function works as expected, this result is not the intended one.
		test_render_keeps_existing_ansi: "\x1b[34m<dimmed>.</>git\x1b[0m<dimmed>/</>" => "\x1b[34m\x1b[2m.\x1b[0mgit\x1b[0m\x1b[2m/\x1b[0m",
	);

	macro_rules! make_len_test {
		( $($name:ident: $markup:expr => $length:expr,)* ) => {
			$(
				#[test]
				fn $name() {
					colored::control::set_override(true); // needed when running tests in CLion
					let length = len($markup);
					assert_eq!(length, $length);
				}
			)*
		}
	}

	make_len_test!(
		test_len_handles_ascii: "a" => 1,
		test_len_handles_latin_supplement: "é" => 1, // e+ ́(combining acute accent)
		test_len_handles_devanagari: "मैं" => 1, // m + ै(devanagari vowel sign ai) + ं(devanagari sign anusvara)

		test_len_handles_simple_emoji: "🤦" => 1, // 🤦(face palm emoji)
		test_len_handles_emoji_with_skin_tone: "🤦🏽" => 1, // ^ + 🏽(skin tone modifier)
		test_len_handles_extended_grapheme_cluster_emoji: "🤦🏽‍♂️" => 1, // ^ + ‍(zero-width joiner) + ♂(male sign) + ️(variation selector-16)

		test_len_handles_nerd_font: "" => 1, // nf-fa-folder

		test_len_ignores_tags: "<bold>bold</>" => 4,
		test_len_drops_hidden_text: "<blue>blue<hidden>hidden</></>" => 4,
	);

	macro_rules! make_render_and_measure_test {
		( $($name:ident: $markup:expr,)* ) => {
			$(
				#[test]
				fn $name() {
					colored::control::set_override(true); // needed when running tests in CLion
					// The single-pass helper must stay equivalent to calling
					// `render` and `len` separately.
					assert_eq!(render_and_measure($markup), (render($markup), len($markup)));
				}
			)*
		};
	}

	make_render_and_measure_test!(
		test_render_and_measure_matches_plain: "plain text",
		test_render_and_measure_matches_single_style: "<bold>bold</>",
		test_render_and_measure_matches_multiple_styles: "<bold italic>bold italic</>",
		test_render_and_measure_matches_nested_tags: "<blue><italic>blue italic</> blue</>",
		test_render_and_measure_matches_trailing_text: "<bold>bold</> trailing",
		test_render_and_measure_matches_hidden_text: "<blue>blue<hidden>hidden</></>",
		test_render_and_measure_matches_escaped_tags: "\\<bold>\\bold",
		test_render_and_measure_matches_unicode: "<bold>मैं</> 🤦🏽‍♂️",
	);
}
