use clap::Subcommand;

/// Represents a `pls` subcommand.
#[derive(Subcommand, Debug)]
pub enum Command {
	/// Manage icon packs.
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
		/// the ID (or marketplace URL) of the icon pack
		source: String,
	},
	/// List the themes provided by every, or a specific, icon pack.
	List {
		/// the ID of the icon pack
		source: Option<String>,
	},
}
