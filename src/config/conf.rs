use crate::config::app_const::AppConst;
use crate::config::entry_const::EntryConst;
use crate::enums::Collapse;
use crate::models::Spec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Create a [`HashMap`] from a list of key-value pairs.
macro_rules! map_str_str {
	( $($k:expr => $v:expr,)* ) => {
		core::convert::From::from([
            $( (String::from($k), String::from($v)), )*
        ])
	};
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
	/// mapping of icon names to actual glyphs from Nerd Fonts or paths to SVGs
	pub icons: HashMap<String, String>,
	/// list of node specs, in ascending order of specificity
	pub specs: Vec<Spec>,
	/// constants that determine the appearance and styling of each entry
	pub entry_const: EntryConst,
	/// constants that determine the appearance and styling of the entire UI
	pub app_const: AppConst,
}

impl Default for Conf {
	fn default() -> Self {
		Self {
			icons: map_str_str!(
				// pls
				"pls"          => "", // nf-oct-primitive_dot
				"missing"      => "", // nf-cod-error
				// Node types
				"file"         => "",
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
				"rust"         => "", // nf-seti-rust
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
			entry_const: EntryConst::default(),
			app_const: AppConst::default(),
		}
	}
}
