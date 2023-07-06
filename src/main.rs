mod enums;
mod fmt;

use fmt::render;

fn main() {
	println!("{}", render("Hello, <blue bold>World!</>"));
}
