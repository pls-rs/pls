//! This module contains code for working with paths.
//!
//! It deals with paths as abstract strings, without interacting with the
//! underlying file system to check if these paths have any real file at the
//! location they reference.
//!
//! The public interface of the module consists of one function:
//!
//! * [`common_ancestor`]

use path_clean::PathClean;
use std::path::{Path, PathBuf};

/// Get the common ancestor of the given paths.
///
/// This function normalises the given paths by resolving `..` to the parent
/// directory and dropping any `.` in the path.
///
/// Note that normalisation of '..' is incorrect for symlinks because the parent
/// of a symlink is not the path component before it.
///
/// # Arguments
///
/// * `paths` - the paths for which to find the common ancestor
pub fn common_ancestor(paths: &[&Path]) -> Option<PathBuf> {
	if paths.is_empty() {
		return None;
	}

	let mut paths = paths.iter().map(|path| path.clean());

	let mut common = paths.next().unwrap().to_path_buf();
	for path in paths {
		common = common_ancestor_two(&common, &path)?;
	}
	Some(common)
}

// =======
// Private
// =======

/// Get the common ancestor of two given paths.
///
/// This function does not handle relative paths and does not resolve symbols
/// like `.` and `..`.
///
/// # Arguments
///
/// * `one` - the first path
/// * `two` - the second path
fn common_ancestor_two(one: &Path, two: &Path) -> Option<PathBuf> {
	let mut one = one.to_path_buf();
	loop {
		if two.starts_with(&one) {
			return Some(one);
		}
		if !one.pop() || one.as_os_str().is_empty() {
			break;
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::common_ancestor;
	use std::path::{Path, PathBuf};

	macro_rules! make_common_ancestor_test {
        ( $($name:ident: $paths:expr => $parent:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let paths: Vec<&str> = $paths;
                    let path_bufs: Vec<_> = paths.iter().map(Path::new).collect();
                    let parent = common_ancestor(&path_bufs);

                    let expected = ($parent as Option<&str>).map(PathBuf::from);
                    assert_eq!(parent, expected);
                }
            )*
        };
    }

	make_common_ancestor_test!(
		test_zero_paths: vec![] => None,
		test_one_path: vec!["/a"] => Some("/a"),
		test_two_paths: vec!["/a/b", "/a/c"] => Some("/a"),
		test_three_paths: vec!["/a/b", "/a/c", "/a/d"] => Some("/a"),

		test_no_common_parent: vec!["a/b", "c/d"] => None,
		test_no_common_till_root: vec!["/a/b", "/c/d"] => Some("/"),

		test_variable_length: vec!["/a", "/a/b", "/a/b/c"] => Some("/a"),

		test_trailing_slash_unequal: vec!["/a/b", "/a/c/"] => Some("/a"),
		test_trailing_slash_equal: vec!["/a/b", "/a/b/"] => Some("/a/b"),

		test_relative: vec!["/a/b/c", "/a/b/../b/./c"] => Some("/a/b/c"),
		test_relative_end: vec!["/a/b/c", "/a/b/c/../."] =>  Some("/a/b"),
		test_relative_extra: vec!["/a/b", "/a/../../../a/b"] => Some("/a/b"),

		test_partial_match: vec!["/a/bat", "/a/ball"] => Some("/a"),
	);
}
