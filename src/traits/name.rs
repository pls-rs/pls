use crate::models::Node;

pub trait Name {
	fn ext(&self) -> String;
	fn cname(&self) -> String;

	fn aligned_name(&self) -> String;
}

impl Name for Node {
	/* Sort fields */
	/* =========== */

	/// Get the extension for a node.
	///
	/// Returns a blank string if the node does not have an extension.
	fn ext(&self) -> String {
		self.path
			.extension()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string()
	}

	/// Get the canonical name for the node.
	///
	/// The canonical name is the name of the node, stripped of leading symbols
	/// and normalised to lowercase.
	fn cname(&self) -> String {
		self.name
			.to_lowercase()
			.trim_start_matches(|c: char| !c.is_alphanumeric())
			.to_string()
	}

	/* Name components */
	/* =============== */

	/// Get the name of the node when aligning for leading dots.
	///
	/// If the node name starts with a dot, the dot is dimmed. If not, the name
	/// is left-padded with a space to line up the alphabetic characters.
	fn aligned_name(&self) -> String {
		// 'clear' ensures that the dot and padding spaces are not formatted.
		if self.name.starts_with('.') {
			format!("<clear dimmed>.</>{}", &self.name[1..])
		} else {
			format!("<clear> </>{}", self.name)
		}
	}
}
