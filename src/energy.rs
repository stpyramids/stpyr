use super::log::DebugLog;
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

    pub fn try_spend(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }
}

pub struct EnergyS;

impl<'a> System<'a> for EnergyS {
    type SystemData = (Entities<'a>, WriteStorage<'a, Energy>, Write<'a, DebugLog>);

    fn run(&mut self, (entities, mut energy, mut debug): Self::SystemData) {
        use specs::Join;

        for (entity, energy) in (&*entities, &mut energy).join() {
            energy.tick();
            debug.log(format!("energy for {:?} = {}", entity, energy.current));
        }
    }
}
