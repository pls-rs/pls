use crate::config::{Args, EntryConst};
use crate::enums::{DetailField, Typ};
use crate::ext::Ctime;
use crate::models::{Node, OwnerMan, Perm};
use log::warn;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::time::SystemTime;
use time::{format_description, OffsetDateTime, UtcOffset};

pub trait Detail {
	fn size_val(&self) -> Option<u64>;
	fn blocks_val(&self) -> Option<u64>;
	fn time_val(&self, field: DetailField) -> Option<SystemTime>;
	fn user_val(&self, owner_man: &mut OwnerMan) -> Option<String>;
	fn group_val(&self, owner_man: &mut OwnerMan) -> Option<String>;

	fn dev(&self, entry_const: &EntryConst) -> Option<String>;
	fn ino(&self, entry_const: &EntryConst) -> Option<String>;
	fn nlink(&self, entry_const: &EntryConst) -> Option<String>;
	fn perm(&self, entry_const: &EntryConst) -> Option<String>;
	fn oct(&self, entry_const: &EntryConst) -> Option<String>;
	fn user(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String>;
	fn uid(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String>;
	fn group(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String>;
	fn gid(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String>;
	fn size(&self, entry_const: &EntryConst, args: &Args) -> Option<String>;
	fn blocks(&self, entry_const: &EntryConst) -> Option<String>;
	fn time(&self, field: DetailField, entry_const: &EntryConst) -> Option<String>;
}

impl Detail for Node<'_> {
	// ===========
	// Sort fields
	// ===========

	/// Compute the size of the node, returning `None` for directories.
	fn size_val(&self) -> Option<u64> {
		self.meta_ok()
			.filter(|_| self.typ != Typ::Dir)
			.map(|meta| meta.len())
	}

	/// Compute the block count for the node, returning `None` for directories.
	fn blocks_val(&self) -> Option<u64> {
		self.meta_ok()
			.filter(|_| self.typ != Typ::Dir)
			.map(|meta| meta.blocks())
	}

	/// Get the value of the system time field specified by `field`.
	fn time_val(&self, field: DetailField) -> Option<SystemTime> {
		self.meta_ok().and_then(|meta| {
			match field {
				DetailField::Atime => meta.accessed(),
				DetailField::Btime => meta.created(),
				DetailField::Ctime => meta.c_time(),
				DetailField::Mtime => meta.modified(),
				_ => unreachable!("src/traits/det.rs / impl Detail for Node / time_val"),
			}
			.ok()
		})
	}

	/// Get the name of the user that owns this node, if known.
	fn user_val(&self, owner_man: &mut OwnerMan) -> Option<String> {
		self.meta_ok()
			.and_then(|meta| owner_man.user(meta.uid()).name)
	}

	/// Get the name of the group that owns this node, if known.
	fn group_val(&self, owner_man: &mut OwnerMan) -> Option<String> {
		self.meta_ok()
			.and_then(|meta| owner_man.group(meta.gid()).name)
	}

	// ===========
	// Renderables
	// ===========

	/// Get the device number, not the human-readable device name, of the node.
	///
	/// This function returns a marked-up string.
	fn dev(&self, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok().map(|meta| {
			let dev = meta.dev().to_string();
			let directives = &entry_const.dev_style;
			format!("<{directives}>{dev}</>")
		})
	}

	/// Get the inode number of the node.
	///
	/// This function returns a marked-up string.
	fn ino(&self, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok().map(|meta| {
			let ino = meta.ino().to_string();
			let directives = &entry_const.ino_style;
			format!("<{directives}>{ino}</>")
		})
	}

	/// Get the number of hard links pointing to the node.
	///
	/// Usually files have 1 hard link and directories have more than 1. So this
	/// function highlights files with more than 1 and directories with 1 hard
	/// link.
	///
	/// This function returns a marked-up string.
	fn nlink(&self, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok()
			.map(|meta| entry_const.nlink_styles.format(meta.nlink(), &self.typ))
	}

	/// Get the symbolic representation of the permissions of the node.
	///
	/// This function returns a marked-up string.
	fn perm(&self, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok()
			.map(|meta| Perm::from(meta.mode()).sym(entry_const))
	}

	/// Get the octal representation of the permissions of a node.
	///
	/// This function returns a marked-up string.
	fn oct(&self, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok()
			.map(|meta| Perm::from(meta.mode()).oct(entry_const))
	}

	/// Get the name of the user that owns this node. The name is highlighted if
	/// the owner is the current user.
	///
	/// This function returns a marked-up string.
	fn user(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok()
			.map(|meta| owner_man.user(meta.uid()).name(entry_const))
	}

	/// Get the UID of the user that owns this node. The UID is highlighted if
	/// the owner is the current user.
	///
	/// This function returns a marked-up string.
	fn uid(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok()
			.map(|meta| owner_man.user(meta.uid()).id(entry_const))
	}

	/// Get the name of the group that owns this node. The name is highlighted
	/// if the current user belongs to this group.
	///
	/// This function returns a marked-up string.
	fn group(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok()
			.map(|meta| owner_man.group(meta.gid()).name(entry_const))
	}

	/// Get the GID of the group that owns this node. The GID is highlighted
	/// if the current user belongs to this group.
	///
	/// This function returns a marked-up string.
	fn gid(&self, owner_man: &mut OwnerMan, entry_const: &EntryConst) -> Option<String> {
		self.meta_ok()
			.map(|meta| owner_man.group(meta.gid()).id(entry_const))
	}

	/// Get the size of the file in bytes, optionally with higher units in
	/// powers of 2^10 or 10^3.
	///
	/// This function returns a marked-up string.
	fn size(&self, entry_const: &EntryConst, args: &Args) -> Option<String> {
		self.size_val()
			.map(|size| args.unit.size(size, entry_const))
	}

	/// Get the number of blocks occupied by the file.
	///
	/// This function returns a marked-up string.
	fn blocks(&self, entry_const: &EntryConst) -> Option<String> {
		self.blocks_val().map(|blocks| {
			let directives = &entry_const.blocks_style;
			format!("<{directives}>{blocks}</>")
		})
	}

	/// Get the chosen timestamp field.
	///
	/// This function returns a marked-up string.
	fn time(&self, field: DetailField, entry_const: &EntryConst) -> Option<String> {
		self.time_val(field).map(|time| {
			let mut dt: OffsetDateTime = time.into();
			match UtcOffset::current_local_offset() {
				Ok(offset) => dt = dt.to_offset(offset),
				Err(_) => {
					warn!("Could not determine UTC offset")
				}
			}
			let format = format_description::parse_borrowed::<2>(
				entry_const.timestamp_formats.get(&field).unwrap(),
			)
			.unwrap();
			dt.format(&format).unwrap()
		})
	}
}
