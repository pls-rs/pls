use crate::exc::Exc;
use crate::utils::dirs::cache_dir;
use log::debug;
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};
use std::fs::read_to_string;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

/// Get the RGBA data for a given SVG file at a given size.
///
/// The result is read from and written to a cache under the XDG cache directory
/// (see [`cache_dir`]), so an icon is only rasterised once across runs. Caching
/// is skipped silently when no cache directory can be located.
///
/// # Arguments
///
/// * `id` - the icon's cache key (see [`compute_hash`](super::compute_hash))
/// * `path` - the path to the SVG file
/// * `size` - the size at which to render the icon
pub fn get_rgba(id: u32, path: &Path, size: u8) -> Option<Vec<u8>> {
	let cache_file = cache_file(id);

	if let Some(cache_file) = &cache_file
		&& let Some(rgba_data) = load_from_cache(cache_file)
	{
		return Some(rgba_data);
	}

	let rgba_data = compute_rgba(path, size)
		.map_err(|e| debug!("Could not render icon: {e}"))
		.ok()?;

	if let Some(cache_file) = &cache_file
		&& let Err(e) = save_to_cache(cache_file, &rgba_data)
	{
		debug!("Could not cache icon {id}: {e}");
	}

	Some(rgba_data)
}

/// The path of the cache file for the given cache key, if a cache directory can
/// be located.
fn cache_file(id: u32) -> Option<PathBuf> {
	Some(cache_dir()?.join("icons").join(id.to_string()))
}

/// Compute the RGBA data for a given SVG file at a given size.
fn compute_rgba(path: &Path, size: u8) -> Result<Vec<u8>, Exc> {
	// Read SVG file
	let svg_data = read_to_string(path).map_err(Exc::Io)?;

	// Create a default options struct with the target dimensions
	let opt = Options::default();
	let rtree = Tree::from_str(&svg_data, &opt).map_err(|e| Exc::Svg(Box::new(e)))?;

	// Create a pixmap with the desired dimensions
	let mut pixmap =
		Pixmap::new(size.into(), size.into()).ok_or(Exc::Other(String::from("Pixmap was None")))?;

	// Render the SVG tree into the pixmap
	resvg::render(
		&rtree,
		Transform::from_scale(
			size as f32 / rtree.size().width(),
			size as f32 / rtree.size().height(),
		),
		&mut pixmap.as_mut(),
	);

	// Get the RGBA data
	let rgba_data = pixmap.data().to_vec();
	Ok(rgba_data)
}

/// Load the RGBA data from the cache, or `None` if it is absent or unreadable.
fn load_from_cache(cache_file: &Path) -> Option<Vec<u8>> {
	std::fs::read(cache_file).ok()
}

/// Save the RGBA data to the cache, creating the necessary directories.
fn save_to_cache(cache_file: &Path, rgba_data: &[u8]) -> IoResult<()> {
	if let Some(parent) = cache_file.parent() {
		std::fs::create_dir_all(parent)?;
	}
	std::fs::write(cache_file, rgba_data)
}
