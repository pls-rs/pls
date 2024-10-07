mod args;
mod config;
mod enums;
mod exc;
mod ext;
mod fmt;
mod gfx;
mod models;
mod output;
mod traits;
mod utils;

use crate::gfx::is_supported;
use crate::models::Pls;
use crate::models::Window;

use log::debug;
use std::sync::LazyLock;

static PLS: LazyLock<Pls> = LazyLock::new(|| {
	let window = Window::try_new();
	let supports_gfx = match &window {
		Some(win) if win.ws_xpixel > 0 && win.ws_ypixel > 0 => is_supported(),
		_ => false,
	};

	Pls {
		supports_gfx,
		window,
		..Pls::default()
	}
});

/// Create a `Pls` instance and immediately delegate to it.
///
/// This is the entry point of the application.
fn main() {
	env_logger::init();
	debug!("Hello!");

	PLS.cmd();

	debug!("Bye!");
}
