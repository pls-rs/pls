//! This module contains code for working with graphics.
//!
//! Kitty terminal graphics protocol provides ways to render images in
//! the terminal. We use this protocol to show icons beyond the standard
//! collection present in Nerd Fonts.
//!
//! The public interface of the module consists of three functions:
//!
//! * [`icon_size`]
//! * [`is_supported`]
//! * [`render_image`]
//! * [`strip_image`]
//! * [`get_rgba`]

mod kitty;
mod svg;

pub use kitty::{icon_size, is_supported, render_image, strip_image};
pub use svg::get_rgba;
