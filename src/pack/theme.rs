use crate::exc::Exc;
use crate::pack::packs_dir;
use crate::vsc::{ExtPackage, IconThemeDef};
use log::debug;
use std::path::{Path, PathBuf};

/// A resolved icon theme: its theme file and the version of the pack it came
/// from (the latter feeds the icon cache key, see
/// [`compute_hash`](crate::gfx::compute_hash)).
pub struct ResolvedTheme {
	pub file: PathBuf,
	pub version: String,
}

/// Resolve an installed pack and theme to its theme file and pack version.
///
/// Returns `None` when resolution fails, logging the reason at debug level. The
/// caller resolves the active theme once per run, so this is not itself memoised.
///
/// # Arguments
///
/// * `pack_id` - the pack's ID (`<publisher>.<name>`)
/// * `theme_id` - the ID of the theme within the pack, if disambiguation is
///   needed (packs contributing a single theme need none)
pub fn resolve(pack_id: &str, theme_id: Option<&str>) -> Option<ResolvedTheme> {
	resolve_ref(pack_id, theme_id)
		.map_err(|e| debug!("Could not resolve icon theme: {e}"))
		.ok()
}

/// Resolve against the installed packs directory.
fn resolve_ref(pack_id: &str, theme_id: Option<&str>) -> Result<ResolvedTheme, Exc> {
	resolve_in(&packs_dir()?, pack_id, theme_id)
}

/// Resolve against `packs_root`, the directory holding installed packs. Split
/// out from [`resolve`] so resolution is testable without a real data directory.
fn resolve_in(
	packs_root: &Path,
	pack_id: &str,
	theme_id: Option<&str>,
) -> Result<ResolvedTheme, Exc> {
	let root = packs_root.join(pack_id);
	if !root.is_dir() {
		return Err(Exc::Other(format!(
			"Icon pack {pack_id} is not installed — run `pls icon-pack add {pack_id}`."
		)));
	}
	let ext_package = ExtPackage::try_from(root.as_path())?;
	let entries = ext_package.icon_themes();

	let entry = match theme_id {
		Some(id) => entries
			.iter()
			.find(|t| t.id.as_deref() == Some(id))
			.ok_or_else(|| {
				Exc::Other(format!(
					"Icon pack {pack_id} has no theme {id:?}. Available: {}.",
					available(entries)
				))
			})?,
		None => match entries {
			[entry] => entry,
			[] => {
				return Err(Exc::Other(format!(
					"Icon pack {pack_id} contributes no icon themes."
				)));
			}
			_ => {
				return Err(Exc::Other(format!(
					"Icon pack {pack_id} contributes multiple themes; disambiguate with a theme ID. Available: {}.",
					available(entries)
				)));
			}
		},
	};

	Ok(ResolvedTheme {
		file: root.join(entry.path.trim_start_matches("./")),
		version: ext_package.version().clone(),
	})
}

/// A comma-separated list of the selectable theme IDs, for error messages.
fn available(entries: &[IconThemeDef]) -> String {
	entries
		.iter()
		.filter_map(|t| t.id.as_deref())
		.collect::<Vec<_>>()
		.join(", ")
}

#[cfg(test)]
mod tests {
	use super::resolve_in;
	use std::fs::{create_dir_all, write};
	use std::path::{Path, PathBuf};

	/// Prepare an empty, unique packs root under the temp directory.
	fn temp_root(name: &str) -> PathBuf {
		let root = std::env::temp_dir().join(name);
		std::fs::remove_dir_all(&root).ok();
		root
	}

	/// Write a fake installed pack (its `package.json`) into `root`.
	fn install(root: &Path, pack_id: &str, package_json: &str) {
		let dir = root.join(pack_id);
		create_dir_all(&dir).unwrap();
		write(dir.join("package.json"), package_json).unwrap();
	}

	#[test]
	fn test_resolve_single_theme_without_id() {
		let root = temp_root("pls-theme-single");
		install(
			&root,
			"test-pub.single",
			r#"{ "publisher": "test-pub", "name": "single", "version": "1.2.3", "contributes": { "iconThemes": [
				{ "id": "only", "label": "Only", "path": "./dist/theme.json" }
			] } }"#,
		);
		let resolved = resolve_in(&root, "test-pub.single", None).unwrap();
		assert!(resolved.file.ends_with("test-pub.single/dist/theme.json"));
		assert_eq!(resolved.version, "1.2.3");
		std::fs::remove_dir_all(&root).ok();
	}

	#[test]
	fn test_resolve_selects_theme_by_id() {
		let root = temp_root("pls-theme-multi");
		install(
			&root,
			"test-pub.multi",
			r#"{ "publisher": "test-pub", "name": "multi", "version": "1.2.3", "contributes": { "iconThemes": [
				{ "id": "dark", "label": "Dark", "path": "./dark.json" },
				{ "id": "light", "label": "Light", "path": "./light.json" }
			] } }"#,
		);
		assert!(
			resolve_in(&root, "test-pub.multi", Some("light"))
				.unwrap()
				.file
				.ends_with("light.json")
		);
		// Ambiguous without a theme ID.
		assert!(resolve_in(&root, "test-pub.multi", None).is_err());
		// Unknown theme ID.
		assert!(resolve_in(&root, "test-pub.multi", Some("nope")).is_err());
		std::fs::remove_dir_all(&root).ok();
	}

	#[test]
	fn test_resolve_missing_pack_is_error() {
		let root = temp_root("pls-theme-missing");
		create_dir_all(&root).unwrap();
		assert!(resolve_in(&root, "test-pub.absent", None).is_err());
		std::fs::remove_dir_all(&root).ok();
	}
}
