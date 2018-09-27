use super::{energy::*, events::*, log::*, map::*, movement::*, pos::*};
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Turn {
    cost: f32,
    action: Action,
    succeeded: bool,
}

impl Default for Turn {
    fn default() -> Turn {
        Turn {
            cost: 1.0,
            action: Action::Wait,
            succeeded: false,
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Wait,
    Walk(i32, i32),
}

impl Turn {
    pub fn wait() -> Turn {
        Turn::default()
    }
    pub fn walk(dx: i32, dy: i32) -> Turn {
        Turn {
            cost: 1.0,
            action: Action::Walk(dx, dy),
            succeeded: false,
        }
    }
}
pub struct TurnS;
impl<'a> System<'a> for TurnS {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Turn>,
        WriteStorage<'a, Energy>,
        WriteStorage<'a, Location>,
        WriteStorage<'a, MovementMap>,
        Write<'a, Events>,
        Write<'a, DebugLog>,
    );

    fn run(
        &mut self,
        (entities, mut turns, mut energies, mut pos, movemaps, mut events, mut debug): Self::SystemData,
){
        use specs::Join;

        for (entity, turn, energy, pos, movemap) in
            (&*entities, &mut turns, &mut energies, &mut pos, &movemaps).join()
        {
            if energy.can_spend(turn.cost) {
                debug.log(format!("{:?}", turn));
                match turn.action {
                    Action::Wait => {
                        turn.succeeded = true;
                    }
                    Action::Walk(dx, dy) => {
                        let new_pos = pos.move_pos_xy(dx, dy);
                        let new_pos =
                            new_pos.clamp((0, 0), (movemap.0.width - 1, movemap.0.height - 1));
                        if movemap.blocked(new_pos) {
                            events.push(Event::MoveFailed(entity, dx, dy));
                        } else {
                            pos.set_pos(new_pos);
                            turn.succeeded = true;
                        }
                    }
                };
            }
            if turn.succeeded {
                energy.spend(turn.cost);
            }
        }
    }
}
