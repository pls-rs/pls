use crate::enums::Collapse;
use crate::models::{Constants, Spec};
use std::collections::HashMap;

/// Create a [`HashMap`] from a list of key-value pairs.
macro_rules! map_str_str {
	( $($k:expr => $v:expr,)* ) => {
		core::convert::From::from([
            $( (String::from($k), String::from($v)), )*
        ])
	};
}

pub struct Conf {
	pub icons: HashMap<String, String>,
	pub specs: Vec<Spec>,
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
				"pipe"         => "󰟥", // nf-md-pipe
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
				// Exact names
				Spec::new(r"^\.DS_Store$", "apple"),
				Spec::new(r"^\.pls\.yml$", "pls").importance(0),
				Spec::new(r"^\.gitignore$", "git"),
				Spec::new(r"^\.github$", "github"),
				Spec::new(r"^src$", "source").importance(1),
				Spec::new(r"^(justfile|Makefile)$", "runner"),
				Spec::new(r"^Cargo\.toml$", "package"),
				Spec::new(r"^Cargo\.lock$", "lock")
					.importance(-1)
					.collapse(Collapse::Name(String::from("Cargo.toml"))),
				Spec::new(r"^rustfmt.toml$", "broom"),
				// Partial names
				Spec::new(r"^\.env\b", "env"),
				Spec::new(r"^README\b", "book").importance(2),
				Spec::new(r"^LICENSE\b", "law"),
				Spec::new(r"docker-compose.*\.yml$", "container"),
				Spec::new(r"Dockerfile", "container"),
				// Extensions
				Spec::new(r"\.sh$", "shell"),
				Spec::new(r"\.rs$", "rust").style("rgb(247,76,0)"),
				Spec::new(r"\.txt$", "text"),
				Spec::new(r"\.md$", "markdown"),
				Spec::new(r"\.ini$", "config"),
				Spec::new(r"\.(json|toml|yml|yaml)$", "json"),
				Spec::new(r"\.(jpg|jpeg|png|svg|webp|gif|ico)$", "image"),
				Spec::new(r"\.(mov|mp4|mkv|webm|avi|flv)$", "video"),
				Spec::new(r"\.(mp3|flac|ogg|wav)$", "audio"),
			],
			constants: Constants::default(),
		}
	}
}
