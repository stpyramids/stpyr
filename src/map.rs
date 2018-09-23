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
pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> TileMap {
        TileMap {
            width,
            height,
            tiles: vec![
                Tile {
                    glyph: Glyph('.'),
                    solid: false,
                    opaque: false
                };
                (width * height + 1) as usize
            ],
        }
    }
    pub fn at(&self, pos: Pos) -> &Tile {
        &self.tiles[pos.to_idx(self.width)]
    }
    pub fn contains(&self, pos: Pos) -> bool {
        pos.0 < self.width && pos.1 < self.height
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
