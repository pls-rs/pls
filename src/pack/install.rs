use crate::exc::Exc;
use crate::fmt::render;
use crate::pack::{list, vsix};
use crate::vsc::ExtRef;
use std::fs::remove_dir_all;

/// Download and install the icon pack identified by `source`, printing the icon
/// themes it provides.
pub fn add(source: &str) -> Result<(), Exc> {
	let pack = source.parse::<ExtRef>()?;

	let id = format!("{}.{}", pack.publisher, pack.name);
	let dest = list::packs_dir()?.join(&id);

	print!("{}", render(format!(" Downloading <bold>{id}</>...")));
	let bytes = pack.download()?;

	if dest.exists() {
		remove_dir_all(&dest).map_err(Exc::Io)?;
	}
	vsix::extract(&bytes, &dest)?;

	println!("{}", render("<green>done!</>"));

	// Delegate the report of what was downloaded to the list subcommand.
	list::list(Some(id.as_str()))
}
