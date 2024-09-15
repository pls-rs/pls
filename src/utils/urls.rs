//! This module contains code for working with URLs.
//!
//! The public interface of the module consists of one function:
//!
//! * [`get_osc`]

use std::fmt::Display;

/// Get the OSC-8 escape sequence for the given URL.
///
/// Many terminal emulators support OSC-8, which allows terminals to
/// render hyperlinks that have a display text that points to a URL
/// which is not displayed.
///
/// # Arguments
///
/// * `url` - the URL to generate the escape sequence for
/// * `text` - the text to display for the hyperlink
pub fn get_osc<S>(url: S, text: Option<S>) -> String
where
	S: AsRef<str> + Display,
{
	let text = text.as_ref().unwrap_or(&url);
	format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
}

#[cfg(test)]
mod tests {
	use super::get_osc;

	macro_rules! make_test {
		( $($name:ident: $url:expr, $text:expr => $expected:expr,)* ) => {
			$(
				#[test]
				fn $name() {
					assert_eq!(get_osc($url, $text), $expected);
				}
			)*
		};
	}

	make_test!(
		test_url_and_test: "https://example.com", Some("Example") => "\x1b]8;;https://example.com\x1b\\Example\x1b]8;;\x1b\\",
		test_url_only: "https://example.com", None => "\x1b]8;;https://example.com\x1b\\https://example.com\x1b]8;;\x1b\\",
	);
}
