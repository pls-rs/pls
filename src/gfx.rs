//! This module contains code for working with graphics.
//!
//! Kitty terminal graphics protocol provides ways to render images in
//! the terminal. We use this protocol to show icons beyond the standard
//! collection present in Nerd Fonts.
//!
//! The public interface of the module consists of five functions:
//!
//! * [`compute_hash`]
//! * [`is_supported`]
//! * [`render_image`]
//! * [`strip_image`]
//! * [`get_rgba`]

mod hash;
mod kitty;
mod svg;

pub use hash::compute_hash;
pub use kitty::{is_supported, render_image, strip_image};
pub use svg::get_rgba;
