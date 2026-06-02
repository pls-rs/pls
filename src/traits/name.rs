use crate::models::Node;
use std::path::PathBuf;

pub trait Name {
	fn ext(&self) -> &str;
	fn stem(&self) -> String;
	fn cname(&self) -> &str;

	fn aligned_name(&self) -> String;
}

impl Name for Node<'_> {
	// ===========
	// Sort fields
	// ===========

	/// Get the extension for a node.
	///
	/// Returns a blank string if the node does not have an extension. The value
	/// is computed once and cached, as it is used for sorting.
	fn ext(&self) -> &str {
		self.ext_cache.get_or_init(|| {
			self.path
				.extension()
				.unwrap_or_default()
				.to_string_lossy()
				.to_string()
		})
	}

	/// Get the name for the node, without the extension, if any.
	///
	/// Returns the full name if the node does not have an extension.
	fn stem(&self) -> String {
		self.path
			.file_stem()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string()
	}

	/// Get the canonical name for the node.
	///
	/// The canonical name is the name of the node, stripped of leading symbols
	/// and normalised to lowercase. The value is computed once and cached, as it
	/// is used for sorting.
	fn cname(&self) -> &str {
		self.cname_cache.get_or_init(|| {
			self.name
				.to_lowercase()
				.trim_start_matches(|c: char| !c.is_alphanumeric())
				.to_string()
		})
	}

	// ===============
	// Name components
	// ===============

	/// Get the name of the node when aligning for leading dots.
	///
	/// If the node name starts with a dot, the dot is dimmed. If not, the name
	/// is left-padded with a space to line up the alphabetic characters.
	fn aligned_name(&self) -> String {
		let path = PathBuf::from(self.disp_name());
		if let Some(name) = path.file_name() {
			let name = name.to_string_lossy();

			// 'clear' ensures that the dot and padding spaces are not formatted.
			let aligned_name = if name.starts_with('.') {
				format!("<clear dimmed>.</>{}", name.strip_prefix('.').unwrap())
			} else {
				format!("<clear> </>{}", name)
			};

			if let Some(parent) = path.parent() {
				return parent.join(aligned_name).to_string_lossy().to_string();
			}
		}
		self.disp_name().to_string()
	}
}
