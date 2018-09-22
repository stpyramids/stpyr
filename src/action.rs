use super::{energy::*, map::Location, pos::*};
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Turn {
    cost: f32,
    action: Action,
}

impl Default for Turn {
    fn default() -> Turn {
        Turn {
            cost: 0.0,
            action: Action::Wait,
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
        }
    }
}
pub struct TurnS;
impl<'a> System<'a> for TurnS {
    type SystemData = (
        WriteStorage<'a, Turn>,
        WriteStorage<'a, Energy>,
        WriteStorage<'a, Location>,
    );

    fn run(&mut self, (mut turns, mut energies, mut pos): Self::SystemData) {
        use specs::Join;

        for (turn, energy, pos) in (&mut turns, &mut energies, &mut pos).join() {
            if energy.try_spend(turn.cost) {
                match turn.action {
                    Action::Wait => (),
                    Action::Walk(dx, dy) => {
                        pos.move_pos_xy(dx, dy);
                    }
                };
                *turn = Turn::default();
            }
        }
    }
}
