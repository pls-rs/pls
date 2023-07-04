mod config;
mod enums;
mod fmt;
mod models;

use fmt::render;

fn main() {
	println!("{}", render("Hello, <blue bold>World!</>"));
}
