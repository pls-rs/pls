use crate::exc::Exc;
use crate::fmt::render;
use crate::pack::packs_dir;
use crate::vsc::ExtRef;
use std::fs::remove_dir_all;

/// Remove the specified icon pack, if it exists.
///
/// # Arguments
///
/// * `source` - the identification for the icon pack extension to remove
pub fn remove(source: &str) -> Result<(), Exc> {
	let pack = source.parse::<ExtRef>()?;
	let id = pack.to_string();
	let dest = packs_dir()?.join(&id);

	if dest.exists() {
		print!("{}", render(format!("󰧧 Removing <bold>{id}</>...")));
	} else {
		println!("{}", render(format!("󱂨 <bold>{id}</> does not exist.")));
		return Ok(());
	}

	remove_dir_all(&dest).map_err(Exc::Io)?;
	println!("{}", render("<green>done!</>"));

	Ok(())
}
