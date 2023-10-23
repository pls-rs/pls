//! This module contains code for working with paths entered as arguments to the
//! application.
//!
//! Since `pls` can accept multiple paths as positional arguments, the module
//! expresses them in terms of [`inputs`](Input) and [`groups`](Group).
//!
//! Each individual path is treated as one input. All directories given as
//! inputs are mapped to [`one group each`](Group::Dir). All files given as
//! input are collected into a [`single group`](Group::Files).
//!
//! The public interface of the module consists of two structs:
//!
//! * [`Group`]
//! * [`Input`]

mod dir_group;
mod files_group;
mod group;
mod input;

pub use group::Group;
pub use input::Input;
