use crate::exc::Exc;
use crate::fmt::render;
use crate::pack::list;
use crate::vsc::ExtRef;
use std::fs::remove_dir_all;
use std::io::{stdout, Write};

/// Download and install the icon pack identified by `source`, printing the icon
/// themes it provides.
pub fn add(source: &str) -> Result<(), Exc> {
	let pack = source.parse::<ExtRef>()?;

	let id = format!("{}.{}", pack.publisher, pack.name);
	let dest = list::packs_dir()?.join(&id);

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
	list::list(Some(id.as_str()))
}
