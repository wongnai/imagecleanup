#[macro_use]
extern crate log;
extern crate docker;

mod imagecleanup;
mod strategy;

pub use imagecleanup::*;
pub use strategy::*;
