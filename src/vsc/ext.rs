use regex::Regex;
use serde::Deserialize;
use std::{str::FromStr, sync::LazyLock};

use crate::exc::Exc;

/// The Open VSX Registry URL may contain a trailing slash.
static VSX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"/extension/(?P<pub>[\w-]+)/(?P<name>[\w-]+)/?$").unwrap());

/// The same regex matches both plain extension ID and VS Code Marketplace URL.
static ID_OR_VSC: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"(\?itemName=)?(?P<pub>[\w-]+)\.(?P<name>[\w-]+)$").unwrap());

/// Represents a reference to a VS Code extension resolved from user input.
#[derive(Debug, PartialEq, Eq)]
pub struct ExtRef {
	pub publisher: String,
	pub name: String,
}

impl FromStr for ExtRef {
	type Err = Exc;

	/// Parse a source string into a [`ExtRef`].
	///
	/// This accepts three forms of input:
	///
	/// * an Open VSX URL which contains the publisher and name as path params
	///   i.e. `/extension/<publisher>/<name>`
	/// * an extension ID
	///   i.e. `<publisher>.<name>`
	/// * a VS Code Marketplace URL which contains the extension ID as a query param
	///   i.e. `?itemName=<publisher>.<name>`
	fn from_str(source: &str) -> Result<Self, Self::Err> {
		let source = source.trim();

		// Open VSX URL: /extension/<publisher>/<name>
		if let Some(caps) = VSX.captures(source) {
			return Ok(ExtRef {
				publisher: caps["pub"].to_string(),
				name: caps["name"].to_string(),
			});
		}
		if let Some(caps) = ID_OR_VSC.captures(source) {
			return Ok(ExtRef {
				publisher: caps["pub"].to_string(),
				name: caps["name"].to_string(),
			});
		}

		Err(Exc::Other("Invalid extension ID or URL.".to_string()))
	}
}

impl ExtRef {
	pub fn download(&self) -> Result<Vec<u8>, Exc> {
		let url = self.fetch_download_url()?;
		let mut res = ureq::get(&url).call().map_err(|e| Exc::Http(Box::new(e)))?;
		res.body_mut()
			.with_config()
			.limit(64 * 1024 * 1024) // `.vsix` packages can exceed ureq's default 10 MB cap.
			.read_to_vec()
			.map_err(|e| Exc::Http(Box::new(e)))
	}

	// =======
	// Private
	// =======

	fn fetch_download_url(&self) -> Result<String, Exc> {
		let url = format!("https://open-vsx.org/api/{}/{}", self.publisher, self.name);
		let mut res = ureq::get(&url).call().map_err(|e| Exc::Http(Box::new(e)))?;

		let body = res
			.body_mut()
			.read_to_string()
			.map_err(|e| Exc::Http(Box::new(e)))?;
		let meta: Metadata = json5::from_str(&body)
			.map_err(|e| Exc::Other(format!("Bad Open VSX response: {e}")))?;
		meta.files
			.download
			.ok_or_else(|| Exc::Other("Open VSX response has no download URL.".to_string()))
	}
}

// =======
// Private
// =======

#[derive(Deserialize)]
struct Metadata {
	files: MetadataFiles,
}

#[derive(Deserialize)]
struct MetadataFiles {
	download: Option<String>,
}

#[cfg(test)]
mod tests {
	use super::ExtRef;
	use crate::exc::Exc;
	use regex::Regex;

	macro_rules! make_test {
		( $($test_name:ident: $input:expr => $publisher:expr, $name:expr,)* ) => {
			$(
				#[test]
				fn $test_name() {
					let ext_ref = $input.parse::<ExtRef>().unwrap();
					assert_eq!(ext_ref.publisher, $publisher);
					assert_eq!(ext_ref.name, $name);
				}
			)*
		};
	}

	make_test!(
		test_plain_id: "catppuccin.catppuccin-vsc-icons" => "catppuccin", "catppuccin-vsc-icons",
		test_marketplace_url: "https://marketplace.visualstudio.com/items?itemName=catppuccin.catppuccin-vsc-icons" => "catppuccin", "catppuccin-vsc-icons",
		test_open_vsx_url: "https://open-vsx.org/extension/catppuccin/catppuccin-vsc-icons" => "catppuccin", "catppuccin-vsc-icons",
	);

	#[test]
	fn test_malformed_is_error() {
		let ext_ref: Result<ExtRef, Exc> = "https://open-vsx.org/extension/catppuccin".parse();
		assert!(ext_ref.is_err());
	}

	#[test]
	fn test_gets_download_url_from_metadata() {
		let ext_ref: ExtRef = "catppuccin.catppuccin-vsc-icons".parse().unwrap();
		println!("{:?}", ext_ref.fetch_download_url());
		let valid_url = Regex::new(
			r"^(?x)
				https://open-vsx.org/api/
				Catppuccin/catppuccin-vsc-icons/
				\d+\.\d+\.\d+/
				file/
				Catppuccin.catppuccin-vsc-icons-\d+\.\d+\.\d+\.vsix
			$",
		)
		.unwrap();
		assert_eq!(
			valid_url.is_match(&ext_ref.fetch_download_url().unwrap()),
			true
		);
	}
}
