use crate::commands::IconPackSubcommand;
use clap::Subcommand;

/// Represents a `pls` subcommand.
///
/// This enum must list all `pls` subcommands except the base mode, which is
/// actually represented by `None` where this enum is used inside `Option`.
#[derive(Subcommand, Debug)]
pub enum PlsSubcommand {
	/// Manage icon packs.
	IconPack {
		#[command(subcommand)]
		action: IconPackSubcommand,
	},
}

// ===============
// Implementations
// ===============

impl PlsSubcommand {
	/// Execute the appropriate subcommand.
	///
	/// If the subcommand raises any exception, it will be printed to the console.
	pub fn handle(&self) {
		let res = match self {
			Self::IconPack { action } => action.handle(),
		};
		if let Err(exc) = res {
			println!("{exc}");
		};
	}
}
