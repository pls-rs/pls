use crate::config::Conf;
use serde::{Deserialize, Serialize};

/// This enum contains different groups of permissions defined on nodes, in a
/// UNIX-like operating system, as they would appear in the octal notation. Each
/// variant of this enum corresponds to one digit of the mode in octal notation.
///
/// Note that while the values of `Special` cause changes to user, group and
/// other permissions, they are all stored in fourth digit of the octal number.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Oct {
	Special,
	User,
	Group,
	Other,
}

/// This enum contains different types of permissions defined on nodes, in a
/// UNIX-like operating system, as they would appear in the symbolic notation.
///
/// Note that in a symbolic triplet, `Execute` and `Special` are both expressed
/// combined in the third character.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sym {
	None, // no permissions
	Read,
	Write,
	Execute,
	/// setuid, setgid or sticky bit
	Special,
}

impl Sym {
	/// Get the symbolic character associated with a permission.
	///
	/// This does not support [`Sym::Special`] because that character can vary
	/// based on other factors, use [`Sym::special_ch`].
	///
	/// This function returns a marked-up string.
	pub fn ch(&self, conf: &Conf) -> String {
		let ch = match self {
			Sym::None => '-',
			Sym::Read => 'r',
			Sym::Write => 'w',
			Sym::Execute => 'x',
			// Special maps to 4 characters: 's', 't', 'S' or 'T'.
			Sym::Special => panic!("Use `Perm::special_ch` instead."),
		};
		let directives = conf.constants.perm_styles.get(self).unwrap();
		format!("<{directives}>{ch}</>")
	}

	/// Get the symbolic character associated with a special permission.
	///
	/// This function is the equivalent of [`Sym::ch`] that specifically
	/// handles [`Sym::Special`].
	///
	/// This function returns a marked-up string.
	pub fn special_ch(&self, oct: Oct, execute: bool, conf: &Conf) -> String {
		if self != &Sym::Special {
			panic!("Use `Perm::ch` instead.")
		}

		let ch = match (oct, execute) {
			(Oct::Other, false) => 'T',
			(Oct::Other, true) => 't',
			(_, false) => 'S',
			(_, true) => 's',
		};
		let directives = conf.constants.perm_styles.get(self).unwrap();
		format!("<{directives}>{ch}</>")
	}
}
