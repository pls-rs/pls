use crate::gfx::query_raw;
use log::debug;
use regex::Regex;
use std::env;
use std::sync::{LazyLock, OnceLock};

/// Matches the `rgb:RRRR/GGGG/BBBB` payload of an OSC 11 response. Each channel
/// may be 1–4 hex digits.
static BG_RGB: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"rgb:([0-9a-fA-F]{1,4})/([0-9a-fA-F]{1,4})/([0-9a-fA-F]{1,4})").unwrap()
});

/// The process-wide detected color scheme, computed at most once on first use.
static DETECTED: OnceLock<ColorScheme> = OnceLock::new();

/// This enum contains the color schemes that the terminal can be in.
///
/// `pls` uses this to pick between a dark and a light icon pack so that SVG
/// icons (whose colors are baked in) stay visible, mirroring the way Nerd Font
/// glyphs inherit the terminal foreground color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorScheme {
	Dark,
	Light,
}

impl ColorScheme {
	/// Get the terminal's color scheme, detecting and caching it on first call.
	///
	/// Detection is lazy: it only runs the first time a light/dark decision is
	/// actually needed (e.g. when resolving a `theme:` icon).
	pub fn detect() -> Self {
		*DETECTED.get_or_init(|| Self::decide(env::var("PLS_COLOR_SCHEME").ok(), Self::query_bg))
	}

	/// Decide the color scheme from the available signals, in order of precedence.
	///
	/// 1. The `PLS_COLOR_SCHEME` override (`light`/`dark`), if valid.
	/// 2. The queried terminal background, if available.
	/// 3. A `Dark` default.
	///
	/// # Arguments
	///
	/// * `override_val` - the value of the `PLS_COLOR_SCHEME` environment variable
	/// * `query` - a fallback that queries the terminal background
	fn decide(override_val: Option<String>, query: impl FnOnce() -> Option<Self>) -> Self {
		if let Some(val) = override_val {
			match val.to_ascii_lowercase().as_str() {
				"light" => return Self::Light,
				"dark" => return Self::Dark,
				_ => debug!("Ignoring invalid PLS_COLOR_SCHEME value {val:?}."),
			}
		}
		query().unwrap_or(Self::Dark)
	}

	/// Query the terminal background color via OSC 11 and classify it.
	fn query_bg() -> Option<Self> {
		let res = query_raw("\x1b]11;?\x1b\\", 200)
			.map_err(|e| debug!("Could not query terminal background: {e}"))
			.ok()?;
		Self::classify_bg(&res)
	}

	/// Classify an OSC 11 background-color response as light or dark.
	///
	/// The relative luminance is computed from the parsed RGB channels; a value
	/// above 0.5 is considered light.
	///
	/// # Arguments
	///
	/// * `response` - the raw OSC 11 response, e.g. `\x1b]11;rgb:1c1c/1c1c/1c1c\x1b\\`
	fn classify_bg(response: &str) -> Option<Self> {
		let caps = BG_RGB.captures(response)?;
		let channel = |i: usize| -> f32 {
			let hex = &caps[i];
			let max = 16f32.powi(hex.len() as i32) - 1.0;
			u32::from_str_radix(hex, 16).unwrap() as f32 / max
		};

		let luminance = 0.2126 * channel(1) + 0.7152 * channel(2) + 0.0722 * channel(3);
		Some(if luminance > 0.5 {
			Self::Light
		} else {
			Self::Dark
		})
	}
}

#[cfg(test)]
mod tests {
	use super::ColorScheme;

	#[test]
	fn test_classify_white_is_light() {
		assert_eq!(
			ColorScheme::classify_bg("\x1b]11;rgb:ffff/ffff/ffff\x1b\\"),
			Some(ColorScheme::Light)
		);
	}

	#[test]
	fn test_classify_black_is_dark() {
		assert_eq!(
			ColorScheme::classify_bg("\x1b]11;rgb:0000/0000/0000\x1b\\"),
			Some(ColorScheme::Dark)
		);
	}

	#[test]
	fn test_classify_eight_bit_channels() {
		assert_eq!(
			ColorScheme::classify_bg("rgb:1c/1c/1c"),
			Some(ColorScheme::Dark)
		);
		assert_eq!(
			ColorScheme::classify_bg("rgb:ee/ee/ee"),
			Some(ColorScheme::Light)
		);
	}

	#[test]
	fn test_classify_garbage_is_none() {
		assert_eq!(ColorScheme::classify_bg("no color here"), None);
	}

	#[test]
	fn test_decide_override_wins() {
		assert_eq!(
			ColorScheme::decide(Some("light".into()), || Some(ColorScheme::Dark)),
			ColorScheme::Light
		);
		assert_eq!(
			ColorScheme::decide(Some("DARK".into()), || Some(ColorScheme::Light)),
			ColorScheme::Dark
		);
	}

	#[test]
	fn test_decide_falls_back_to_query() {
		assert_eq!(
			ColorScheme::decide(None, || Some(ColorScheme::Light)),
			ColorScheme::Light
		);
		assert_eq!(
			ColorScheme::decide(Some("bogus".into()), || Some(ColorScheme::Light)),
			ColorScheme::Light
		);
	}

	#[test]
	fn test_decide_defaults_to_dark() {
		assert_eq!(ColorScheme::decide(None, || None), ColorScheme::Dark);
	}
}
