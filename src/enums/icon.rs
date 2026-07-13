use crate::PLS;
use crate::config::{Conf, LightTransform};
use crate::enums::ColorScheme;
use crate::gfx::{compute_hash, get_rgba, render_image, send_image};
use crate::models::IconTheme;
use crate::pack::resolve as resolve_theme;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex, OnceLock};

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

/// The icon theme `theme:<key>` icons resolve against for this run.
///
/// The active theme is a function of the config and the terminal color scheme,
/// both fixed for the lifetime of the process, so it is resolved once and reused
/// for every node. Icon resolution runs on many threads (nodes are rendered in
/// parallel), so this must be lock-free on the hot path — hence a [`OnceLock`]
/// rather than a locked cache. A `None` (no pack configured, or resolution
/// failed) is likewise cached, disabling `theme:` icons for the rest of the run.
static ACTIVE_THEME: OnceLock<Option<ActiveTheme>> = OnceLock::new();

/// The resolved icon theme for this run, together with the color scheme it was
/// chosen for and the pack version it came from.
struct ActiveTheme {
	/// the detected terminal color scheme, used to pick light-variant keys
	scheme: ColorScheme,
	/// the loaded theme; owned by the [`ACTIVE_THEME`] static, so borrows of its
	/// icon paths are effectively `'static`
	theme: IconTheme,
	/// the version of the pack, folded into each icon's cache key so an update
	/// invalidates previously cached renders
	version: String,
}

/// This enum contains the two formats of icons supported by `pls`.
pub enum Icon {
	/// a Nerd Font or emoji icon
	Text(String),
	/// an SVG image icon
	Image {
		/// the path to the SVG file to rasterise
		path: String,
		/// the icon's cache key (see [`compute_hash`])
		id: u32,
	},
}

/// Resolve an icon key against a theme, preferring the light-terminal variant
/// when appropriate.
///
/// In a light terminal with a configured `light_transform`, the rewritten key
/// is tried first (e.g. `rust` → `rust_light`), falling back to the original key
/// when the transform does not match or the theme has no such variant. In a dark
/// terminal, or without a transform, the original key is used directly.
///
/// # Arguments
///
/// * `theme` - the loaded icon theme to look the key up in
/// * `key` - the icon-definition key to resolve
/// * `color_scheme` - the detected terminal color scheme
/// * `light_transform` - the configured light-variant key rewrite, if any
fn resolve_key<'theme>(
	theme: &'theme IconTheme,
	key: &str,
	color_scheme: ColorScheme,
	light_transform: Option<&LightTransform>,
) -> Option<&'theme Path> {
	if color_scheme == ColorScheme::Light
		&& let Some(transform) = light_transform
	{
		let light_key = transform.apply(key);
		if light_key.as_ref() != key
			&& let Some(path) = theme.resolve(&light_key)
		{
			return Some(path);
		}
	}
	theme.resolve(key)
}

/// The active icon theme for this run, resolved once and cached (including a
/// negative result) for every subsequent call.
///
/// Resolution is lazy: it runs the first time a `theme:` icon is actually
/// needed, so runs that display no pack icons never query the terminal or read a
/// theme from disk. The terminal's color scheme is detected only when it can
/// change the outcome (`IconPackConfig::needs_scheme`).
///
/// # Arguments
///
/// * `conf` - the active configuration
fn active_theme(conf: &Conf) -> &'static Option<ActiveTheme> {
	ACTIVE_THEME.get_or_init(|| {
		let icon_pack = conf.icon_pack.as_ref()?;
		let scheme = if icon_pack.needs_scheme() {
			ColorScheme::default() // auto-detects
		} else {
			ColorScheme::Dark
		};
		let resolved = resolve_theme(&icon_pack.name, icon_pack.theme_id(scheme))?;
		let theme = IconTheme::load(&resolved.file)?;
		Some(ActiveTheme {
			scheme,
			theme,
			version: resolved.version,
		})
	})
}

impl Icon {
	/// Resolve an icon name to a concrete [`Icon`], or `None` if the name cannot
	/// be used for this node in the current terminal.
	///
	/// A name prefixed with `theme:` is an icon key and is resolved directly
	/// against the active theme (see [`Icon::theme_icon`]). Any other name is
	/// looked up in the [`icons`](Conf::icons) map; its value may itself be a
	/// `theme:` key, a literal `.svg` path, or a Nerd Font glyph. Theme keys and
	/// `.svg` paths yield `None` without graphics support, so resolution falls
	/// through to the next name in the list (typically a glyph).
	///
	/// # Arguments
	///
	/// * `name` - the icon name to resolve
	/// * `conf` - the active configuration
	pub fn resolve(name: &str, conf: &Conf) -> Option<Icon> {
		if let Some(key) = name.strip_prefix("theme:") {
			return Self::theme_icon(key, conf);
		}

		let value = conf.icons.get(name)?;
		if let Some(key) = value.strip_prefix("theme:") {
			return Self::theme_icon(key, conf);
		}
		if value.ends_with(".svg") {
			return Self::path_icon(value);
		}
		Some(Icon::Text(value.clone()))
	}

	/// Build an image icon from a literal SVG path (a `.svg` value in the
	/// [`icons`](Conf::icons) map). Its cache key is the path itself, since such
	/// an icon has no pack, theme or key. Yields `None` without graphics support.
	fn path_icon(path: &str) -> Option<Icon> {
		if !PLS.supports_gfx {
			return None;
		}
		Some(Icon::Image {
			path: path.to_string(),
			id: compute_hash(path, Icon::size()),
		})
	}

	/// Resolve an icon key (the body of a `theme:` name) to an image icon from
	/// the active icon theme.
	///
	/// The active theme (see [`active_theme`]) is resolved once per run; here its
	/// icon definitions are looked up by key (see [`resolve_key`]). Yields `None`
	/// when the terminal lacks graphics support, no pack is configured, or the
	/// key is absent.
	///
	/// The cache key combines the pack, its version, the theme, the color scheme
	/// and the icon key, so it survives the pack moving on disk and is
	/// invalidated when the pack is updated. The color scheme is included because
	/// a `light_transform` can resolve the same key to a different SVG per scheme.
	///
	/// # Arguments
	///
	/// * `key` - the part of a `theme:` name after the prefix
	/// * `conf` - the active configuration
	fn theme_icon(key: &str, conf: &Conf) -> Option<Icon> {
		if !PLS.supports_gfx {
			return None;
		}
		let active = active_theme(conf).as_ref()?;
		let icon_pack = conf.icon_pack.as_ref()?;
		let key = key.trim();

		let icon_path = resolve_key(
			&active.theme,
			key,
			active.scheme,
			icon_pack.light_transform(),
		)?;

		let theme = icon_pack.theme_id(active.scheme).unwrap_or_default();
		let scheme = match active.scheme {
			ColorScheme::Dark => "dark",
			ColorScheme::Light => "light",
		};
		let ident = format!(
			"{}@{}/{theme}/{scheme}/{key}",
			icon_pack.name, active.version
		);

		Some(Icon::Image {
			path: icon_path.to_string_lossy().into_owned(),
			id: compute_hash(&ident, Icon::size()),
		})
	}

	/// Get the size of the icon in pixels.
	///
	/// The icon size is determined by the width of a cell in the terminal
	/// multiplied by a scaling factor.
	pub fn size() -> u8 {
		let scale = std::env::var("PLS_ICON_SCALE")
			.ok()
			.and_then(|string| string.parse().ok())
			.unwrap_or(2.0f32)
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

			Icon::Image { path, id } => {
				let default = String::from("  ");

				// SVG icons support expanding environment variables in
				// the path for theming purposes.
				let path = match shellexpand::env(path) {
					Ok(path) => path,
					Err(_) => return default,
				};

				let size = Icon::size();

				let mut image_data_store = IMAGE_DATA.lock().unwrap();
				let data = image_data_store
					.entry(*id)
					.or_insert_with(|| ImageData { count: 0, id: 0 });

				data.count += 1;
				if data.count == 1 {
					// If the image is appearing for the first time in
					// this session, we send it to the terminal and get
					// an ID assigned to it.
					match get_rgba(*id, &PathBuf::from(path.as_ref()), size) {
						Some(rgba_data) => {
							data.id = send_image(*id, size, &rgba_data).unwrap();
						}
						None => return default,
					}
				}
				render_image(data.id, size, data.count)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Icon, resolve_key};
	use crate::config::{Conf, LightTransform};
	use crate::enums::ColorScheme;
	use crate::models::IconTheme;
	use std::path::Path;

	/// A theme with a base `rust` icon and a `rust_light` variant.
	fn theme() -> IconTheme {
		IconTheme::from_json_str(
			r#"{ "iconDefinitions": {
				"rust": { "iconPath": "./rust.svg" },
				"rust_light": { "iconPath": "./rust_light.svg" }
			} }"#,
			Path::new("/base"),
		)
		.unwrap()
	}

	/// The Material-style `<key>` → `<key>_light` suffix rewrite.
	fn suffix() -> LightTransform {
		"s/$/_light/".parse().unwrap()
	}

	#[test]
	fn test_resolve_key_dark_ignores_transform() {
		assert_eq!(
			resolve_key(&theme(), "rust", ColorScheme::Dark, Some(&suffix())),
			Some(Path::new("/base/rust.svg"))
		);
	}

	#[test]
	fn test_resolve_key_light_prefers_variant() {
		assert_eq!(
			resolve_key(&theme(), "rust", ColorScheme::Light, Some(&suffix())),
			Some(Path::new("/base/rust_light.svg"))
		);
	}

	#[test]
	fn test_resolve_key_light_falls_back_when_no_variant() {
		// `json` has no `json_light`, so the bare key is used.
		let theme = IconTheme::from_json_str(
			r#"{ "iconDefinitions": { "json": { "iconPath": "./json.svg" } } }"#,
			Path::new("/base"),
		)
		.unwrap();
		assert_eq!(
			resolve_key(&theme, "json", ColorScheme::Light, Some(&suffix())),
			Some(Path::new("/base/json.svg"))
		);
	}

	#[test]
	fn test_resolve_key_light_without_transform() {
		assert_eq!(
			resolve_key(&theme(), "rust", ColorScheme::Light, None),
			Some(Path::new("/base/rust.svg"))
		);
	}

	// The `theme:` branch of `Icon::resolve` reads the `PLS` global (graphics
	// support), so only the graphics-independent paths are unit-tested here; the
	// SVG resolution itself is covered by the pure `IconTheme` and config tests.

	#[test]
	fn test_resolve_glyph_name() {
		// A non-SVG name never touches graphics support (short-circuit).
		let conf = Conf::default();
		assert!(matches!(Icon::resolve("rust", &conf), Some(Icon::Text(_))));
	}

	#[test]
	fn test_resolve_unknown_name() {
		let conf = Conf::default();
		assert!(Icon::resolve("does-not-exist", &conf).is_none());
	}
}
