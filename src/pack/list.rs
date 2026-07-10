use crate::exc::{fmt_warning, Exc};
use crate::ext::SubDirs;
use crate::pack::packs_dir;
use crate::vsc::ExtPackage;
use crate::vsc::ExtRef;

/// List the icon themes exposed by installed icon packs.
///
/// If a `source` is provided, this function lists only the themes inside the
/// matching pack. Otherwise, it lists every pack found in the data directory.
///
/// # Arguments
///
/// * `source` - the identification for the icon pack extension to list
pub fn list(source: Option<&str>) -> Result<(), Exc> {
	let root = packs_dir()?;

	let packs = match source {
		Some(source) => {
			let pack = source.parse::<ExtRef>()?;
			let id = pack.to_string();
			let dir = root.join(&id);
			if !dir.is_dir() {
				return Err(Exc::Other(format!("{id} is not installed.")));
			}
			vec![dir]
		}
		None => {
			let dirs = root.sub_dirs();
			if dirs.is_empty() {
				println!("{}", fmt_warning("No icon packs installed."));
				return Ok(());
			}
			dirs
		}
	};

	for (i, pack) in packs.iter().enumerate() {
		if i > 0 {
			println!(); // Separate consecutive packs with a blank line.
		}
		let ext_package = ExtPackage::try_from(pack.as_path())?;
		println!("{}", ext_package);
	}
	Ok(())
}
