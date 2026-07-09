use crate::exc::{fmt_warning, Exc};
use crate::fmt::render;
use crate::pack::source;
use crate::pack::vsix::{self, ThemeEntry};
use crate::utils::dirs::data_dir;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

/// Tree connectors, matching the defaults `pls` uses for its node listings.
const TEE: &str = "├─ ";
const BEND: &str = "└─ ";

/// List the icon themes exposed by installed icon packs.
///
/// With `source`, lists only the matching pack; otherwise lists every pack
/// found in the data directory.
pub fn list(source: Option<&str>) -> Result<(), Exc> {
	let root = packs_dir()?;

	let packs = match source {
		Some(source) => {
			let pack = source::parse(source)?;
			let id = format!("{}.{}", pack.publisher, pack.name);
			let dir = root.join(&id);
			if !dir.is_dir() {
				println!("{}", fmt_warning(&format!("{id} is not installed.")),);
				return Ok(());
			}
			vec![dir]
		}
		None => {
			let dirs: Vec<_> = installed(&root)
				.into_iter()
				.filter(|p| p.is_dir())
				.collect();
			if dirs.is_empty() {
				println!("{}", fmt_warning("No icon packs installed."));
				return Ok(());
			}
			dirs
		}
	};

	for (i, pack) in packs.iter().enumerate() {
		if i > 0 {
			println!(); // Separate consecutive packs with a blank line.
		}
		report(pack);
	}
	Ok(())
}

/// The directory under which icon packs are installed.
pub fn packs_dir() -> Result<PathBuf, Exc> {
	data_dir().map(|d| d.join("icon-packs")).ok_or_else(|| {
		Exc::Other(String::from(
			"Could not determine a data directory for icon packs.",
		))
	})
}

// =======
// Private
// =======

/// Print a pack's ID and the tree of icon themes it provides.
///
/// Each theme's `id` is what disambiguates it in the `icon_pack` config (via
/// `default.name` or a `per_scheme` entry) when a pack provides more than one.
fn report(dest: &Path) {
	let name = dest.file_name().unwrap().to_string_lossy();
	println!("{}", render(format!("<bold>{name}</>")));

	let entries = themes_of(dest);
	if entries.is_empty() {
		println!("{}", fmt_warning("This pack contributes no icon themes."));
		return;
	}
	for (i, entry) in entries.iter().enumerate() {
		let connector = if i + 1 == entries.len() { BEND } else { TEE };
		let id = entry.id.as_deref().unwrap_or("—");
		println!(
			"{}",
			render(format!(
				"<dimmed>{connector}</><cyan>{id:<24}</> <dimmed>{}</>",
				entry.label
			))
		);
	}
}

/// The icon themes a pack provides, empty when it declares none.
fn themes_of(dest: &Path) -> Vec<ThemeEntry> {
	let package_json =
		read_to_string(dest.join("package.json")).expect("installed icon pack has a package.json");
	vsix::theme_entries(&package_json)
}

/// The installed pack directories under `root`, or an empty vector when `root`
/// does not exist or cannot be read.
fn installed(root: &Path) -> Vec<PathBuf> {
	read_dir(root)
		.into_iter()
		.flatten()
		.flatten()
		.map(|entry| entry.path())
		.collect()
}
