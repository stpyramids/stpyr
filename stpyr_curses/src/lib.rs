#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
// SystemData is often complex :(
#![cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]

#[macro_use]
extern crate log;

pub mod curses;
pub use self::curses::*;
