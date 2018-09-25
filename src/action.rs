use super::{energy::*, events::*, log::*, map::*, player::*, pos::*};
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
        ReadStorage<'a, PlayerBrain>,
        ReadStorage<'a, TileMap>,
        Write<'a, Events>,
        Write<'a, DebugLog>,
    );

    fn run(
        &mut self,
        (entities, mut turns, mut energies, mut pos, player, maps, mut events, mut debug): Self::SystemData,
){
        use specs::Join;
        let map: &TileMap;

        {
            let (playerpos, &_) = (&mut pos, &player).join().next().unwrap();
            map = maps.get(playerpos.map).unwrap();
        }

        for (entity, turn, energy, pos) in (&*entities, &mut turns, &mut energies, &mut pos).join()
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
                            new_pos.clamp((0, 0), (map.tiles.width - 1, map.tiles.height - 1));
                        if map.at(new_pos).solid {
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
