use super::{adventure::*, def::*, display::*, pos::*, resources::*, *};
use specs::{prelude::*, shred};

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

pub struct AdventureScene<D>
where
    D: specs::RunNow<'_>,
{
    dispatcher: Dispatcher<'static, 'static>,
    _phantom:   std::marker::PhantomData<D>,
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

impl<D> AdventureScene<D>
where
    D: for<'_> specs::RunNow<'_> + specs::System<'_>,
{
    pub fn new(display: &D) -> AdventureScene<D> {
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
                .with(action::TurnS, "turn", &["player_move"])
                .with(fov::FovS, "fov_end", &["turn"])
                .with_barrier()
                .with_thread_local(*display)
                .with_thread_local(events::EventPumpS)
                .build(),
            _phantom:   std::marker::PhantomData,
        }
    }
}

impl<L: ResourceDataLoader, D: Display, DS: System<'static>> Scene<L, D> for AdventureScene<DS> {
    fn setup(&mut self, aworld: &mut AWorld<L, D>) {
        let world = &mut aworld.specs_world;
        let adventure = &aworld.adventure;

        self.dispatcher.setup(&mut world.res);

        let firstmap = aworld.adventure.first_map();
        let map = world.create_entity().with(firstmap).build();

        adventure
            .actor("player".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(player::PlayerBrain)
            .with(map::Location {
                map,
                pos: Pos(8, 8),
            })
            .build();

        adventure
            .actor("snake".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(behavior::HunterBrain::new(1))
            .with(map::Location {
                map,
                pos: Pos(1, 1),
            })
            .build();

        adventure
            .actor("cat".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(behavior::HunterBrain::new(3))
            .with(map::Location {
                map,
                pos: Pos(13, 12),
            })
            .build();
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
