use crate::exc::Exc;
use crate::fmt::render;
use crate::pack::list;
use crate::pack::packs_dir;
use crate::vsc::ExtRef;
use std::fs::remove_dir_all;
use std::io::{Write, stdout};

/// Download the specified icon pack and print the icon themes it provides.
///
/// # Arguments
///
/// * `source` - the identification for the icon pack extension to install
pub fn add(source: &str) -> Result<(), Exc> {
	let pack = source.parse::<ExtRef>()?;
	let id = pack.to_string();
	let dest = packs_dir()?.join(&id);

	if dest.exists() {
		print!("{}", render(format!("󰚰 Updating <bold>{id}</>...")));
		remove_dir_all(&dest).map_err(Exc::Io)?;
	} else {
		print!("{}", render(format!("󰇚 Downloading <bold>{id}</>...")));
	}
	stdout().flush().ok(); // Show the prefix before the network call blocks.

	pack.download(&dest)?;
	println!("{}", render("<green>done!</>"));

	// Delegate the report of what was downloaded to the list subcommand.
	list::list(Some(&id))
}
