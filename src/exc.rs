use crate::fmt::render;
use std::fmt::{Display, Formatter, Result};

pub enum Exc {
	/// wraps all occurrences of errors in I/O operations
	IoError(std::io::Error),
	ConfError(figment::Error),
}

impl Display for Exc {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let attn = "<bold red>error:</>";
		let err = match self {
			Exc::IoError(err) => err.to_string(),
			Exc::ConfError(err) => err.to_string(),
		};
		let msg = format!("{attn} {err}");
		write!(f, "{}", render(msg))
	}
}
