use crate::fmt::format::fmt;
use std::iter::Peekable;
use std::str::Chars;
use unicode_segmentation::UnicodeSegmentation;

const ESCAPE: char = '\\';
const TAG_OPEN: char = '<';
const TAG_CLOSE: char = '>';

/// Given a peekable iterable of characters, get a string comprised of all
/// characters that satisfy the given condition.
///
/// # Arguments
///
/// * `tokens` - the peekable iterator of characters to consume from
/// * `predicate` - the condition each character must satisfy to be consumed
fn select_while<F>(tokens: &mut Peekable<Chars>, predicate: F) -> String
where
	F: Fn(char) -> bool,
{
	let mut selected = String::new();
	while tokens.peek().is_some_and(|ch| predicate(*ch)) {
		selected.push(tokens.next().unwrap());
	}
	selected
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
fn reduce_markup<S, T, F>(markup: S, init: T, reducer: F) -> T
where
	S: AsRef<str>,
	F: Fn(&Vec<Vec<String>>, &mut String, T) -> T,
{
	let mut stack: Vec<Vec<String>> = Vec::new(); // the list of currently active tags
	let mut curr: String = String::default(); // the current continuous text block

	let mut acc = init; // initialise the accumulator

	let mut tokens = markup.as_ref().chars().peekable();
	while let Some(next_char) = tokens.peek() {
		match *next_char {
			ESCAPE => {
				tokens.next(); // Consume `BACKSLASH`.
				match tokens.peek() {
					Some(then_char) if *then_char == TAG_OPEN => {
						curr.push(TAG_OPEN);
						tokens.next(); // Consume `TAG_OPEN`.
					}
					_ => curr.push(ESCAPE),
				}
			}
			TAG_OPEN => {
				// Handle the current run of continuous text.
				acc = reducer(&stack, &mut curr, acc);

				tokens.next(); // Consume `TAG_OPEN`.
				let tag = select_while(&mut tokens, |c| c != TAG_CLOSE);
				if tag == "/" {
					stack.pop();
				} else {
					stack.push(tag.split(' ').map(String::from).collect());
				}
				tokens.next(); // Consume `TAG_CLOSE`.
			}
			_ => {
				let text = select_while(&mut tokens, |c| c != TAG_OPEN && c != ESCAPE);
				curr.push_str(&text);
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
#[allow(clippy::needless_borrows_for_generic_args)]
pub fn render<S>(markup: S) -> String
where
	S: AsRef<str>,
{
	reduce_markup(markup, String::default(), |stack, curr, acc| {
		let mut acc = acc;
		if !curr.is_empty() {
			let directives: Vec<_> = stack.iter().flatten().collect();
			if !directives.contains(&&String::from("hidden")) {
				acc.push_str(&fmt(&curr, &directives));
			}
			curr.clear();
		}
		acc
	})
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
	reduce_markup(markup, 0, |stack, curr, acc| {
		let count = if curr.is_empty() || stack.iter().flatten().any(|tag| tag == "hidden") {
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
	use super::{len, render, select_while};

	macro_rules! make_select_while_test {
        ( $($name:ident: $predicate:expr => $selected:expr,)* ) => {
            $(
                #[test]
                fn $name() {
					colored::control::set_override(true); // needed when running tests in CLion
                    let mut tokens = "Hello, World!".chars().peekable();
                    let selected = select_while(&mut tokens, $predicate);
                    assert_eq!(selected, $selected);
                }
            )*
        };
    }

	make_select_while_test!(
		test_select_selects_part_of_string: |c| c != ',' => "Hello",
		test_select_consumes_till_end_of_string: |c| c != '?' => "Hello, World!",
	);

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
}
