use crate::enums::Collapse;
use regex::bytes::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};

/// Represents the specification for identifying and styling a node.
///
/// Specs are the ideological core of `pls` and the key differentiating factor
/// from other tools.
#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
	/// a regex pattern to match against the node's name
	#[serde(with = "serde_regex")]
	pub pattern: Regex,
	/// names of the icon to use for the node
	pub icons: Option<Vec<String>>,
	/// styles to apply to the node name and icon
	pub style: Option<String>,
	/// the importance level of the node
	pub importance: Option<i8>,
	/// the rule for determining the parent node, if any, for this node
	pub collapse: Option<Collapse>,
}

impl Spec {
	/// Create a basic `Spec` instance with only a pattern and an icon.
	///
	/// `Spec` follows a builder pattern, so you can chain the following method
	/// to define the remaining fields.
	///
	/// - [`importance`](Spec::importance)
	/// - [`style`](Spec::style)
	/// - [`collapse`](Spec::collapse)
	pub fn new(pattern: &str, icon: &str) -> Self {
		Self {
			pattern: RegexBuilder::new(pattern).unicode(false).build().unwrap(),
			icons: Some(vec![String::from(icon)]),
			style: None,
			importance: None,
			collapse: None,
		}
	}

	/// Consume the current `Spec` instance and return a new one with the
	/// specified importance level.
	pub fn importance(self, importance: i8) -> Self {
		Self {
			importance: Some(importance),
			..self
		}
	}

	/// Consume the current `Spec` instance and return a new one with the
	/// specified style directives.
	pub fn style(self, style: &str) -> Self {
		Self {
			style: Some(String::from(style)),
			..self
		}
	}

	/// Consume the current `Spec` instance and return a new one with the
	/// specified collapse definition.
	pub fn collapse(self, collapse: Collapse) -> Self {
		Self {
			collapse: Some(collapse),
			..self
		}
	}
}
