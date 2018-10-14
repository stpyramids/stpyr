use super::action::ActiveFlag;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Energy {
    pub per_tick: f32,
    pub current:  f32,
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
        self.current >= amount
    }
}

pub struct EnergyS;

impl<'a> System<'a> for EnergyS {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, ActiveFlag>,
        WriteStorage<'a, Energy>,
    );

    fn run(&mut self, (entities, actives, mut energy): Self::SystemData) {
        use specs::Join;

        for (entity, energy, ..) in (&*entities, &mut energy, &actives).join() {
            energy.tick();
            debug!("energy for {:?} = {}", entity, energy.current);
        }
    }
}
