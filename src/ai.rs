use super::{action::Turn, events::*, map::Location, pos::*};
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
    );

    fn run(&mut self, (entities, target, pos, mut turn, mut events): Self::SystemData) {
        use specs::Join;

        for (entity, target, pos, turn) in (&*entities, &target, &pos, &mut turn).join() {
            *turn = Turn::wait();

            if target.pos == pos.pos {
                events.push(Event::TargetReached(entity));
            } else {
                let PosDiff(dx, dy) = target.diff(pos).clamp((-1, -1), (1, 1));
                *turn = Turn::walk(dx, dy);
            }
        }
    }
}
