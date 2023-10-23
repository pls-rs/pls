//! This module provides a trait [`Ctime`], that can be used to extend
//! `Metadata` with a method `c_time` that provides the `st_ctime` of a node
//! with an API that matches the other timestamp fields.

use std::fs::Metadata;
use std::io::Result as IoResult;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// =====
// Trait
// =====

/// This trait provides a method `ctime` that provides the `st_ctime` of a node.
/// What this field represents depends on the operating system.
///
/// > On some systems (like Unix) is the time of the last metadata change, and,
/// > on others (like Windows), is the creation time.
/// >
/// > — [Python documentation](https://docs.python.org/3/library/stat.html#stat.ST_CTIME)
pub trait Ctime {
	/// Compute the `st_ctime` of the node.
	///
	/// This function matches the signature of other timestamp fields:
	///
	/// * [`accessed`](Metadata::accessed)
	/// * [`created`](Metadata::created)
	/// * [`modified`](Metadata::modified)
	fn c_time(&self) -> IoResult<SystemTime>;
}

// ===============
// Implementations
// ===============

impl Ctime for Metadata {
	fn c_time(&self) -> IoResult<SystemTime> {
		let sec = self.ctime();
		let nanosec = self.ctime_nsec();
		let ctime = UNIX_EPOCH + Duration::new(sec as u64, nanosec as u32);
		Ok(ctime)
	}
}
