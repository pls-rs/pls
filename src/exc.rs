use crate::fmt::render;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Exc {
	/// wraps all occurrences of errors in I/O operations
	Io(std::io::Error),
	/// wraps all occurrences of errors in SVG operations
	Svg(Box<resvg::usvg::Error>),
	/// wraps all occurrences of errors in configuration loading
	Conf(Box<figment2::Error>),
	/// wraps errors from the `ureq` HTTP client
	Http(Box<ureq::Error>),
	/// wraps exceptions from the `xterm-query` crate
	Xterm(Box<xterm_query::XQError>),
	/// wraps errors from the `zip` crate
	Zip(Box<zip::result::ZipError>),
	/// wraps errors from the `json5` crate
	Json(Box<json5::Error>),
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
			Exc::Http(err) => err.to_string(),
			Exc::Xterm(err) => err.to_string(),
			Exc::Zip(err) => err.to_string(),
			Exc::Json(err) => err.to_string(),
		};
		let msg = format!("{attn} {err}");
		write!(f, "{}", render(msg))
	}
}

/// Format a warning message for display to the user.
///
/// This follows the same appearance as the rendering for error messages, except
/// that the "warning:" prefix is bold and yellow.
pub fn fmt_warning(msg: &str) -> String {
	render(format!("<bold yellow>warning:</> {msg}"))
}
