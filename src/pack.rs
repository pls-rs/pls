//! This module contains code for downloading and installing VS Code icon packs.
//!
//! Its public interface backs the `icon-pack` subcommands ([`add`], [`list()`])
//! and the [`resolve`] entry point that turns an `icon_pack` config selection
//! (a pack ID and optional theme ID) into a theme file path.

mod add;
mod bust;
mod list;
mod remove;
mod theme;
mod vsix;

pub use add::add;
pub use bust::bust;
pub use list::list;
pub use remove::remove;
pub use theme::resolve;
