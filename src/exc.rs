use crate::fmt::render;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Exc {
	/// wraps all occurrences of errors in I/O operations
	Io(std::io::Error),
	Conf(figment::Error),
}

impl Display for Exc {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let attn = "<bold red>error:</>";
		let err = match self {
			Exc::Io(err) => err.to_string(),
			Exc::Conf(err) => err.to_string(),
		};
		let msg = format!("{attn} {err}");
		write!(f, "{}", render(msg))
	}
}
