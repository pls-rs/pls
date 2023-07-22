use crate::config::{Args, Conf};
use crate::enums::{DetailField, Typ};
use crate::models::{Node, OwnerMan, Perm};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::{format_description, OffsetDateTime, UtcOffset};

pub trait Detail {
	fn ctime(&self) -> SystemTime;
	fn size_val(&self) -> Option<u64>;
	fn blocks_val(&self) -> Option<u64>;
	fn time_val(&self, field: DetailField) -> std::io::Result<SystemTime>;
	fn user_val(&self, owner_man: &mut OwnerMan) -> Option<String>;
	fn group_val(&self, owner_man: &mut OwnerMan) -> Option<String>;

	fn dev(&self, conf: &Conf) -> String;
	fn ino(&self, conf: &Conf) -> String;
	fn nlink(&self, conf: &Conf) -> String;
	fn perm(&self, conf: &Conf) -> String;
	fn oct(&self, conf: &Conf) -> String;
	fn user(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String;
	fn uid(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String;
	fn group(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String;
	fn gid(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String;
	fn size(&self, conf: &Conf, args: &Args) -> String;
	fn blocks(&self, conf: &Conf) -> String;
	fn time(&self, field: DetailField, conf: &Conf) -> String;
}

impl Detail for Node<'_> {
	/// Compute the ctime of the node.
	fn ctime(&self) -> SystemTime {
		let sec = self.meta.ctime();
		let nanosec = self.meta.ctime_nsec();
		UNIX_EPOCH + Duration::new(sec as u64, nanosec as u32)
	}

	/* Sort fields */
	/* =========== */

	/// Compute the size of the node, returning `None` for directories.
	fn size_val(&self) -> Option<u64> {
		if self.typ == Typ::Dir {
			None
		} else {
			Some(self.meta.len())
		}
	}

	/// Compute the block count for the node, returning `None` for directories.
	fn blocks_val(&self) -> Option<u64> {
		if self.typ == Typ::Dir {
			None
		} else {
			Some(self.meta.blocks())
		}
	}

	/// Get the value of the system time field specified by `field`.
	fn time_val(&self, field: DetailField) -> std::io::Result<SystemTime> {
		match field {
			DetailField::Atime => self.meta.accessed(),
			DetailField::Btime => self.meta.created(),
			DetailField::Ctime => Ok(self.ctime()),
			DetailField::Mtime => self.meta.modified(),
			_ => unreachable!("src/traits/det.rs / impl Detail for Node / time_val"),
		}
	}

	/// Get the name of the user that owns this node, if known.
	fn user_val(&self, owner_man: &mut OwnerMan) -> Option<String> {
		owner_man.user(self.meta.uid()).name
	}

	/// Get the name of the group that owns this node, if known.
	fn group_val(&self, owner_man: &mut OwnerMan) -> Option<String> {
		owner_man.group(self.meta.gid()).name
	}

	/* Renderables */
	/* =========== */

	/// Get the device number, not the human-readable device name, of the node.
	///
	/// This function returns a marked-up string.
	fn dev(&self, conf: &Conf) -> String {
		let dev = self.meta.dev().to_string();
		let directives = &conf.constants.dev_style;
		format!("<{directives}>{dev}</>")
	}

	/// Get the inode number of the node.
	///
	/// This function returns a marked-up string.
	fn ino(&self, conf: &Conf) -> String {
		let ino = self.meta.ino().to_string();
		let directives = &conf.constants.ino_style;
		format!("<{directives}>{ino}</>")
	}

	/// Get the number of hard links pointing to the node.
	///
	/// Usually files have 1 hard link and directories have more than 1. So this
	/// function highlights files with more than 1 and directories with 1 hard
	/// link.
	///
	/// This function returns a marked-up string.
	fn nlink(&self, conf: &Conf) -> String {
		let nlink = self.meta.nlink();
		let directives = match (self.typ, nlink) {
			(Typ::Dir, 1) => &conf.constants.nlink_styles.dir_sing,
			(Typ::Dir, _) => &conf.constants.nlink_styles.dir_plur,
			(_, 1) => &conf.constants.nlink_styles.file_sing,
			(_, _) => &conf.constants.nlink_styles.file_plur,
		};
		format!("<{directives}>{nlink}</>")
	}

	/// Get the symbolic representation of the permissions of the node.
	///
	/// This function returns a marked-up string.
	fn perm(&self, conf: &Conf) -> String {
		let perm: Perm = self.meta.mode().into();
		perm.sym(conf)
	}

	/// Get the octal representation of the permissions of a node.
	///
	/// This function returns a marked-up string.
	fn oct(&self, conf: &Conf) -> String {
		let perm: Perm = self.meta.mode().into();
		perm.oct(conf)
	}

	/// Get the name of the user that owns this node. The name is highlighted if
	/// the owner is the current user.
	///
	/// This function returns a marked-up string.
	fn user(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String {
		owner_man.user(self.meta.uid()).name(conf)
	}

	/// Get the UID of the user that owns this node. The UID is highlighted if
	/// the owner is the current user.
	///
	/// This function returns a marked-up string.
	fn uid(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String {
		owner_man.user(self.meta.uid()).id(conf)
	}

	/// Get the name of the group that owns this node. The name is highlighted
	/// if the current user belongs to this group.
	///
	/// This function returns a marked-up string.
	fn group(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String {
		owner_man.group(self.meta.gid()).name(conf)
	}

	/// Get the GID of the group that owns this node. The GID is highlighted
	/// if the current user belongs to this group.
	///
	/// This function returns a marked-up string.
	fn gid(&self, owner_man: &mut OwnerMan, conf: &Conf) -> String {
		owner_man.group(self.meta.gid()).id(conf)
	}

	/// Get the size of the file in bytes, optionally with higher units in
	/// powers of 2^10 or 10^3.
	///
	/// This function returns a marked-up string.
	fn size(&self, conf: &Conf, args: &Args) -> String {
		match self.size_val() {
			Some(size) => args.unit.size(size, conf),
			None => String::default(),
		}
	}

	/// Get the number of blocks occupied by the file.
	fn blocks(&self, _conf: &Conf) -> String {
		match self.blocks_val() {
			Some(blocks) => blocks.to_string(),
			None => String::default(),
		}
	}

	/// Get the chosen timestamp field.
	///
	/// This function returns a marked-up string.
	fn time(&self, field: DetailField, conf: &Conf) -> String {
		let time = match self.time_val(field) {
			Ok(mtime) => mtime,
			Err(_) => return String::default(),
		};
		let mut dt: OffsetDateTime = time.into();
		if let Ok(offset) = UtcOffset::current_local_offset() {
			dt = dt.to_offset(offset);
		}
		let format = format_description::parse_borrowed::<2>(
			conf.constants.timestamp_formats.get(&field).unwrap(),
		)
		.unwrap();
		dt.format(&format).unwrap()
	}
}
