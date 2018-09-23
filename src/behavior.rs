use super::{ai::WalkTarget, events::*, fov::*, map::Location, player::*, pos::*};
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
        ReadStorage<'a, FovMap>,
        WriteStorage<'a, WalkTarget>,
        Write<'a, Events>,
    );

    fn run(
        &mut self,
        (entities, game, player, mut hunter, pos, fovs, mut target, mut events): Self::SystemData,
    ) {
        use specs::Join;
        let (playerpos, &_) = (&pos, &player).join().next().unwrap();
        let playerpos = *playerpos.pos();

        if !game.active() {
            return;
        }

        for (entity, hunter, fov) in (&*entities, &mut hunter, &fovs).join() {
            match hunter.state {
                HunterState::Idle => {
                    if fov.visible(playerpos) {
                        events.push(Event::HunterHunts(entity));
                        hunter.state = HunterState::Hunting;
                        target
                            .insert(entity, WalkTarget { pos: playerpos })
                            .unwrap();
                    } else {
                        hunter.state = HunterState::Satisfied(hunter.laziness);
                    }
                }
                HunterState::Hunting => {
                    for evt in &events.events {
                        match evt {
                            Event::TargetReached(entity) => {
                                hunter.state = HunterState::Satisfied(hunter.laziness);
                                target.remove(*entity);
                            }
                            Event::MoveFailed(entity, _, _) => {
                                hunter.state = HunterState::Idle;
                                target.remove(*entity);
                            }
                            _ => (),
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
