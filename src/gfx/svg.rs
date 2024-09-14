use crate::exc::Exc;
use log::debug;
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};
use std::env;
use std::fs::{read_to_string, File};
use std::io::{Read, Result as IoResult, Write};
use std::path::Path;

/// Get the RGBA data for a given SVG file at a given size.
///
/// This function can retrieve the RGBA data from the cache, if present, and
/// also compute and cache it, if not present. Caching is only enabled if the
/// `PLS_CACHE` environment variable is set.
///
/// # Arguments
///
/// * `id` - the unique ID of the image
/// * `path` - the path to the SVG file
/// * `size` - the size at which to render the icon
pub fn get_rgba(id: u32, path: &Path, size: u8) -> Option<Vec<u8>> {
	let cache_file = env::var("PLS_CACHE")
		.ok()
		.map(|cache| Path::new(&cache).join("icons").join(id.to_string()));

	if let Some(cache_file) = &cache_file {
		if let Some(rgba_data) = load_from_cache(cache_file) {
			return Some(rgba_data);
		}
	}

	let rgba_data = match compute_rgba(path, size) {
		Ok(rgba_data) => Some(rgba_data),
		Err(exc) => {
			debug!("{}", exc);
			None
		}
	};

	if let Some(cache_file) = &cache_file {
		if let Some(rgba_data) = &rgba_data {
			save_to_cache(cache_file, rgba_data).expect("E");
		}
	}

	rgba_data
}

/// Compute the RGBA data for a given SVG file at a given size.
fn compute_rgba(path: &Path, size: u8) -> Result<Vec<u8>, Exc> {
	// Read SVG file
	let svg_data = read_to_string(path).map_err(Exc::Io)?;

	// Create a default options struct with the target dimensions
	let opt = Options::default();
	let rtree = Tree::from_str(&svg_data, &opt).map_err(Exc::Svg)?;

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

/// Load the RGBA data from the cache, if present.
fn load_from_cache(cache_file: &Path) -> Option<Vec<u8>> {
	if cache_file.exists() {
		let mut file = File::open(cache_file).expect("A");
		let mut buffer = Vec::new();
		file.read_to_end(&mut buffer).ok()?;
		Some(buffer)
	} else {
		None
	}
}

/// Save the RGBA data to the cache, creating the necessary directories.
fn save_to_cache(cache_file: &Path, rgba_data: &[u8]) -> IoResult<()> {
	std::fs::create_dir_all(cache_file.parent().unwrap())?;
	let mut file = File::create(cache_file)?;
	file.write_all(rgba_data)?;
	Ok(())
}
