extern crate specs;
extern crate specs_derive;
extern crate stpyrl;

use specs::prelude::*;
use stpyrl::{pos::*, *};

fn main() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        curses::CursesDisplayS::finish();
        default_panic(panic);
    }));

    run_game();
    curses::CursesDisplayS::finish();
}

fn make_actor(
    world: &mut World,
    map: Entity,
    glyph: char,
    speed: f32,
    pos: Pos,
    extra: fn(EntityBuilder) -> EntityBuilder,
) {
    let builder = world
        .create_entity()
        .with(curses::Glyph(glyph))
        .with(energy::Energy::new(speed))
        .with(action::Turn::default())
        .with(map::Location { map, pos })
        .with(fov::FovMap::default())
        .with(movement::MovementMap::default());
    extra(builder).build();
}

fn make_player(world: &mut World, map: Entity) {
    make_actor(world, map, '@', 1.0, Pos(7, 9), |builder| {
        builder.with(player::PlayerBrain)
    });
}

fn run_game() {
    curses::CursesDisplayS::init();

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(player::PlayerStateS, "player", &[])
        .with(action::ActiveS, "active", &["player"])
        .with(movement::MovementS, "movement", &["active"])
        .with(fov::FovS, "fov", &["movement"])
        .with(energy::EnergyS, "energy", &["fov"])
        .with(behavior::HunterBrainS, "hunter_brain", &["energy"])
        .with(ai::AIMoveS, "ai_move", &["hunter_brain"])
        .with(player::PlayerMoveS, "player_move", &["ai_move"])
        .with(action::TurnS, "turn", &["player_move"])
        .with_thread_local(curses::CursesDisplayS)
        .with_thread_local(events::EventPumpS)
        .build();
    dispatcher.setup(&mut world.res);

    let loader = resources::FileResourceDataLoader::new("res");
    let adventure = adventure::Adventure::new(loader);
    let firstmap = adventure.first_map();
    let map = world.create_entity().with(firstmap).build();

    world.add_resource(events::Events::new());
    world.add_resource(player::GameState::Starting);
    world.add_resource(adventure);

    make_player(&mut world, map);
    make_actor(&mut world, map, 's', 0.2, Pos(1, 1), |b| {
        b.with(behavior::HunterBrain::new(1))
    });
    make_actor(&mut world, map, 'c', 1.1, Pos(13, 12), |b| {
        b.with(behavior::HunterBrain::new(3))
    });

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        let ch = curses::CursesDisplayS::getch();
        match ch {
            Some(ch) => match ch {
                'q' => break,
                other => {
                    let mut state = world.write_resource::<player::GameState>();
                    *state = player::GameState::Active(Some(other));
                }
            },
            None => {
                let mut state = world.write_resource::<player::GameState>();
                *state = player::GameState::Idle;
            }
        };
    }
}
