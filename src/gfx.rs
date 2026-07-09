//! This module contains code for working with graphics.
//!
//! Kitty terminal graphics protocol provides ways to render images in
//! the terminal. We use this protocol to show icons beyond the standard
//! collection present in Nerd Fonts.
//!
//! The public interface of the module consists of the following functions:
//!
//! * [`compute_hash`]
//! * [`is_supported`]
//! * [`render_image`]
//! * [`send_image`]
//! * [`strip_image`]
//! * [`get_rgba`]
//! * [`query_raw`] (crate-internal; used by [`ColorScheme`](crate::enums::ColorScheme))

mod hash;
mod kitty;
mod svg;
mod term;

pub use hash::compute_hash;
pub use kitty::{is_supported, render_image, send_image, strip_image};
pub use svg::get_rgba;
pub(crate) use term::query_raw;
