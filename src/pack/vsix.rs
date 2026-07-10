use serde::Deserialize;

/// Represents an icon theme declared by a pack's `contributes.iconThemes` field.
pub struct ThemeEntry {
	/// the theme's `id`, used to disambiguate it in the `icon_pack` config;
	/// absent for the rare theme that omits it (still usable as a pack's sole
	/// theme, just not individually selectable)
	pub id: Option<String>,
	pub label: String,
	/// path to the theme file, relative to the pack root
	pub path: String,
}

#[derive(Deserialize)]
struct PackageJson {
	version: Option<String>,
	contributes: Option<Contributes>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contributes {
	#[serde(default)]
	icon_themes: Vec<IconThemeDef>,
}

#[derive(Deserialize)]
struct IconThemeDef {
	id: Option<String>,
	label: String,
	path: String,
}

/// Parse the `contributes.iconThemes` of a pack's `package.json`.
///
/// Returns an empty vector when the manifest declares no icon themes.
pub fn theme_entries(package_json: &str) -> Vec<ThemeEntry> {
	let parsed: PackageJson =
		json5::from_str(package_json).expect("icon pack package.json is valid");
	parsed
		.contributes
		.map(|c| c.icon_themes)
		.unwrap_or_default()
		.into_iter()
		.map(|d| ThemeEntry {
			id: d.id,
			label: d.label,
			path: d.path,
		})
		.collect()
}

/// Parse the `version` of a pack's `package.json`, empty when it declares none.
///
/// The version is folded into an icon's cache key so that updating a pack
/// invalidates its previously rendered icons.
pub fn version(package_json: &str) -> String {
	json5::from_str::<PackageJson>(package_json)
		.ok()
		.and_then(|p| p.version)
		.unwrap_or_default()
}

#[cfg(test)]
mod tests {
	use super::theme_entries;

	#[test]
	fn test_theme_entries_parsed() {
		let pkg = r#"{ "contributes": { "iconThemes": [
			{ "id": "cat-mocha", "label": "Catppuccin Mocha", "path": "./dist/mocha/theme.json" }
		] } }"#;
		let entries = theme_entries(pkg);
		assert_eq!(entries.len(), 1);
		assert_eq!(entries[0].id.as_deref(), Some("cat-mocha"));
		assert_eq!(entries[0].label, "Catppuccin Mocha");
		assert_eq!(entries[0].path, "./dist/mocha/theme.json");
	}

	#[test]
	fn test_theme_entries_absent_is_empty() {
		assert!(theme_entries(r#"{ "contributes": {} }"#).is_empty());
	}
}
