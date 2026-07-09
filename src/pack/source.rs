use crate::exc::Exc;

/// Represents a reference to a VS Code icon pack resolved from user input.
///
/// A pack is distributed as a VS Code extension, so the accepted forms are the
/// extension's ID and marketplace URLs (see [`parse`]).
#[derive(Debug, PartialEq, Eq)]
pub struct PackRef {
	pub publisher: String,
	pub name: String,
	pub version: Option<String>,
}

/// Parse a source string into a [`PackRef`].
///
/// This accepts three forms of input:
///
/// * a pack ID of the form `<publisher>.<name>` (which can include a version
///   with `@<version>` suffix)
/// * a VS Code Marketplace URL which contains the pack ID as a query param
///   `?itemName=<publisher>.<name>`
/// * an Open VSX URL which contains the pack's publisher and name as path
///   fragments `/extension/<publisher>/<name>`
pub fn parse(source: &str) -> Result<PackRef, Exc> {
	let source = source.trim();

	// Open VSX URL: /extension/<publisher>/<name>
	if let Some(rest) = source.split("/extension/").nth(1) {
		let mut segs = rest.split('/').filter(|s| !s.is_empty());
		if let (Some(publisher), Some(name)) = (segs.next(), segs.next()) {
			return Ok(PackRef {
				publisher: publisher.to_string(),
				name: name.to_string(),
				version: None,
			});
		}
	}

	// Marketplace URL: ?itemName=<publisher>.<name>
	let id = if let Some(idx) = source.find("itemName=") {
		&source[idx + "itemName=".len()..]
	} else if source.contains("://") {
		return Err(Exc::Other(format!(
			"Unrecognised icon pack URL: {source:?}."
		)));
	} else {
		source
	};

	// `publisher.name[@version]`
	let (id, version) = match id.split_once('@') {
		Some((id, ver)) => (id, Some(ver.to_string())),
		None => (id, None),
	};
	let (publisher, name) = id
		.split_once('.')
		.filter(|(p, n)| !p.is_empty() && !n.is_empty())
		.ok_or_else(|| {
			Exc::Other(format!(
				"Invalid pack ID: {id:?} (expected `publisher.name`)."
			))
		})?;

	Ok(PackRef {
		publisher: publisher.to_string(),
		name: name.to_string(),
		version,
	})
}

#[cfg(test)]
mod tests {
	use super::parse;

	#[test]
	fn test_plain_id() {
		let r = parse("catppuccin.catppuccin-vsc-icons").unwrap();
		assert_eq!(r.publisher, "catppuccin");
		assert_eq!(r.name, "catppuccin-vsc-icons");
		assert_eq!(r.version, None);
	}

	#[test]
	fn test_id_with_version() {
		let r = parse("catppuccin.catppuccin-vsc-icons@1.2.3").unwrap();
		assert_eq!(r.publisher, "catppuccin");
		assert_eq!(r.name, "catppuccin-vsc-icons");
		assert_eq!(r.version.as_deref(), Some("1.2.3"));
	}

	#[test]
	fn test_marketplace_url() {
		let r = parse(
			"https://marketplace.visualstudio.com/items?itemName=catppuccin.catppuccin-vsc-icons",
		)
		.unwrap();
		assert_eq!(r.publisher, "catppuccin");
		assert_eq!(r.name, "catppuccin-vsc-icons");
		assert_eq!(r.version, None);
	}

	#[test]
	fn test_open_vsx_url() {
		let r = parse("https://open-vsx.org/extension/catppuccin/catppuccin-vsc-icons").unwrap();
		assert_eq!(r.publisher, "catppuccin");
		assert_eq!(r.name, "catppuccin-vsc-icons");
		assert_eq!(r.version, None);
	}

	#[test]
	fn test_malformed_is_error() {
		assert!(parse("no-dot-here").is_err());
		assert!(parse("").is_err());
	}
}
