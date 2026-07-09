use std::hash::{Hash, Hasher};

/// Compute the cache key for an icon identity and render size.
///
/// The `ident` is a stable, structured identity for the icon (for pack icons,
/// a combination of pack, version, theme, color scheme and key; for a literal
/// SVG path, the path itself), so the key is independent of where the SVG
/// happens to live on disk and changes when the pack is updated.
///
/// # Arguments
///
/// * `ident` - the icon's stable identity
/// * `size` - the size at which the icon is rendered
pub fn compute_hash(ident: &str, size: u8) -> u32 {
	let mut hasher = NumericHasher::default();
	ident.hash(&mut hasher);
	size.hash(&mut hasher);
	hasher.state
}

/// A 32-bit [FNV-1a](https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function)
/// hasher.
struct NumericHasher {
	state: u32,
}

impl Default for NumericHasher {
	fn default() -> Self {
		Self {
			state: 0x811c_9dc5, // FNV-1a 32-bit offset basis
		}
	}
}

impl Hasher for NumericHasher {
	fn finish(&self) -> u64 {
		self.state as u64
	}

	fn write(&mut self, bytes: &[u8]) {
		for &byte in bytes {
			self.state ^= byte as u32;
			self.state = self.state.wrapping_mul(0x0100_0193); // FNV-1a 32-bit prime
		}
	}
}
