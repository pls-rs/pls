use crate::fmt::render;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Exc {
	/// wraps all occurrences of errors in I/O operations
	Io(std::io::Error),
	/// wraps all occurrences of errors in SVG operations
	Svg(resvg::usvg::Error),
	Conf(figment::Error),
	/// wraps exceptions from the `xterm-query` crate
	Xterm(xterm_query::XQError),
	/// wraps all other errors
	Other(String),
}

impl Display for Exc {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let attn = "<bold red>error:</>";
		let err = match self {
			Exc::Io(err) => err.to_string(),
			Exc::Conf(err) => err.to_string(),
			Exc::Svg(err) => err.to_string(),
			Exc::Other(text) => text.to_string(),
			Exc::Xterm(err) => err.to_string(),
		};
		let msg = format!("{attn} {err}");
		write!(f, "{}", render(msg))
	}
}
