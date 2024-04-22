//! This module contains code for working with configuration and CLI arguments.
//!
//! `pls` supports customisation in two ways, through CLI arguments that change
//! the output per session and through `.pls.yml` YAML files that can go deeper
//! to tweak each individual string, change icons and add new node specs.
//! Together they make `pls` the most customisable file lister.
//!
//! For example, the CLI arg `--det` controls what metadata columns must be
//! shown in a given run, whereas the `.pls.yml` file can be used to change the
//! individual name for these columns.
//!
//! The public interface of the module consists of five structs:
//!
//! * [`AppConst`]
//! * [`Args`]
//! * [`Conf`]
//! * [`EntryConst`]
//! * [`ConfMan`]

mod app_const;
mod args;
mod conf;
mod entry_const;
mod man;

pub use app_const::AppConst;
pub use args::Args;
pub use conf::Conf;
pub use entry_const::EntryConst;
pub use man::ConfMan;
