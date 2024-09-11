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

use crate::models::Pls;

use log::debug;
use std::sync::LazyLock;

static PLS: LazyLock<Pls> = LazyLock::new(Pls::default);

/// Create a `Pls` instance and immediately delegate to it.
///
/// This is the entry point of the application.
fn main() {
	env_logger::init();
	debug!("Hello!");

	PLS.run();

	debug!("Bye!");
}
