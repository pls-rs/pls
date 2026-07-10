use crate::exc::Exc;
use crate::utils::dirs::data_dir;
use std::path::PathBuf;

/// The directory under which icon packs are installed.
pub fn packs_dir() -> Result<PathBuf, Exc> {
	data_dir().map(|d| d.join("icon-packs")).ok_or_else(|| {
		Exc::Other(String::from(
			"Could not determine a data directory for icon packs.",
		))
	})
}
