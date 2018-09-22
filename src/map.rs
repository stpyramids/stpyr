use super::pos::*;
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct Map {
    pub width: usize,
    pub height: usize,
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
