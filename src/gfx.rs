//! This module contains code for working with graphics.
//!
//! Kitty terminal graphics protocol provides ways to render images in
//! the terminal. We use this protocol to show icons beyond the standard
//! collection present in Nerd Fonts.
//!
//! The public interface of the module consists of three functions:
//!
//! * [`is_supported`]
//! * [`get_rgba`]
//! * [`render_image`]

mod kitty;
mod svg;

pub use kitty::{is_supported, render_image};
pub use svg::get_rgba;
