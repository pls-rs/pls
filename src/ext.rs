//! This module contains extensions to the standard library.
//!
//! The public interface of this module consists of the following traits:
//!
//! * [`Abs`]
//! * [`Ctime`]

mod abs;
mod ctime;

pub use abs::Abs;
pub use ctime::Ctime;
