#![feature(try_blocks)]
#![feature(tool_lints)]
#![warn(clippy::all)]
// SystemData is often complex :(
#![cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]

extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate line_drawing;
extern crate ncurses;
extern crate pathfinding;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod action;
pub mod adventure;
pub mod ai;
pub mod appearance;
pub mod behavior;
pub mod curses;
pub mod def;
pub mod energy;
pub mod events;
pub mod fov;
pub mod grid;
pub mod log;
pub mod map;
pub mod movement;
pub mod player;
pub mod pos;
pub mod resources;
pub mod scene;
pub mod vault;
