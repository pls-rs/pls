use clap::Subcommand;

/// Represents a `pls` subcommand.
#[derive(Subcommand, Debug)]
pub enum Command {
	/// Manage SVG icon packs.
	IconPack {
		#[command(subcommand)]
		action: IconPackAction,
	},
}

/// This enum covers actions under `pls icon-pack` subcommand.
#[derive(Subcommand, Debug)]
pub enum IconPackAction {
	/// Download and install an icon pack from Open VSX.
	Add {
		/// the identifier for the icon pack
		source: String,
	},
	/// List the exposed themes for every, or a specific, icon pack.
	List {
		/// the identifier for the icon pack
		source: Option<String>,
	},
}
