/// This enum contains all the different ways a node can appear.
///
/// A node can be a combination of appearances as well. For example, a node may be a tree parent as
/// well as a tree child. When a node has no special appearances, it is a normal listing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Appearance {
	/// The node appears as the target of a symlink.
	///
	/// The display text of the node is set to the symlink destination. It is
	/// not based on the name of the node.
	Symlink,
	/// The node appears as the child of another.
	///
	/// The tree-drawing shapes are shown before the name of the node, which is
	/// the same as [`Appearance::Normal`].
	TreeChild,
	/// The node appears as the parent of another.
	///
	/// This provides the ability to use an alternative "open-folder" icon for
	/// directories.
	TreeParent,
	/// The node appears as an individual file being listed.
	///
	/// The name of the node is shown exactly as it was passed to the CLI. It
	/// could be the name, or a relative/absolute path.
	SoloFile,
}
