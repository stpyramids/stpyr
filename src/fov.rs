use super::{action::*, grid::*, map::*, pos::*};
use line_drawing::Bresenham;
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Debug, Component)]
#[storage(BTreeStorage)]
pub struct FovMap {
    pub width:   u32,
    pub height:  u32,
    pub blocked: Grid<bool>,
    pub visible: Grid<bool>,
}

impl Default for FovMap {
    fn default() -> FovMap {
        FovMap::new(1, 1)
    }
}

impl FovMap {
    pub fn new(width: u32, height: u32) -> FovMap {
        FovMap {
            width,
            height,
            blocked: Grid::new(width, height, false),
            visible: Grid::new(width, height, true),
        }
    }

    pub fn new_for_map(map: &TileMap) -> FovMap {
        let mut fov = FovMap::new(map.tiles.width, map.tiles.height);
        for (idx, tile) in map.tiles.iter().enumerate() {
            fov.blocked[idx] = tile.opaque;
        }
        fov
    }

    pub fn visible(&self, pos: Pos) -> bool {
        *self.visible.at(pos)
    }

    pub fn blocked(&self, pos: Pos) -> bool {
        *self.blocked.at(pos)
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    pub fn compute(&mut self, pov: Pos) {
        let Pos(px, py) = pov;
        for lx in 0..self.width {
            for ly in 0..self.height {
                for (x, y) in Bresenham::new((lx as i32, ly as i32), (px as i32, py as i32)).skip(1)
                {
                    if *self.blocked.at(Pos(x as u32, y as u32)) {
                        self.visible.set(Pos(lx, ly), false);
                    }
                }
            }
        }
        self.visible.set(pov, true);
    }
}

pub struct FovS;
impl<'a> System<'a> for FovS {
    type SystemData = (
        WriteStorage<'a, FovMap>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, ActiveFlag>,
        ReadStorage<'a, TileMap>,
    );

    fn run(&mut self, (mut fovs, locs, actives, maps): Self::SystemData) {
        use specs::Join;

        for (fov, loc, ..) in (&mut fovs, &locs, &actives).join() {
            let map = maps.get(loc.map).unwrap();
            let mut newfov = FovMap::new_for_map(&map);
            newfov.compute(loc.pos);
            *fov = newfov;
        }
    }
}
