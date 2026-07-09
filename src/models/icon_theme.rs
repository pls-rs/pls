use log::debug;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/// A VS Code icon theme, reduced to just the icon definitions that map a key to
/// an SVG file.
///
/// `pls` consumes only the `iconDefinitions` of a VS Code icon theme.
///
/// Icon assignment stays spec-driven, so the theme is merely a named source of
/// SVGs referenced via the `theme:<key>` syntax in `icons` lists.
pub struct IconTheme {
	/// mapping of icon-definition key to the absolute path of its SVG
	defs: HashMap<String, PathBuf>,
}

/// The subset of a VS Code icon-theme JSON that `pls` deserializes.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ThemeJson {
	#[serde(default)]
	icon_definitions: HashMap<String, IconDef>,
}

/// A single entry of `iconDefinitions`.
///
/// Only `iconPath` is read; font-glyph definitions (`fontCharacter` etc.) have
/// no `iconPath` and are skipped.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconDef {
	icon_path: Option<String>,
}

impl IconTheme {
	/// Parse an icon theme from the contents of a theme file.
	///
	/// `iconPath`s are resolved relative to `base`, which must be the directory
	/// containing the theme file. The input may be JSONC (comments and trailing
	/// commas are tolerated).
	///
	/// # Arguments
	///
	/// * `contents` - the (possibly JSONC) contents of the theme file
	/// * `base` - the directory against which `iconPath`s are resolved
	pub fn from_json_str(contents: &str, base: &Path) -> Option<Self> {
		let theme: ThemeJson = json5::from_str(contents)
			.map_err(|e| debug!("Could not parse icon theme: {e}"))
			.ok()?;

		let defs = theme
			.icon_definitions
			.into_iter()
			.filter_map(|(key, def)| def.icon_path.map(|path| (key, base.join(path))))
			.collect();

		Some(Self { defs })
	}

	/// Load and parse the icon theme at the given theme-file path.
	///
	/// Returns `None` when the file cannot be read or parsed, logging the reason
	/// at debug level. The caller loads the active theme once per run and holds
	/// onto it, so this is not itself memoised.
	///
	/// # Arguments
	///
	/// * `theme_file` - the absolute path to the theme file
	pub fn load(theme_file: &Path) -> Option<Self> {
		read_to_string(theme_file)
			.map_err(|e| debug!("Could not read icon theme {theme_file:?}: {e}"))
			.ok()
			.zip(theme_file.parent())
			.and_then(|(contents, base)| Self::from_json_str(&contents, base))
	}

	/// Get the absolute path of the SVG for the given icon-definition key.
	pub fn resolve(&self, key: &str) -> Option<&Path> {
		self.defs.get(key).map(PathBuf::as_path)
	}
}

#[cfg(test)]
mod tests {
	use super::IconTheme;
	use std::path::Path;

	#[test]
	fn test_parses_icon_definitions() {
		let json = r#"{
			// A comment, as allowed in JSONC.
			"iconDefinitions": {
				"rust": { "iconPath": "./icons/rust.svg" },
				"json": { "iconPath": "./icons/json.svg" }, // trailing comma below
			}
		}"#;
		let theme = IconTheme::from_json_str(json, Path::new("/packs/theme")).unwrap();
		assert_eq!(
			theme.resolve("rust"),
			Some(Path::new("/packs/theme/icons/rust.svg"))
		);
		assert_eq!(
			theme.resolve("json"),
			Some(Path::new("/packs/theme/icons/json.svg"))
		);
	}

	#[test]
	fn test_skips_font_glyph_definitions() {
		let json = r##"{
			"iconDefinitions": {
				"_R": { "fontCharacter": "\\E001", "fontColor": "#519aba" },
				"rust": { "iconPath": "./rust.svg" }
			}
		}"##;
		let theme = IconTheme::from_json_str(json, Path::new("/base")).unwrap();
		assert_eq!(theme.resolve("_R"), None);
		assert_eq!(theme.resolve("rust"), Some(Path::new("/base/rust.svg")));
	}

	#[test]
	fn test_missing_key_resolves_to_none() {
		let json = r#"{ "iconDefinitions": { "rust": { "iconPath": "./rust.svg" } } }"#;
		let theme = IconTheme::from_json_str(json, Path::new("/base")).unwrap();
		assert_eq!(theme.resolve("missing"), None);
	}

	#[test]
	fn test_invalid_json_returns_none() {
		assert!(IconTheme::from_json_str("not json", Path::new("/base")).is_none());
	}

	#[test]
	fn test_load_reads_from_disk() {
		let dir = std::env::temp_dir().join("pls-icon-theme-load-test");
		std::fs::create_dir_all(&dir).unwrap();
		let theme_file = dir.join("theme.json");
		std::fs::write(
			&theme_file,
			r#"{ "iconDefinitions": { "rust": { "iconPath": "./rust.svg" } } }"#,
		)
		.unwrap();

		let theme = IconTheme::load(&theme_file).unwrap();
		assert_eq!(theme.resolve("rust"), Some(dir.join("rust.svg").as_path()));

		std::fs::remove_dir_all(&dir).ok();
	}

	#[test]
	fn test_load_missing_file_returns_none() {
		assert!(IconTheme::load(Path::new("/nonexistent/theme.json")).is_none());
	}
}
