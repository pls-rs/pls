use std::hash::{Hash, Hasher};
use std::path::Path;

/// Compute the hash for a given path and size pair. This hash acts as the key
/// for the icon cache.
pub fn compute_hash(path: &Path, size: u8) -> u32 {
	let mut hasher = NumericHasher::default();
	path.hash(&mut hasher);
	size.hash(&mut hasher);
	// Perform a lossy conversion to u32, throwing away the upper bits.
	hasher.finish() as u32
}

#[derive(Default)]
struct NumericHasher {
	state: u32,
}

impl Hasher for NumericHasher {
	fn finish(&self) -> u64 {
		(self.state as u64) + 1
	}

	fn write(&mut self, bytes: &[u8]) {
		for &byte in bytes {
			// Example hash function: FNV-1a variant
			self.state = self.state.wrapping_mul(16777619) ^ (byte as u32);
		}
	}
}
