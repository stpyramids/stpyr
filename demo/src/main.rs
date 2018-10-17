#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

use fern;
use log;

use stpyr::prelude::*;
use stpyr_curses::*;

pub struct AdventureScene {
    dispatcher: Dispatcher<'static, 'static>,
}

impl AdventureScene {
    pub fn new() -> AdventureScene {
        AdventureScene {
            dispatcher: DispatcherBuilder::new()
                .with(PlayerStateS, "player", &[])
                .with(ActiveS, "active", &["player"])
                .with(FovS, "fov_start", &["active"])
                .with(MovementS, "movement", &["fov_start"])
                .with(EnergyS, "energy", &["fov_start"])
                .with(HunterBrainS, "hunter_brain", &["energy"])
                .with(AIMoveS, "ai_move", &["hunter_brain"])
                .with(PlayerMoveS, "player_move", &["energy"])
                .with(TurnS, "turn", &["player_move"])
                .with(FovS, "fov_end", &["turn"])
                .with_barrier()
                .with_thread_local(CursesDisplayS::new())
                .with_thread_local(EventPumpS)
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

        adventure
            .actor("player".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(PlayerBrain)
            .with(Location {
                map,
                pos: Pos(8, 8),
            })
            .build();

        adventure
            .actor("snake".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(HunterBrain::new(1))
            .with(Location {
                map,
                pos: Pos(1, 1),
            })
            .build();

        adventure
            .actor("cat".to_string())
            .unwrap()
            .mint(world.create_entity())
            .with(HunterBrain::new(3))
            .with(Location {
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
                    let mut state = world.write_resource::<GameState>();
                    *state = GameState::Active(Some(other));
                }
            },
            None => {
                let mut state = world.write_resource::<GameState>();
                *state = GameState::Idle;
            }
        }

        SceneChange::None
    }
}

fn main() {
    setup_logger().unwrap();

    let mut world = AWorld::new(
        Adventure::new(FileResourceDataLoader::new("demo/res")),
        CursesDisplay::init(),
    );
    let mut scene = AdventureScene::new();
    scene.setup(&mut world);

    loop {
        if let SceneChange::Exit = scene.update(&mut world) {
            break;
        }
    }
}

fn setup_logger() -> std::result::Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
