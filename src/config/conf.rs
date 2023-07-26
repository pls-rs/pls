use crate::enums::Collapse;
use crate::exc::Exc;
use crate::models::{Constants, Spec};
use figment::providers::{Data, Format, Serialized, Yaml};
use figment::Figment;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Create a [`HashMap`] from a list of key-value pairs.
macro_rules! map_str_str {
	( $($k:expr => $v:expr,)* ) => {
		core::convert::From::from([
            $( (String::from($k), String::from($v)), )*
        ])
	};
}

/// Manages the configuration system of the application. This manager provides
/// `Conf` instances tailored to each path, while caching the base configuration
/// for performance.
pub struct ConfMan {
	/// the base configuration, i.e. the serialized output of [`Conf::default`]
	pub base: Figment,
}

impl Default for ConfMan {
	fn default() -> Self {
		info!("Preparing base configuration.");

		let mut base = Figment::from(Serialized::defaults(Conf::default()));
		if let Some(home_yaml) = home::home_dir().and_then(Self::conf_at) {
			base = base.admerge(home_yaml);
		}

		info!("Base configuration prepared.");
		Self { base }
	}
}

impl ConfMan {
	/// Look for a config file in the given directory and return its contents.
	///
	/// This function will return `None` if no config file is found inside the
	/// given directory.
	fn conf_at<P>(dir: P) -> Option<Data<Yaml>>
	where
		P: AsRef<Path>,
	{
		let conf_file = dir.as_ref().join(".pls.yml");
		conf_file.exists().then(|| {
			debug!("Found config file {conf_file:?}.");
			Yaml::file(conf_file)
		})
	}

	/// Collects all the relevant `.pls.yml` config files into a vector.
	///
	/// This includes config files from the following locations:
	///
	/// * the home directory, if determined
	/// * TODO: the Git root, if Git-tracked
	/// * the given path, if a directory, or it's parent
	///
	/// # Arguments
	///
	/// * `path` - the path to scan for config files
	fn yaml_contents(path: &Path) -> Vec<Data<Yaml>> {
		[
			// TODO: Add Git root here.
			if path.is_dir() {
				Some(path.to_path_buf())
			} else {
				path.parent().map(Path::to_path_buf)
			},
		]
		.iter()
		.flatten()
		.filter_map(Self::conf_at)
		.collect()
	}

	/// Get a `Conf` instance for the given path.
	///
	/// This merges the path-specific config files with the base and returns the
	/// resulting [`Conf`] instance, If there is an error parsing the config
	/// files, an [`Exc`] instance will be returned.
	pub fn get(&self, path: Option<&Path>) -> Result<Conf, Exc> {
		let mut fig = self.base.clone();

		if let Some(path) = path {
			for file in Self::yaml_contents(path) {
				fig = fig.admerge(file);
			}
		}

		fig.extract().map_err(Exc::ConfError)
	}
}

/// Represents the complete configuration of `pls`.
///
/// `pls` comes with a lean configuration out-of-the-box and users are
/// encouraged to add their own configuration using YAML files in the home
/// directory, project Git root and/or working directory.
///
/// Note that `pls` also accepts CLI arguments, which are not represented here.
/// Refer to [`Args`](crate::config::Args) for those.
#[derive(Serialize, Deserialize)]
pub struct Conf {
	/// mapping of icon names to actual glyphs from Nerd Fonts
	pub icons: HashMap<String, String>,
	/// list of node specs, in ascending order of specificity
	pub specs: Vec<Spec>,
	/// constants that determine the appearance and styling of the output
	pub constants: Constants,
}

impl Default for Conf {
	fn default() -> Self {
		Self {
			icons: map_str_str!(
				// pls
				"pls"          => "", // nf-oct-primitive_dot
				"missing"      => "", // nf-cod-error
				// Node types
				"file"         => "", // nf-fa-file
				"dir"          => "", // nf-fa-folder
				"symlink"      => "󰌹", // nf-md-link-variant
				"fifo"         => "󰟥", // nf-md-pipe
				"socket"       => "󰟨", // nf-md-power_socket_uk
				"char_device"  => "", // nf-fa-paragraph
				"block_device" => "󰋊", // nf-md-harddisk
				// Generic
				"audio"        => "󰓃", // nf-md-speaker
				"book"         => "", // nf-fa-book
				"broom"        => "󰃢", // nf-md-broom
				"config"       => "", // nf-seti-config
				"container"    => "", // nf-oct-container
				"env"          => "", // nf-fae-plant
				"image"        => "󰋩", // nf-md-image
				"json"         => "", // nf-seti-json
				"law"          => "", // nf-oct-law
				"lock"         => "", // nf-oct-lock
				"package"      => "", // nf-oct-package
				"runner"       => "󰜎", // nf-md-run
				"shell"        => "", // nf-oct-terminal
				"source"       => "", // nf-oct-file_code
				"test"         => "󰙨", // nf-md-test_tube
				"text"         => "", // nf-seti-text
				"video"        => "󰕧", // nf-md-video
				// Brands
				"apple"        => "", // nf-fa-apple
				"git"          => "󰊢", // nf-md-git
				"github"       => "", // nf-oct-mark_github
				"markdown"     => "", // nf-oct-markdown
				"rust"         => "", // nf-dev-rust
			),
			specs: vec![
				// Extensions
				Spec::new(r"\.sh$", "shell"),
				Spec::new(r"\.rs$", "rust").style("rgb(247,76,0)"),
				Spec::new(r"\.(txt|rtf)$", "text"),
				Spec::new(r"\.mdx?$", "markdown"),
				Spec::new(r"\.ini$", "config"),
				Spec::new(r"\.(json|toml|yml|yaml)$", "json"),
				Spec::new(r"\.(jpg|jpeg|png|svg|webp|gif|ico)$", "image"),
				Spec::new(r"\.(mov|mp4|mkv|webm|avi|flv)$", "video"),
				Spec::new(r"\.(mp3|flac|ogg|wav)$", "audio"),
				// Partial names
				Spec::new(r"^\.env\b", "env"),
				Spec::new(r"^README\b", "book").importance(2),
				Spec::new(r"^LICENSE\b", "law"),
				Spec::new(r"docker-compose.*\.yml$", "container"),
				Spec::new(r"Dockerfile", "container"),
				// Exact names
				Spec::new(r"^\.DS_Store$", "apple").importance(-2),
				Spec::new(r"^\.pls\.yml$", "pls").importance(0),
				Spec::new(r"^\.git$", "git").importance(-2),
				Spec::new(r"^\.gitignore$", "git"),
				Spec::new(r"^\.github$", "github"),
				Spec::new(r"^src$", "source").importance(1),
				Spec::new(r"^(justfile|Makefile)$", "runner"),
				Spec::new(r"^Cargo\.toml$", "package"),
				Spec::new(r"^Cargo\.lock$", "lock")
					.importance(-1)
					.collapse(Collapse::Name(String::from("Cargo.toml"))),
				Spec::new(r"^rustfmt.toml$", "broom"),
			],
			constants: Constants::default(),
		}
	}
}
