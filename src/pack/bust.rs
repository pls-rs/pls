use crate::exc::Exc;
use crate::utils::dirs::cache_dir;
use std::fs::remove_dir_all;

/// Delete the icon cache directory.
pub fn bust() -> Result<(), Exc> {
	let cache = cache_dir();
	if cache.is_none() {
		print!("No cache to bust.");
		return Ok(());
	}

	let cache = cache.unwrap();
	let location = cache.join("icons");
	if location.exists() {
		remove_dir_all(&location).map_err(Exc::Io)?;
	}
	Ok(())
}
