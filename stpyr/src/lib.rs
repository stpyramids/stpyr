#![feature(try_blocks)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
// SystemData is often complex :(
#![cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]

#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod action;
pub mod adventure;
pub mod ai;
pub mod appearance;
pub mod behavior;
pub mod def;
pub mod display;
pub mod energy;
pub mod events;
pub mod fov;
pub mod grid;
pub mod labyrinth;
pub mod map;
pub mod movement;
pub mod player;
pub mod pos;
pub mod resources;
pub mod scene;
pub mod tile_generator;
pub mod vault;
