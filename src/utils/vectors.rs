//! This module contains some helper functions for vector collections.
//!
//! The public interface of the module consists of one function:
//!
//! * [`dedup`]

use indexmap::IndexSet;
use std::hash::Hash;

/// Deduplicate a vector, by preserving the last appearance of a value.
///
/// The values are moved into an [`IndexSet`], which both deduplicates and
/// remembers insertion order, so it doubles as the output buffer and the
/// "seen" set. Inserting in reverse means the first insertion of each value is
/// its last appearance in the input; reversing again restores the original
/// order. This needs no `Clone` bound and never copies a value.
///
/// # Arguments
///
/// * `vec` - the vector to deduplicate
pub fn dedup<T: Hash + Eq>(vec: Vec<T>) -> Vec<T> {
	let mut set: IndexSet<T> = IndexSet::with_capacity(vec.len());
	for item in vec.into_iter().rev() {
		set.insert(item);
	}
	set.into_iter().rev().collect()
}
