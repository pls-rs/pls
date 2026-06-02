use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Collapse {
	/// Name-based collapsing matches this node with another having the exact
	/// given name.
	Name(String),
	/// Extension-based collapsing matches this node with another having the
	/// same base name and the given extension.
	Ext(String),
	/// Substitution-based collapsing builds the parent's name by expanding the
	/// given replacement template against the capture groups of the spec's own
	/// pattern. For example, a pattern of `^(?<base>.+)\.min\.js$` with a
	/// substitution of `$base.js` collapses `app.min.js` into `app.js`.
	Sub(String),
}
