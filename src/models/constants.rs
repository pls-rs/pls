use crate::enums::{Oct, Sym};
use std::collections::HashMap;

pub struct Constants {
	/// mapping of symbolic permission bits to style
	pub sym_styles: HashMap<Sym, String>,
	/// mapping of octal permission bits to style
	pub oct_styles: HashMap<Oct, String>,
	/// style for magnitude and unit of node size
	pub size_styles: SizeStyles,
}

impl Default for Constants {
	fn default() -> Self {
		Self {
			sym_styles: [
				(Sym::None, "dimmed"),
				(Sym::Read, "yellow"),
				(Sym::Write, "red"),
				(Sym::Execute, "green"),
				(Sym::Special, "magenta"),
			]
			.into_iter()
			.map(|(k, v)| (k, v.to_string()))
			.collect(),
			oct_styles: [
				(Oct::Special, "magenta"),
				(Oct::User, "blue"),
				(Oct::Group, "blue dimmed"),
				(Oct::Other, "dimmed"),
			]
			.into_iter()
			.map(|(k, v)| (k, v.to_string()))
			.collect(),
			size_styles: SizeStyles {
				mag: String::from("bold"),
				prefix: String::default(),
				base: String::from("dimmed"),
			},
		}
	}
}

pub struct SizeStyles {
	/// style for the node size magnitude
	pub mag: String,
	/// style for the node size unit prefix
	pub prefix: String,
	/// style for the node size base unit
	pub base: String,
}
