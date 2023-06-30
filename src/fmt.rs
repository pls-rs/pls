//! This module contains code for working with markup strings.
//!
//! Markup strings are a more convenient way to represent ANSI-formatted text.
//! They use an HTML-like syntax instead of arcane escape sequences.
//!
//! For example, to render the string "Hello, World!" in bold, you would write
//! `<bold>Hello, World!</>`.
//!
//! The tag consists of space separated directives. See [`fmt`](format::fmt) for
//! a list of supported directives. Tags can be nested, with inner tags capable
//! of overwriting directives from outer tags.
//!
//! The public interface of the module consists of two functions:
//!
//! * [`len`]
//! * [`render`]

mod format;
mod markup;

pub use markup::{len, render};
