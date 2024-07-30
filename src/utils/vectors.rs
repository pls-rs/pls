//! This module contains some helper functions for vector collections.
//!
//! The public interface of the module consists of one function:
//!
//! * [`dedup`]

use std::collections::{HashSet, VecDeque};

/// Deduplicate a vector, by preserving the last appearance of a value.
///
/// # Arguments
///
/// * vec - the vector to deduplicate
pub fn dedup<T: std::hash::Hash + Eq + Clone>(vec: Vec<T>) -> Vec<T> {
	let mut dedup = VecDeque::new();

	let mut set = HashSet::new();
	for item in vec.into_iter().rev() {
		if set.insert(item.clone()) {
			dedup.push_front(item);
		}
	}

	dedup.into()
}
