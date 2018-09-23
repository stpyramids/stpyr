use super::{log::DebugLog, player::*};
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Energy {
    pub per_tick: f32,
    pub current: f32,
}

impl Energy {
    pub fn new(per_tick: f32) -> Energy {
        Energy {
            per_tick,
            current: 0.0,
        }
    }

    fn tick(&mut self) {
        self.current += self.per_tick;
    }

    pub fn spend(&mut self, amount: f32) {
        if self.current >= amount {
            self.current -= amount;
        }
    }

    pub fn can_spend(&self, amount: f32) -> bool {
        if self.current >= amount {
            true
        } else {
            false
        }
    }
}

pub struct EnergyS;

impl<'a> System<'a> for EnergyS {
    type SystemData = (
        Entities<'a>,
        Read<'a, GameState>,
        WriteStorage<'a, Energy>,
        Write<'a, DebugLog>,
    );

    fn run(&mut self, (entities, game, mut energy, mut debug): Self::SystemData) {
        use specs::Join;

        if game.active() {
            for (entity, energy) in (&*entities, &mut energy).join() {
                energy.tick();
                debug.log(format!("energy for {:?} = {}", entity, energy.current));
            }
        }
    }
}
