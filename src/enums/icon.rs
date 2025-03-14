use crate::gfx::{compute_hash, get_rgba, render_image, send_image};
use crate::PLS;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

struct ImageData {
	/// the ID assigned by the terminal to our image
	///
	/// This is different from the hash of the image data. Allowing the
	/// terminal to choose an ID prevents new invocations of `pls` from
	/// overwriting IDs of images that were displayed by previous
	/// invocations.
	id: u32,
	/// the number of times the image has been displayed
	///
	/// This generates new placement IDs for the images. This is
	/// required specifically because WezTerm has a bug where not
	/// setting unique placement IDs overwrites placements instead of
	/// creating new ones.
	count: u8,
}

static IMAGE_DATA: LazyLock<Mutex<HashMap<u32, ImageData>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));

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

		(scale * PLS.window.as_ref().unwrap().cell_width() as f32) // Convert to px.s
			.round() as u8
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
				let default = String::from("  ");

				// SVG icons support expanding environment variables in
				// the path for theming purposes.
				let path = match shellexpand::env(path) {
					Ok(path) => path,
					Err(_) => return default,
				};

				let size = Icon::size();
				let hash = compute_hash(&PathBuf::from(path.as_ref()), size);

				let mut image_data_store = IMAGE_DATA.lock().unwrap();
				let data = image_data_store
					.entry(hash)
					.or_insert_with(|| ImageData { count: 0, id: 0 });

				data.count += 1;
				if data.count == 1 {
					// If the image is appearing for the first time in
					// this session, we send it to the terminal and get
					// an ID assigned to it.
					match get_rgba(hash, &PathBuf::from(path.as_ref()), size) {
						Some(rgba_data) => {
							data.id = send_image(hash, size, &rgba_data).unwrap();
						}
						None => return default,
					}
				}
				render_image(data.id, size, data.count)
			}
		}
	}
}
