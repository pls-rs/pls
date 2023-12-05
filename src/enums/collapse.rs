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
}
