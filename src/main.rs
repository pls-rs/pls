mod config;
mod enums;
mod fmt;
mod models;
mod output;

use models::Pls;

/// Create a `Pls` instance and immediately delegate to it.
///
/// This is the entry point of the application.
fn main() {
	let pls = Pls::default();
	pls.run();
}
