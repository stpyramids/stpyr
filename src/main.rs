extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate ncurses;

pub mod action;
pub mod ai;
pub mod behavior;
pub mod curses;
pub mod energy;
pub mod events;
pub mod log;
pub mod map;
pub mod player;
pub mod pos;

use self::pos::*;
use specs::prelude::{Builder, DispatcherBuilder, World};

fn main() {
    curses::CursesDisplayS::init();

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(energy::EnergyS, "energy", &[])
        .with(behavior::HunterBrainS, "hunter_brain", &["energy"])
        .with(ai::AIMoveS, "ai_move", &["hunter_brain"])
        .with(player::PlayerMoveS, "player_move", &["ai_move"])
        .with(action::TurnS, "turn", &["player_move"])
        .with_thread_local(curses::CursesDisplayS)
        .with_thread_local(events::EventPumpS)
        .build();
    dispatcher.setup(&mut world.res);

    world.add_resource(events::Events::new());
    world.add_resource(player::Input(None));
    let map = world
        .create_entity()
        .with(map::Map {
            width: 15,
            height: 15,
        }).build();
    world
        .create_entity()
        .with(curses::Glyph('@'))
        .with(player::PlayerBrain)
        .with(map::Location {
            map: map,
            pos: Pos(7, 9),
        }).build();
    world
        .create_entity()
        .with(curses::Glyph('s'))
        .with(energy::Energy::new(0.2))
        .with(action::Turn::wait())
        .with(behavior::HunterBrain::new(1))
        .with(map::Location {
            map: map,
            pos: Pos(1, 1),
        }).build();
    world
        .create_entity()
        .with(curses::Glyph('c'))
        .with(energy::Energy::new(1.1))
        .with(action::Turn::wait())
        .with(behavior::HunterBrain::new(3))
        .with(map::Location {
            map: map,
            pos: Pos(13, 12),
        }).build();

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        let ch = curses::CursesDisplayS::getch();
        match ch {
            'q' => break,
            other => {
                let mut input = world.write_resource::<player::Input>();
                *input = player::Input(Some(other));
            }
        }
    }

    curses::CursesDisplayS::finish();
}
