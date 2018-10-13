use super::{adventure::*, def::*, display::*, pos::*, resources::*, *};
use specs::prelude::*;

pub struct AWorld<L: ResourceDataLoader, D: Display> {
    pub specs_world: specs::World,
    pub display:     D,
    pub adventure:   Adventure<L>,
}

impl<L: ResourceDataLoader, D: Display> AWorld<L, D> {
    pub fn new(adventure: Adventure<L>, display: D) -> AWorld<L, D> {
        AWorld {
            specs_world: specs::World::new(),
            adventure,
            display,
        }
    }
}

pub struct AdventureScene {
    dispatcher: Dispatcher<'static, 'static>,
}

pub enum SceneChange<L: ResourceDataLoader, D: Display> {
    None,
    Switch(Box<dyn Scene<L, D>>),
    Push(Box<dyn Scene<L, D>>),
    Pop,
    Exit,
}

pub trait Scene<L: ResourceDataLoader, D: Display> {
    fn setup(&mut self, world: &mut AWorld<L, D>);
    fn update(&mut self, world: &mut AWorld<L, D>) -> SceneChange<L, D>;
}

impl AdventureScene {
    pub fn new() -> AdventureScene {
        AdventureScene {
            dispatcher: DispatcherBuilder::new()
                .with(player::PlayerStateS, "player", &[])
                .with(action::ActiveS, "active", &["player"])
                .with(fov::FovS, "fov_start", &["active"])
                .with(movement::MovementS, "movement", &["fov_start"])
                .with(energy::EnergyS, "energy", &["fov_start"])
                .with(behavior::HunterBrainS, "hunter_brain", &["energy"])
                .with(ai::AIMoveS, "ai_move", &["hunter_brain"])
                .with(player::PlayerMoveS, "player_move", &["energy"])
                .with_barrier()
                .with(action::TurnS, "turn", &["player_move"])
                .with(fov::FovS, "fov_end", &["turn"])
                .with_thread_local(curses::CursesDisplayS::new())
                .with_thread_local(events::EventPumpS)
                .build(),
        }
    }
}

impl Default for AdventureScene {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: ResourceDataLoader, D: Display> Scene<L, D> for AdventureScene {
    fn setup(&mut self, aworld: &mut AWorld<L, D>) {
        let world = &mut aworld.specs_world;
        let adventure = &aworld.adventure;

        self.dispatcher.setup(&mut world.res);

        let firstmap = aworld.adventure.first_map();
        let map = world.create_entity().with(firstmap).build();

        world.add_resource(events::Events::new());
        world.add_resource(player::GameState::Starting);

        adventure
            .actor("player".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(player::PlayerBrain)
            .with(map::Location {
                map,
                pos: Pos(7, 9),
            }).build();

        adventure
            .actor("snake".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(behavior::HunterBrain::new(1))
            .with(map::Location {
                map,
                pos: Pos(1, 1),
            }).build();

        adventure
            .actor("cat".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(behavior::HunterBrain::new(3))
            .with(map::Location {
                map,
                pos: Pos(13, 12),
            }).build();
    }

    fn update(&mut self, aworld: &mut AWorld<L, D>) -> SceneChange<L, D> {
        let world = &mut aworld.specs_world;
        self.dispatcher.dispatch(&world.res);
        world.maintain();

        let ch = aworld.display.getch();
        match ch {
            Some(ch) => match ch {
                'q' => return SceneChange::Exit,
                '!' => panic!("panic button pressed"),
                other => {
                    let mut state = world.write_resource::<player::GameState>();
                    *state = player::GameState::Active(Some(other));
                }
            },
            None => {
                let mut state = world.write_resource::<player::GameState>();
                *state = player::GameState::Idle;
            }
        }

        SceneChange::None
    }
}
