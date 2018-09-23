use super::{energy::*, log::*, map::*, player::*, pos::*};
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
            cost: 1.0,
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
        ReadStorage<'a, PlayerBrain>,
        ReadStorage<'a, Map>,
        Write<'a, DebugLog>,
    );

    fn run(
        &mut self,
        (mut turns, mut energies, mut pos, player, maps, mut debug): Self::SystemData,
    ) {
        use specs::Join;
        let map: &Map;

        {
            let (playerpos, &_) = (&mut pos, &player).join().next().unwrap();
            map = maps.get(playerpos.map).unwrap();
        }

        for (turn, energy, pos) in (&mut turns, &mut energies, &mut pos).join() {
            if energy.try_spend(turn.cost) {
                match turn.action {
                    Action::Wait => (),
                    Action::Walk(dx, dy) => {
                        pos.move_pos_xy(dx, dy);
                        pos.clamp((0, 0), (map.width - 1, map.height - 1));
                        debug.log(format!("{:?}", pos));
                    }
                };
                *turn = Turn::default();
            }
        }
    }
}
