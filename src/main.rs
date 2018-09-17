extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate ncurses;

pub mod ai;
pub mod behavior;
pub mod display;
pub mod events;
pub mod player;
pub mod pos;

use self::{ai::*, behavior::*, display::*, events::*, player::*, pos::*};
use specs::prelude::{Builder, DispatcherBuilder, World};

fn main() {
    DisplayS::init();

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HunterBrainS, "hunter_brain", &[])
        .with(AIMoveS, "ai_move", &["hunter_brain"])
        .with(PlayerMoveS, "player_move", &["ai_move"])
        .with_thread_local(DisplayS)
        .with_thread_local(EventPumpS)
        .build();
    dispatcher.setup(&mut world.res);

    world.add_resource(Events::new());
    world.add_resource(Input(None));
    world
        .create_entity()
        .with(Glyph('@'))
        .with(PlayerBrain)
        .with(Location { pos: Pos(7, 9) })
        .build();
    world
        .create_entity()
        .with(Glyph('s'))
        .with(HunterBrain::new(5))
        .with(Location { pos: Pos(1, 1) })
        .build();
    world
        .create_entity()
        .with(Glyph('c'))
        .with(HunterBrain::new(1))
        .with(Location { pos: Pos(13, 12) })
        .build();
    world
        .create_entity()
        .with(Glyph('#'))
        .with(Location { pos: Pos(2, 4) })
        .build();

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        let ch = DisplayS::getch();
        match ch {
            'q' => break,
            other => {
                let mut input = world.write_resource::<Input>();
                *input = Input(Some(other));
            }
        }
    }

    DisplayS::finish();
}
