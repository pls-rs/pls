use crate::exc::Exc;
use crate::pack::source::PackRef;
use serde::Deserialize;

/// The resolved download coordinates for a pack version.
pub struct Resolved {
	pub version: String,
	pub download_url: String,
}

#[derive(Deserialize)]
struct Metadata {
	version: String,
	files: MetadataFiles,
}

#[derive(Deserialize)]
struct MetadataFiles {
	download: Option<String>,
}

/// Parse an Open VSX metadata response into [`Resolved`].
fn parse_metadata(body: &str) -> Result<Resolved, Exc> {
	let meta: Metadata =
		json5::from_str(body).map_err(|e| Exc::Other(format!("Bad Open VSX response: {e}")))?;
	let download_url = meta
		.files
		.download
		.ok_or_else(|| Exc::Other(String::from("Open VSX response has no download URL.")))?;
	Ok(Resolved {
		version: meta.version,
		download_url,
	})
}

/// Resolve a pack's download URL and version from Open VSX.
///
/// Queries the latest version, or the pinned version when `pack.version` is set.
pub fn resolve(pack: &PackRef) -> Result<Resolved, Exc> {
	let mut url = format!("https://open-vsx.org/api/{}/{}", pack.publisher, pack.name);
	if let Some(version) = &pack.version {
		url.push('/');
		url.push_str(version);
	}
	let mut res = ureq::get(&url).call().map_err(|e| match e {
		ureq::Error::StatusCode(404) => Exc::Other(format!(
			"{}.{} was not found on Open VSX — it may only be on the VS Code Marketplace.",
			pack.publisher, pack.name
		)),
		other => Exc::Http(Box::new(other)),
	})?;
	let body = res
		.body_mut()
		.read_to_string()
		.map_err(|e| Exc::Http(Box::new(e)))?;
	parse_metadata(&body)
}

/// Download the bytes of a `.vsix` from the given URL.
pub fn download(url: &str) -> Result<Vec<u8>, Exc> {
	let mut res = ureq::get(url).call().map_err(|e| Exc::Http(Box::new(e)))?;
	res.body_mut()
		.with_config()
		.limit(64 * 1024 * 1024) // .vsix packs can exceed ureq's default 10 MB cap.
		.read_to_vec()
		.map_err(|e| Exc::Http(Box::new(e)))
}

#[cfg(test)]
mod tests {
	use super::parse_metadata;

	#[test]
	fn test_parse_metadata() {
		let json = r#"{
			"version": "1.26.0",
			"files": { "download": "https://open-vsx.org/x/pack.vsix", "icon": "x.png" }
		}"#;
		let r = parse_metadata(json).unwrap();
		assert_eq!(r.version, "1.26.0");
		assert_eq!(r.download_url, "https://open-vsx.org/x/pack.vsix");
	}

	#[test]
	fn test_parse_metadata_missing_download() {
		assert!(parse_metadata(r#"{ "version": "1.0.0", "files": {} }"#).is_err());
	}
}
