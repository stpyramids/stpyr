#![feature(try_blocks)]

extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate line_drawing;
extern crate ncurses;
extern crate pathfinding;
#[macro_use]
extern crate failure;

pub mod action;
pub mod ai;
pub mod behavior;
pub mod curses;
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
pub mod vault;
pub mod adventure;