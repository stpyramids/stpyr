use super::{action::*, grid::*, map::*, pos::*};
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Blockage {
    Open,
    Terrain,
    Critter,
    Player,
}

impl Default for Blockage {
    fn default() -> Self { Blockage::Open }
}

#[derive(Debug, Component)]
#[storage(BTreeStorage)]
pub struct MovementMap(pub Grid<Blockage>);

impl Default for MovementMap {
    fn default() -> MovementMap { MovementMap::new(1, 1) }
}

impl MovementMap {
    pub fn new(width: u32, height: u32) -> MovementMap {
        MovementMap(Grid::new(width, height, Blockage::Open))
    }

    pub fn new_for_map(map: &TileMap) -> MovementMap {
        let mut fov = MovementMap::new(map.tiles.width, map.tiles.height);
        for (idx, tile) in map.tiles.iter().enumerate() {
            if tile.solid {
                fov.0[idx] = Blockage::Terrain;
            }
        }
        fov
    }

    pub fn at(&self, pos: Pos) -> Blockage { self.0.at(pos).to_owned() }

    pub fn blocked(&self, pos: Pos) -> bool { *self.0.at(pos) != Blockage::Open }
}

pub struct MovementS;
impl<'a> System<'a> for MovementS {
    type SystemData = (
        WriteStorage<'a, MovementMap>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, ActiveFlag>,
        ReadStorage<'a, TileMap>,
    );

    fn run(&mut self, (mut movemaps, locs, actives, maps): Self::SystemData) {
        use specs::Join;

        for (movemap, loc, ..) in (&mut movemaps, &locs, &actives).join() {
            let map = maps.get(loc.map).unwrap();
            let mut newmovemap = MovementMap::new_for_map(&map);

            for (oloc,) in (&locs,).join() {
                if oloc.map == loc.map {
                    newmovemap.0.set(oloc.pos, Blockage::Critter);
                }
            }
            *movemap = newmovemap;
        }
    }
}
