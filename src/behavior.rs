use super::{ai::WalkTarget, events::*, map::Location, player::*, pos::*};
use specs::{prelude::*, storage::BTreeStorage};

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
pub struct HunterBrain {
    state: HunterState,
    laziness: u32,
}
#[derive(Debug)]
enum HunterState {
    Idle,
    Hunting,
    Satisfied(u32),
}
impl HunterBrain {
    pub fn new(laziness: u32) -> HunterBrain {
        HunterBrain {
            state: HunterState::Idle,
            laziness,
        }
    }
}

pub struct HunterBrainS;
impl<'a> System<'a> for HunterBrainS {
    type SystemData = (
        Entities<'a>,
        Read<'a, GameState>,
        ReadStorage<'a, PlayerBrain>,
        WriteStorage<'a, HunterBrain>,
        ReadStorage<'a, Location>,
        WriteStorage<'a, WalkTarget>,
        Write<'a, Events>,
    );

    fn run(
        &mut self,
        (entities, game, player, mut hunter, pos, mut target, mut events): Self::SystemData,
    ) {
        use specs::Join;
        let (playerpos, &_) = (&pos, &player).join().next().unwrap();

        if !game.active() {
            return;
        }

        for (entity, hunter) in (&*entities, &mut hunter).join() {
            match hunter.state {
                HunterState::Idle => {
                    events.push(Event::HunterHunts(entity));
                    hunter.state = HunterState::Hunting;
                    target
                        .insert(
                            entity,
                            WalkTarget {
                                pos: *playerpos.pos(),
                            },
                        ).unwrap();
                }
                HunterState::Hunting => {
                    for evt in &events.events {
                        if let Event::TargetReached(entity) = evt {
                            hunter.state = HunterState::Satisfied(hunter.laziness);
                            target.remove(*entity);
                        }
                    }
                }
                HunterState::Satisfied(n) => {
                    if n == 0 {
                        hunter.state = HunterState::Idle;
                    } else {
                        hunter.state = HunterState::Satisfied(n - 1);
                    }
                }
            }
        }
    }
}
