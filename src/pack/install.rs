use crate::exc::Exc;
use crate::fmt::render;
use crate::pack::{list, openvsx, source, vsix};
use std::fs::remove_dir_all;

/// Download and install the icon pack identified by `source`, printing the
/// installed location and the icon themes it provides.
pub fn add(source: &str) -> Result<(), Exc> {
	let pack = source::parse(source)?;
	let resolved = openvsx::resolve(&pack)?;

	let id = format!("{}.{}", pack.publisher, pack.name);
	let dest = list::packs_dir()?.join(&id);

	print!(
		"{}",
		render(format!(
			" Downloading <bold>{id}@{}</>...",
			resolved.version
		))
	);
	let bytes = openvsx::download(&resolved.download_url)?;

	if dest.exists() {
		remove_dir_all(&dest).map_err(Exc::Io)?;
	}
	vsix::extract(&bytes, &dest)?;

	println!("{}", render("<green>done!</>"));

	// Show the installed pack exactly as `icon-pack list <id>` would.
	list::list(Some(id.as_str()))
}
