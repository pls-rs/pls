use crate::exc::Exc;
use crate::fmt::render;
use crate::pack::list;
use crate::vsc::ExtRef;
use std::fs::remove_dir_all;

pub fn remove(source: &str) -> Result<(), Exc> {
	let pack = source.parse::<ExtRef>()?;

	let id = format!("{}.{}", pack.publisher, pack.name);
	let dest = list::packs_dir()?.join(&id);

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
