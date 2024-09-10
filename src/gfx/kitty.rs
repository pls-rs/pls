use base64::prelude::*;
use log::debug;
use std::env;

/// Check if the terminal supports Kitty's terminal graphics protocol.
///
/// There are ways to detect this support using CSI sequences, but they
/// are not reliable. Additionally this approach is faster as there is
/// no need to wait or poll for the terminal's response.
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
/// * `rgba_data` - the RGBA data to render
/// * `size` - the size of the image, in pixels
/// * `width` - the width of the icon cell, in columns
pub fn render_image(rgba_data: &[u8], size: u8, width: u8) -> String {
	const CHUNK_SIZE: usize = 4096;

	let encoded = BASE64_STANDARD.encode(rgba_data);
	let mut iter = encoded.chars().peekable();

	let first_chunk: String = iter.by_ref().take(CHUNK_SIZE).collect();
	let mut output = format!("\x1b_Gf=32,t=d,a=T,C=1,m=1,s={size},v={size};{first_chunk}\x1b\\");

	while iter.peek().is_some() {
		let chunk: String = iter.by_ref().take(CHUNK_SIZE).collect();
		output.push_str(&format!("\x1b_Gm=1;{chunk}\x1b\\"));
	}
	output.push_str("\x1b_Gm=0;\x1b\\");

	output.push_str(&format!("\x1b[{}C", width + 1));

	output
}
