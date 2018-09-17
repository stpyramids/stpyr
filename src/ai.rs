use super::{display::Location, events::*, pos::*};
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
        WriteStorage<'a, Location>,
        Write<'a, Events>,
    );

    fn run(&mut self, (entities, target, mut pos, mut events): Self::SystemData) {
        use specs::Join;

        for (entity, target, pos) in (&*entities, &target, &mut pos).join() {
            if target.pos == pos.pos {
                events.push(Event::TargetReached(entity));
            } else {
                let mut diff = target.diff(pos);
                if diff.0 > 1 {
                    diff.0 = 1
                }
                if diff.0 < -1 {
                    diff.0 = -1
                }
                if diff.1 > 1 {
                    diff.1 = 1
                }
                if diff.1 < -1 {
                    diff.1 = -1
                }
                pos.move_pos(diff);
            }
        }
    }
}
