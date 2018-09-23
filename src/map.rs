use super::{curses::Glyph, pos::*};
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Debug, Clone)]
pub struct Tile {
    pub glyph: Glyph,
    pub solid: bool,
    pub opaque: bool,
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            tiles: vec![
                Tile {
                    glyph: Glyph('.'),
                    solid: false,
                    opaque: false
                };
                (width * height) as usize
            ],
        }
    }

    pub fn at(&self, pos: Pos) -> &Tile {
        &self.tiles[pos.to_idx(self.width)]
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Location {
    pub map: Entity,
    pub pos: Pos,
}

impl HasPos for Location {
    fn pos(&self) -> &Pos {
        &self.pos
    }
    fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}
