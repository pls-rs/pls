//! This module contains code for working with VS Code extensions.
//!
//! We use VS Code extensions to provide icon packs for the application. These
//! are much more expressive, colorful and pretty compared to Nerd Fonts.

mod ext;
mod package;

pub use ext::ExtRef;
pub use package::{ExtPackage, IconThemeDef};
