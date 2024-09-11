use crate::gfx::{get_rgba, render_image};
use crate::PLS;
use std::path::PathBuf;

/// This enum contains the two formats of icons supported by `pls`.
pub enum Icon {
	/// a Nerd Font or emoji icon
	Text(String),
	/// the path to an SVG icon
	Image(String),
}

impl From<&str> for Icon {
	fn from(s: &str) -> Self {
		if s.ends_with(".svg") {
			Icon::Image(s.to_string())
		} else {
			Icon::Text(s.to_string())
		}
	}
}

impl Icon {
	/// Get the size of the icon in pixels.
	///
	/// The icon size is determined by the width of a cell in the terminal
	/// multiplied by a scaling factor.
	pub fn size() -> u8 {
		let scale = std::env::var("PLS_ICON_SCALE")
			.ok()
			.and_then(|string| string.parse().ok())
			.unwrap_or(1.0f32)
			.min(2.0); // We only allocate two cells for an icon.

		return (scale * PLS.window.as_ref().unwrap().cell_width() as f32) // Convert to px.s
			.round() as u8;
	}

	/// Get the output of the icon using the appropriate method:
	///
	/// * For text icons, it generates the markup string with the
	///   directives.
	/// * For image icons, it generates the Kitty terminal graphics APC
	///   sequence. If that fails, it falls back to a blank text icon.
	///
	/// The formatting directives for textual icons are a subset of the
	/// formatting directives for text.
	///
	/// # Arguments
	///
	/// * `directives` - the formatting directives to apply to text
	pub fn render(&self, text_directives: &str) -> String {
		match self {
			Icon::Text(text) => {
				// Nerd Font icons look weird with underlines and
				// synthesised italics.
				let directives = text_directives
					.replace("underline", "")
					.replace("italic", "");
				// We leave a space after the icon to allow Nerd Font
				// icons that are slightly bigger than one cell to be
				// displayed correctly.
				format!("<{directives}>{text:<1} </>")
			}

			Icon::Image(path) => {
				// SVG icons support expanding environment variables in
				// the path for theming purposes.
				if let Ok(path) = shellexpand::env(path) {
					let size = Icon::size();
					if let Some(rgba_data) = get_rgba(&PathBuf::from(path.as_ref()), size) {
						return render_image(&rgba_data, size);
					}
				}
				// This would be exactly as if the node had no icon.
				Icon::Text(String::default()).render(text_directives)
			}
		}
	}
}
