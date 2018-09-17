use super::{display::Location, energy::*, events::*, pos::*};
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
        WriteStorage<'a, Energy>,
        Write<'a, Events>,
    );

    fn run(&mut self, (entities, target, mut pos, mut energy, mut events): Self::SystemData) {
        use specs::Join;

        for (entity, target, pos, energy) in (&*entities, &target, &mut pos, &mut energy).join() {
            if target.pos == pos.pos {
                events.push(Event::TargetReached(entity));
            } else {
                if energy.try_spend(1.0) {
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
}
