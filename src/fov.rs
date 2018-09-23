use super::{map::*, pos::*};
use line_drawing::Bresenham;
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Debug, Component)]
#[storage(BTreeStorage)]
pub struct FovMap {
    pub mapped: bool,
    pub width: u32,
    pub height: u32,
    pub blocked: Vec<bool>,
    pub visible: Vec<bool>,
}

impl Default for FovMap {
    fn default() -> FovMap {
        FovMap::new(1, 1)
    }
}

impl FovMap {
    pub fn new(width: u32, height: u32) -> FovMap {
        FovMap {
            mapped: false,
            width,
            height,
            blocked: vec![false; (width * height + 1) as usize],
            visible: vec![true; (width * height + 1) as usize],
        }
    }
    pub fn new_for_map(map: &TileMap) -> FovMap {
        let mut fov = FovMap::new(map.width, map.height);
        for (idx, tile) in map.tiles.iter().enumerate() {
            fov.blocked[idx] = tile.opaque;
        }
        fov
    }
    pub fn visible(&self, pos: Pos) -> bool {
        if !self.mapped {
            true
        } else {
            self.visible[pos.to_idx(self.width)]
        }
    }
    pub fn blocked(&self, pos: Pos) -> bool {
        self.blocked[pos.to_idx(self.width)]
    }
    pub fn contains(&self, pos: Pos) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }
    pub fn compute(&mut self, pov: Pos) {
        let Pos(px, py) = pov;
        for lx in 0..self.width {
            for ly in 0..self.height {
                for (x, y) in Bresenham::new((px as i32, py as i32), (lx as i32, ly as i32)) {
                    if self.blocked(Pos(x as u32, y as u32)) {
                        self.visible[Pos(lx, ly).to_idx(self.width)] = false;
                    }
                }
            }
        }
        self.mapped = true;
    }
}

pub struct FovS;
impl<'a> System<'a> for FovS {
    type SystemData = (
        WriteStorage<'a, FovMap>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, TileMap>,
    );

    fn run(&mut self, (mut fovs, locs, maps): Self::SystemData) {
        use specs::Join;

        for (fov, loc) in (&mut fovs, &locs).join() {
            let map = maps.get(loc.map).unwrap();
            let mut newfov = FovMap::new_for_map(&map);
            newfov.compute(loc.pos);
            *fov = newfov;
        }
    }
}
