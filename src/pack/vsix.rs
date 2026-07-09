use crate::exc::Exc;
use serde::Deserialize;
use std::fs::{create_dir_all, File};
use std::io::{Cursor, Read, Write};
use std::path::Path;

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

/// Extract the `extension/` subtree of a `.vsix` (a zip) into `dest`, stripping
/// the `extension/` prefix. Non-`extension/` members are ignored.
pub fn extract(vsix: &[u8], dest: &Path) -> Result<(), Exc> {
	let mut archive = zip::ZipArchive::new(Cursor::new(vsix)).map_err(|e| Exc::Zip(Box::new(e)))?;
	for i in 0..archive.len() {
		let mut entry = archive.by_index(i).map_err(|e| Exc::Zip(Box::new(e)))?;
		let name = entry.name().to_string();
		let Ok(rel) = Path::new(&name).strip_prefix("extension") else {
			continue;
		};
		if entry.is_dir() || rel.as_os_str().is_empty() {
			continue;
		}
		let out = dest.join(rel);
		if let Some(parent) = out.parent() {
			create_dir_all(parent).map_err(Exc::Io)?;
		}
		let mut bytes = Vec::with_capacity(entry.size() as usize);
		entry.read_to_end(&mut bytes).map_err(Exc::Io)?;
		File::create(&out)
			.map_err(Exc::Io)?
			.write_all(&bytes)
			.map_err(Exc::Io)?;
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::{extract, theme_entries};
	use std::io::Write;

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

	#[test]
	fn test_extract_strips_extension_prefix() {
		// Build an in-memory .vsix (zip) with one `extension/` entry and one
		// entry outside it; only the former should be written, prefix-stripped.
		let mut buf = Vec::new();
		{
			let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
			let opts: zip::write::FileOptions<()> = zip::write::FileOptions::default();
			zip.start_file("extension/dist/theme.json", opts).unwrap();
			zip.write_all(b"{}").unwrap();
			zip.start_file("extension.vsixmanifest", opts).unwrap();
			zip.write_all(b"<xml/>").unwrap();
			zip.finish().unwrap();
		}
		let dir = std::env::temp_dir().join("pls-vsix-extract-test");
		std::fs::remove_dir_all(&dir).ok();
		extract(&buf, &dir).unwrap();
		assert!(dir.join("dist/theme.json").exists());
		assert!(!dir.join("extension.vsixmanifest").exists());
		std::fs::remove_dir_all(&dir).ok();
	}
}
