/// This enum contains all the different appearances a node can have. The
/// appearance controls the `display_name` of the node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Appearance {
	Normal,  // the node appears as a primary listing
	Symlink, // the node appears as the target of a symlink
}
