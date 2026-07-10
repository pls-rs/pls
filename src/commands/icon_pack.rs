use crate::exc::Exc;
use crate::pack::{add, list};
use clap::Subcommand;

/// This enum covers actions under `pls icon-pack` subcommand.
#[derive(Subcommand, Debug)]
pub enum IconPackSubcommand {
	/// Download and install an icon pack from Open VSX.
	Add {
		/// the ID (or URL) of the icon pack
		source: String,
	},
	/// List the themes provided by every, or a specific, icon pack.
	List {
		/// the ID (or URL) of the icon pack
		source: Option<String>,
	},
}

// ===============
// Implementations
// ===============

impl IconPackSubcommand {
	pub fn handle(&self) -> Result<(), Exc> {
		match self {
			Self::Add { source } => add(source),
			Self::List { source } => list(source.as_deref()),
		}
	}
}
