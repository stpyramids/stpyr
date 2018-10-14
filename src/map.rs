use super::{appearance::Glyph, grid::*, pos::*, tile_generator::TileGenerator, vault::*};
use failure::Error;
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Debug, Clone)]
pub struct Tile {
    pub glyph:  Glyph,
    pub solid:  bool,
    pub opaque: bool,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            glyph:  Glyph::new('.'),
            solid:  false,
            opaque: false,
        }
    }
}

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct TileMap {
    pub tiles: Grid<Tile>,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> TileMap {
        TileMap {
            tiles: Grid::new(width, height, Tile::default()),
        }
    }

    pub fn at(&self, pos: Pos) -> &Tile {
        self.tiles.at(pos)
    }

    pub fn contains(&self, pos: Pos) -> bool {
        self.tiles.contains(pos)
    }

    pub fn place_vault(&mut self, start: Pos, vault: &Vault) -> Result<(), Error> {
        self.place(
            start,
            start + PosDiff(vault.tiles.width as i32, vault.tiles.height as i32),
            vault,
        )
    }

    pub fn place(
        &mut self,
        start: Pos,
        end: Pos,
        generator: &dyn TileGenerator,
    ) -> Result<(), Error> {
        let tiles = generator
            .generate(&self.tiles, (start, end))
            .expect("couldn't generate");
        for (pos, tile) in tiles {
            self.tiles.set(pos, tile)
        }
        Ok(())
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Location {
    pub map: Entity,
    pub pos: Pos,
}

impl HasPos for Location {
    fn pos(&self) -> Pos {
        self.pos
    }

    fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}
