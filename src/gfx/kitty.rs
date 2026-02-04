use crate::exc::Exc;
use crate::PLS;
use base64::prelude::*;
use crossterm::terminal::*;
use log::debug;
use regex::Regex;
use std::env;
use std::sync::LazyLock;

const CHUNK_SIZE: usize = 4096;

static KITTY_IMAGE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\x1b_G.*?\x1b\\").unwrap());
static IMAGE_ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"i=(?P<id>\d+)").unwrap());

/// Check if the terminal supports Kitty's terminal graphics protocol.
///
/// Since Kitty support is restricted to a handful of terminals, all of
/// which can be easily and reliably detected, we use that to determine
/// if the terminal supports graphics.
///
/// Additionally, testing for Kitty support using a CSI sequence is
/// unreliable and breaks down in some cases like the macOS Terminal or
/// `to-html`.
pub fn is_supported() -> bool {
	// Detect Kitty by the `TERM` or `TERMINAL` environment variables.
	for env_var in ["TERM", "TERMINAL"] {
		if let Ok(env_val) = env::var(env_var) {
			let env_val = env_val.to_ascii_lowercase();
			if env_val.contains("kitty") {
				debug!("Detected Kitty terminal.");
				return true;
			}
		}
	}

	// Detect WezTerm and Ghostty by the `TERM_PROGRAM` environment variable.
	if let Ok(term_program) = env::var("TERM_PROGRAM") {
		if term_program == "WezTerm" {
			debug!("Detected WezTerm terminal.");
			return true;
		} else if term_program == "ghostty" {
			debug!("Detected Ghostty terminal.");
			return true;
		}
	}

	debug!("Graphics not supported.");
	false
}

/// Send the RGBA data to the terminal and get an ID for the image.
///
/// The image is sent in chunks of 4096 bytes. The last chunk has the
/// `m` parameter set to 0. The terminal then assigns our image an ID,
/// instead of us determining one.
///
/// In this stage, we do not show the image (it will be shown in a later
/// step) so placement controls are not required.
///
/// # Arguments
///
/// * `hash` - the hash of the image data
/// * `size` - the size of the image, in pixels
/// * `rgba_data` - the RGBA data to send
pub fn send_image(hash: u32, size: u8, rgba_data: &[u8]) -> Result<u32, Exc> {
	let mut query = String::new();

	let encoded = BASE64_STANDARD.encode(rgba_data);
	let mut iter = encoded.chars().peekable();

	let first_chunk: String = iter.by_ref().take(CHUNK_SIZE).collect();
	query.push_str(&format!(
		"\x1b_G\
		a=t,I={hash},s={size},v={size},t=d,f=32,m=1;\
		{first_chunk}\
		\x1b\\"
	));

	while iter.peek().is_some() {
		let chunk: String = iter.by_ref().take(CHUNK_SIZE).collect();
		query.push_str(&format!("\x1b_Gm=1;{chunk}\x1b\\"));
	}

	query.push_str("\x1b_Gm=0;\x1b\\");

	let res = query_raw(&query, 200)?;
	IMAGE_ID
		.captures(&res)
		.map(|cap| cap["id"].parse().unwrap())
		.ok_or(Exc::Other(String::from("Could not extract image ID.")))
}

/// Render the image with the given ID to the screen.
///
/// In this stage, we do not transmit the image (it has already been
/// done) so transmission controls are not required.
///
/// The image is rendered in a way that the cursor does not move. Then
/// we move the cursor by as many cells as the icon width (and a space).
///
/// # Arguments
///
/// * `id` - the unique ID of the image
/// * `size` - the size of the image, in pixels
/// * `count` - the number of times this image has appeared so far
pub fn render_image(id: u32, size: u8, count: u8) -> String {
	let cell_height = PLS.window.as_ref().unwrap().cell_height();
	let off_y = if cell_height > size {
		(cell_height - size) / 2
	} else {
		0
	};

	format!(
		"\x1b_G\
		a=p,i={id},s={size},v={size},p={count},C=1,Y={off_y},q=2;\
		\x1b\\\
		\x1b[2C"
	)
}

/// Strip the image data from the text.
///
/// This function removes the all terminal graphics from the string,
/// leaving only the text content.
///
/// # Arguments
///
/// * `text` - the text to strip the image data from
pub fn strip_image<S>(text: S) -> String
where
	S: AsRef<str>,
{
	KITTY_IMAGE
		.replace_all(text.as_ref(), "")
		.replace("\x1b[2C", "  ")
		.to_string()
}

/// Perform the given query in the terminal raw mode.
///
/// This function enables the terminal raw mode, performs the query,
/// records the response and then disables the terminal raw mode. The
/// response is returned as a string.
///
/// # Arguments
///
/// * `query` - the query to perform
/// * `timeout_ms` - the timeout in milliseconds
fn query_raw(query: &str, timeout_ms: u64) -> Result<String, Exc> {
	enable_raw_mode().map_err(Exc::Io)?;
	let res = xterm_query::query_osc(query, timeout_ms).map_err(|e| Exc::Xterm(Box::new(e)));
	disable_raw_mode().map_err(Exc::Io)?;

	res
}

#[cfg(test)]
mod tests {
	use super::strip_image;

	#[test]
	fn test_remove_image_substrings() {
		let text = "\x1b_Gf=32;AAAA\x1b\\Hello, World!";
		assert_eq!(strip_image(text), "Hello, World!");
	}
}
