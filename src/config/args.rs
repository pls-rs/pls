use crate::enums::{DetailField, SortField, Typ, UnitSys};
use crate::fmt::render;
use clap::Parser;
use log::warn;
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

/// Represents the command-line arguments to `pls`.
///
/// `pls` picks sane defaults for the CLI arguments. If you prefer different
/// defaults, users can alias `pls` with their preferred flags set.
///
/// Note that `pls` allows for deep customisation using `.pls.yml` files, which
/// is not represented here. Refer to [`Conf`](crate::config::Conf) for those.
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
		help_heading = "Detail view",
		short,
		long = "det",
		default_value = "none",
		value_enum
	)]
	pub details: Vec<DetailField>,

	/// show headers above columnar data
	#[clap(help_heading = "Detail view", short = 'H', long, default_value = "true", action = clap::ArgAction::Set)]
	pub header: bool,

	/// the type of units to use for the node sizes
	#[clap(
		help_heading = "Detail view",
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

	/// the set of node types to include in the output
	#[clap(
		help_heading = "Filtering",
		short = 't',
		long = "typ",
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

	/// the set of fields to sort by, trailing `_` reverses the direction
	#[clap(help_heading = "Sorting", short, long = "sort", default_values = ["cat", "cname"], value_enum)]
	pub sort_bases: Vec<SortField>,
}

impl Default for Args {
	/// Create a new instance of `Args` parsing real command-line arguments.
	fn default() -> Self {
		let mut args = Args::parse();
		args.post_process();
		args
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

	/// Clean the parsed arguments and log any warnings that are raised.
	///
	/// The output of this function is similar to the format used by
	/// [`Exc`](crate::exc::Exc) as they serve similar purposes.
	fn post_process(&mut self) {
		self.clean().iter().for_each(|warning| {
			warn!("{warning}");
		});
	}

	/// Clean the parsed arguments to resolve conflicting arguments.
	///
	/// `pls` is intentionally lax about conflicting arguments, and will attempt
	/// to resolve conflicts in a way that is least surprising to the user. So,
	/// while the clean function generates warnings, they are not surfaced to
	/// the user and are primarily used for debugging.
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

		// Headers cannot be shown outside of detailed view.
		if self.grid && self.header {
			warnings.push("Grid view disabled column headers.");
			self.header = false;
		}
		if !self.is_detailed() && self.header {
			warnings.push("Lack of metadata disabled column headers.");
			self.header = false;
		}

		if self.grid && self.sym {
			// Symlink targets cannot be shown in grid view.
			warnings.push("Grid view disabled symlink targets.");
			self.sym = false;
		}

		warnings
	}

	/* Getters */
	/* ======= */

	/// Get whether to render the output in detailed view using a table.
	fn is_detailed(&self) -> bool {
		self.details.len() >= 2
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
		test_multi_col_and_sym: ["pls", "--grid", "true", "--sym", "true"] => "Grid view disabled symlink targets.",
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

		// Symlink target is only shown in detailed view.
		test_default_sym: ["pls"] => sym, true,
		test_default_sym_when_detailed: ["pls", "--det", "ino"] => sym, true,
		test_default_sym_when_multi_col: ["pls", "--grid", "true"] => sym, false,
		test_multi_col_beats_sym: ["pls", "--grid", "true", "--sym", "true"] => sym, false,

		// Header is only shown when detailed view is enabled and there is at least one detail field.
		test_default_header: ["pls"] => header, false,
		test_default_header_when_detailed: ["pls", "--det", "ino"] => header, true,
		test_default_header_when_multi_col: ["pls", "--grid", "true"] => header, false,
		test_multi_col_beats_header: ["pls", "--grid", "true", "--header", "true"] => header, false,
	);
}
