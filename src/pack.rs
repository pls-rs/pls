//! This module contains code for downloading and installing VS Code icon packs.
//!
//! Its public interface backs the `add` and `list` actions of the `icon-pack`
//! subcommand.

mod install;
mod list;
mod openvsx;
mod source;
mod vsix;

pub use install::add;
pub use list::list;
