use super::{action::Turn, events::*, log::DebugLog, map::Location, pos::*};
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct WalkTarget {
    pub pos: Pos,
}

impl HasPos for WalkTarget {
    fn pos(&self) -> &Pos {
        &self.pos
    }
    fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}

pub struct AIMoveS;
impl<'a> System<'a> for AIMoveS {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, WalkTarget>,
        ReadStorage<'a, Location>,
        WriteStorage<'a, Turn>,
        Write<'a, Events>,
        Write<'a, DebugLog>,
    );

    fn run(&mut self, (entities, target, pos, mut turn, mut events, mut debug): Self::SystemData) {
        use specs::Join;

        for (entity, target, pos, turn) in (&*entities, &target, &pos, &mut turn).join() {
            if target.pos == pos.pos {
                events.push(Event::TargetReached(entity));
            } else {
                let PosDiff(mut dx, mut dy) = target.diff(pos);
                if dx > 1 {
                    dx = 1
                }
                if dx < -1 {
                    dx = -1
                }
                if dy > 1 {
                    dy = 1
                }
                if dy < -1 {
                    dy = -1
                }
                *turn = Turn::walk(dx, dy);
                debug.log(format!("Turn::walk({}, {})", dx, dy));
            }
        }
    }
}
