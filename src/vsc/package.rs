use crate::exc::Exc;
use crate::fmt::render;
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

/// Tree connectors, matching the defaults `pls` uses for its node listings.
const TEE: &str = "├─ ";
const BEND: &str = "└─ ";

// ======
// Models
// ======

/// Represents a VS Code extension package located on the file system.
pub struct ExtPackage {
	package_json: PackageJson,
}

#[derive(Deserialize)]
pub struct IconThemeDef {
	pub id: Option<String>,
	pub label: String,
	pub path: String,
}

// ===============
// Implementations
// ===============

impl TryFrom<&Path> for ExtPackage {
	type Error = Exc;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		let manifest_path = path.join("package.json");
		let manifest_text = std::fs::read_to_string(&manifest_path).map_err(Exc::Io)?;
		let package_json = json5::from_str(&manifest_text).map_err(|e| Exc::Json(Box::new(e)))?;
		Ok(Self { package_json })
	}
}

impl ExtPackage {
	/// Get the icon themes defined inside this extension.
	///
	/// This function returns an empty slice if the extension does not define
	/// any icon themes.
	pub fn icon_themes(&self) -> &[IconThemeDef] {
		self.package_json
			.contributes
			.as_ref()
			.map(|c| c.icon_themes.as_slice())
			.unwrap_or(&[])
	}

	/// Get the version of this extension.
	///
	/// This version is used as a part of the icon caching key so that updates
	/// to the extension automatically bust the cache.
	pub fn version(&self) -> &String {
		&self.package_json.version
	}
}

impl Display for ExtPackage {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(
			f,
			"{}",
			render(format!(
				"<bold>{}.{}</>",
				self.package_json.publisher, self.package_json.name
			))
		)?;
		let entries = self.icon_themes();
		if entries.is_empty() {
			write!(
				f,
				"\n{}",
				render(format!("<dimmed>{BEND}</><red>no icon themes</>"))
			)?;
		}
		for (i, entry) in entries.iter().enumerate() {
			let connector = if i + 1 == entries.len() { BEND } else { TEE };
			write!(f, "\n{}", render(format!("<dimmed>{connector}</>{entry}")))?;
		}
		Ok(())
	}
}

impl Display for IconThemeDef {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(
			f,
			"{}",
			render(format!(
				"<cyan>{:24}</> {}",
				self.id.as_deref().unwrap_or("—"),
				self.label
			))
		)
	}
}

// =======
// Private
// =======

#[derive(Deserialize)]
struct PackageJson {
	publisher: String,
	name: String,
	version: String,
	contributes: Option<Contributions>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contributions {
	#[serde(default)]
	icon_themes: Vec<IconThemeDef>,
}
