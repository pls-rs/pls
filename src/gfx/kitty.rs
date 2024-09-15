use crate::PLS;
use base64::prelude::*;
use log::debug;
use regex::Regex;
use std::env;
use std::sync::LazyLock;

static KITTY_IMAGE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\x1b_G.*?\x1b\\").unwrap());
const CHUNK_SIZE: usize = 4096;

/// Check if the terminal supports Kitty's terminal graphics protocol.
///
/// There are ways to detect this support using CSI sequences, but since
/// only a few terminals support it, checking for those terminals is
/// easier, and faster as there is no need to wait or poll for the
/// terminal's response.
///
/// We detect if the user is using Kitty or WezTerm, which currently
/// are the only two terminals which support Kitty's terminal graphic
/// protocol.
pub fn is_supported() -> bool {
	// Detect Kitty by the `TERM` or `TERMINAL` environment variables.
	// We assume Kitty, regardless of version and config, supports
	// graphics.
	for env_var in ["TERM", "TERMINAL"] {
		if let Ok(env_val) = env::var(env_var) {
			let env_val = env_val.to_ascii_lowercase();
			if env_val.contains("kitty") {
				debug!("Detected Kitty via {}.", env_var);
				return true;
			}
		}
	}

	// Detect WezTerm with the `TERM_PROGRAM` environment variable and
	// check the version for graphics support. This does not account for
	// the fact that Kitty support might be turned off.
	if let Ok(term_program) = env::var("TERM_PROGRAM") {
		if term_program == "WezTerm" {
			if let Ok(version) = env::var("TERM_PROGRAM_VERSION") {
				if &*version >= "20220105-201556-91a423da" {
					debug!("Detected Wezterm with graphics support.");
					return true;
				}
			}
		}
	}

	debug!("Graphics not supported.");
	false
}

/// Send the RGBA data to the terminal for immediate rendering.
///
/// To achieve this, we must send the following control sequence:
///
/// * f = 32 signals that data will be 32-bit RGBA
/// * t = d signals that data will be within the control sequence
/// * a = T signals that image should be immediately displayed
/// * C = 1 signals that cursor should not be moved
/// * m = 1 if more data follows, 0 if this is the last chunk
/// * s = size for width (pixels)
/// * v = size for height (pixels)
///
/// The image is sent in chunks of 4096 bytes. The last chunk has the
/// `m` parameter set to 0.
///
/// The image is rendered in a way that the cursor does not move. Then
/// we move the cursor by as many cells as the icon width (and a space).
///
/// # Arguments
///
/// * `id` - the unique ID of the image
/// * `count` - the number of times this image has appeared so far
/// * `size` - the size of the image, in pixels
/// * `rgba_data` - the RGBA data to render
pub fn render_image(id: u32, count: u8, size: u8, rgba_data: Option<&[u8]>) -> String {
	let cell_height = PLS.window.as_ref().unwrap().cell_height();
	let off_y = if cell_height > size {
		(cell_height - size) / 2
	} else {
		0
	};

	let mut output = String::new();

	// If data is provided, the image is new, so we transmit it with the
	// control `a=t`.
	if let Some(rgba_data) = rgba_data {
		let encoded = BASE64_STANDARD.encode(rgba_data);
		let mut iter = encoded.chars().peekable();

		let first_chunk: String = iter.by_ref().take(CHUNK_SIZE).collect();

		// TODO: By sending fresh data for existing IDs, the images
		// shown in previous usages of `pls` disappear.
		output.push_str(&format!(
			"\x1b_G\
			f=32,t=d,a=t,m=1,q=2,i={id},s={size},v={size},Y={off_y};\
			{first_chunk}\
			\x1b\\"
		));

		while iter.peek().is_some() {
			let chunk: String = iter.by_ref().take(CHUNK_SIZE).collect();
			output.push_str(&format!("\x1b_Gm=1;{chunk}\x1b\\"));
		}

		output.push_str("\x1b_Gm=0,q=2;\x1b\\");
	}

	// Once the data is sent, we render the previously transmitted image
	// with the control `a=p`.
	output.push_str(&format!(
		"\x1b_G\
		a=p,C=1,q=2,i={id},p={count},s={size},v={size},Y={off_y};\
		\x1b\\"
	));

	output.push_str("\x1b[2C");

	output
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

#[cfg(test)]
mod tests {
	use super::strip_image;

	#[test]
	fn test_remove_image_substrings() {
		let text = "\x1b_Gf=32;AAAA\x1b\\Hello, World!";
		assert_eq!(strip_image(text), "Hello, World!");
	}
}
