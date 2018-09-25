use super::{curses::Glyph, grid::*, pos::*};
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
    pub tiles: Grid<Tile>,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> TileMap {
        TileMap {
            tiles: Grid::new(
                width,
                height,
                Tile {
                    glyph: Glyph('.'),
                    solid: false,
                    opaque: false,
                },
            ),
        }
    }
    pub fn at(&self, pos: Pos) -> &Tile {
        self.tiles.at(pos)
    }
    pub fn contains(&self, pos: Pos) -> bool {
        self.tiles.contains(pos)
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
