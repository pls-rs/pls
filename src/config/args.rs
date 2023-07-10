use crate::enums::{DetailField, SortField, Typ, UnitSys};
use crate::fmt::render;
use clap::Parser;
use regex::bytes::{Regex, RegexBuilder};
use regex::Error as RegexError;
#[cfg(test)]
use std::ffi::OsString;
use std::path::PathBuf;

/// Parse the given string into a [`Regex`] while turning off Unicode mode.
///
/// The default implementations of `Regex` from string-types are all
/// Unicode-aware, so the builder pattern from [`RegexBuilder`] must be used.
///
/// Since user's can input regexes like 'ab.d', which can match invalid UTF-8
/// without Unicode mode, we must use `Regex` and `RegexBuilder` from
/// [`regex::bytes`].
fn regex_parser(s: &str) -> Result<Regex, RegexError> {
	RegexBuilder::new(s).unicode(false).build()
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = render("<red bold>`pls`</> is a prettier and powerful `ls` for the pros."),
	long_about = render([
		"<red bold>`pls`</> is a prettier and powerful `ls` for the pros.\n",
		"<bold>Read docs:</> https://dhruvkb.github.io/pls",
		"<bold>Get source:</> https://github.com/dhruvkb/pls",
		"<bold>Sponsor:</> https://github.com/sponsors/dhruvkb"
	].join("\n")),
)]
pub struct Args {
	/// the paths to list, each of which may be a file or directory
	#[clap(default_value = ".")]
	pub paths: Vec<PathBuf>,

	/// the data points to show about each node
	#[clap(
		help_heading = "Detailed view",
		short,
		long = "det",
		default_value = "none",
		value_enum
	)]
	pub details: Vec<DetailField>,

	/// show headers above columnar data
	#[clap(help_heading = "Detailed view", short = 'H', long, action = clap::ArgAction::Set)]
	pub header: Option<bool>,

	/// the type of units to use for the node sizes
	#[clap(
		help_heading = "Detailed view",
		short,
		long,
		default_value = "binary",
		value_enum
	)]
	pub unit: UnitSys,

	/// display node names in multiple columns
	#[clap(help_heading = "Grid view", short, long, default_value = "false", action = clap::ArgAction::Set)]
	pub grid: bool,

	/// display node names column-first
	#[clap(help_heading = "Grid view", short = 'D', long, default_value = "false", action = clap::ArgAction::Set)]
	pub down: bool,

	/// display icons next to node names
	#[clap(help_heading = "Presentation", short, long, default_value = "true", action = clap::ArgAction::Set)]
	pub icon: bool,

	/// display node type suffixes after the node name
	#[clap(help_heading = "Presentation", short = 'S', long, default_value = "true", action = clap::ArgAction::Set)]
	pub suffix: bool,

	/// show symlink targets
	#[clap(help_heading = "Presentation", short = 'l', long, default_value = "true", action = clap::ArgAction::Set)]
	pub sym: bool,

	/// align items accounting for leading dots
	#[clap(help_heading = "Presentation", short, long, default_value = "true", action = clap::ArgAction::Set)]
	pub align: bool,

	/// the set of fields to sort by, trailing `_` reverses the direction
	#[clap(help_heading = "Sorting", short, long = "sort", default_values = ["cat", "cname"], value_enum)]
	pub sort_bases: Vec<SortField>,

	/// the set of node types to include in the output
	#[clap(
		help_heading = "Filtering",
		short = 't',
		long = "types",
		default_value = "all",
		value_enum,
        value_names = ["TYPES"],
	)]
	pub typs: Vec<Typ>,

	/// the importance cutoff to dim or hide unimportant files
	#[clap(help_heading = "Filtering", short = 'I', long, default_value = "0")]
	pub imp: i8,

	/// the pattern of files to selectively hide from the output
	#[clap(help_heading = "Filtering", short, long, value_parser = regex_parser)]
	pub exclude: Option<Regex>,

	/// the pattern of files to exclusively show in the output
	#[clap(help_heading = "Filtering", short, long, value_parser = regex_parser)]
	pub only: Option<Regex>,
}

impl Default for Args {
	/// Create a new instance of `Args` parsing real command-line arguments.
	fn default() -> Self {
		let args = Args::parse();
		args.post_process()
	}
}

impl Args {
	/// Create a new instance of `Args` parsing the given arguments.
	#[cfg(test)]
	pub fn raw<I, T>(itr: I) -> Self
	where
		I: IntoIterator<Item = T>,
		T: Into<OsString> + Clone,
	{
		Args::parse_from(itr)
	}

	fn post_process(mut self) -> Self {
		let warnings = self.clean();
		for warning in &warnings {
			println!("{} {}", render("<bold yellow>WARN:</>"), warning);
		}
		if !warnings.is_empty() {
			println!();
		}
		self
	}

	/// Clean the parsed arguments to resolve conflicting arguments.
	fn clean(&mut self) -> Vec<&str> {
		let mut warnings = vec![];

		self.details = DetailField::clean(&self.details);
		self.sort_bases = SortField::clean(&self.sort_bases);
		self.typs = Typ::clean(&self.typs);

		if self.grid && self.is_detailed() {
			// Multi-column mode is disabled when detailed mode is enabled.
			warnings.push("Detailed view disabled grid view.");
			self.grid = false;
		}

		if self.grid && self.show_header() {
			// Headers cannot be shown outside of detailed view.
			warnings.push("Grid view disabled column headers.");
			self.header = Some(false);
		}

		if self.header.is_none() && self.is_detailed() {
			// Headers are shown by default in detailed mode.
			self.header = Some(true);
		}

		warnings
	}

	/* Getters */
	/* ======= */

	/// Get whether to render the output in detailed view using a table.
	pub fn is_detailed(&self) -> bool {
		self.details.len() >= 2
	}

	/// Get whether to show the header above the table columns.
	pub fn show_header(&self) -> bool {
		self.header.unwrap_or_default()
	}
}

#[cfg(test)]
mod tests {
	use super::Args;

	macro_rules! make_warning_test {
        ($($name:ident: $argv:expr => $msg:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let mut args = Args::raw($argv);
                    let warnings = args.clean();
                    assert!(warnings.contains(&$msg));
                }
            )*
        }
    }

	make_warning_test!(
		test_details_multi_col: ["pls", "--det", "ino", "--grid", "true"] => "Detailed view disabled grid view.",
		test_multi_col_and_header: ["pls", "--grid", "true", "--header", "true"] => "Grid view disabled column headers.",
	);

	macro_rules! make_clean_test {
        ($($name:ident: $argv:expr => $key:ident, $val:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let mut args = Args::raw($argv);
                    args.clean();
                    assert_eq!(args.$key, $val);
                }
            )*
        }
    }

	make_clean_test!(
		test_details_beats_multi_col: ["pls", "--det", "ino", "--grid", "true"] => grid, false,
		test_multi_col_beats_header: ["pls", "--grid", "true", "--header", "true"] => header, Some(false),
		test_details_aids_header: ["pls", "--det", "ino"] => header, Some(true),
	);
}
