use crate::models::Constants;
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
			constants: Constants::default(),
		}
	}
}
