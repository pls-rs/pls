mod config;
mod enums;
mod exc;
mod fmt;
mod models;
mod output;
mod traits;

use log::debug;
use models::Pls;

/// Create a `Pls` instance and immediately delegate to it.
///
/// This is the entry point of the application.
fn main() {
	env_logger::init();
	debug!("Hello!");

	let pls = Pls::default();
	pls.run();

	debug!("Bye!");
}
