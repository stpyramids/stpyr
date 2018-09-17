extern crate specs;
#[macro_use]
extern crate specs_derive;

mod pos;
use self::pos::*;

use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Location {
    pos: Pos,
}

impl HasPos for Location {
    fn pos(&self) -> &Pos {
        &self.pos
    }
    fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct WalkTarget {
    pos: Pos,
}

impl HasPos for WalkTarget {
    fn pos(&self) -> &Pos {
        &self.pos
    }
    fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Glyph(char);

const MAP_WIDTH: usize = 15;
const MAP_HEIGHT: usize = 15;

struct DisplayS;
impl<'a> System<'a> for DisplayS {
    type SystemData = (
        ReadStorage<'a, Location>,
        ReadStorage<'a, Glyph>,
        Read<'a, Events>,
    );

    fn run(&mut self, (position, glyph, events): Self::SystemData) {
        use specs::Join;

        let mut mapbuf: Vec<char> = vec![];
        mapbuf.resize(MAP_WIDTH * MAP_HEIGHT, '.');

        for (position, glyph) in (&position, &glyph).join() {
            let idx = position.pos_to_idx(MAP_WIDTH);
            mapbuf[idx] = glyph.0;
        }
        println!("TURN:");
        for row in mapbuf.chunks(MAP_WIDTH) {
            let rowstr: String = row.into_iter().collect();
            println!("{}", rowstr);
        }
        for evt in &events.events {
            println!("LOG: {:?}", evt);
        }
    }
}

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
struct PlayerBrain;

struct PlayerMoveS;
impl<'a> System<'a> for PlayerMoveS {
    type SystemData = (ReadStorage<'a, PlayerBrain>, WriteStorage<'a, Location>);

    fn run(&mut self, (_player, mut pos): Self::SystemData) {
        use specs::Join;

        for pos in (&mut pos).join() {
            // pass
        }
    }
}

struct AIMoveS;
impl<'a> System<'a> for AIMoveS {
    type SystemData = (
        ReadStorage<'a, WalkTarget>,
        WriteStorage<'a, Location>,
        Write<'a, Events>,
    );

    fn run(&mut self, (target, mut pos, mut events): Self::SystemData) {
        use specs::Join;

        for (target, pos) in (&target, &mut pos).join() {
            if target.pos == pos.pos {
                events.push(Event::TargetReached);
            } else {
                let mut diff = target.diff(pos);
                if diff.0 > 1 {
                    diff.0 = 1
                }
                if diff.0 < -1 {
                    diff.0 = -1
                }
                if diff.1 > 1 {
                    diff.1 = 1
                }
                if diff.1 < -1 {
                    diff.1 = -1
                }
                pos.move_pos(diff);
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Event {
    TargetReached,
}

#[derive(Default)]
struct Events {
    events: Vec<Event>,
    next_events: Vec<Event>,
}

impl Events {
    fn new() -> Self {
        Events {
            events: vec![],
            next_events: vec![],
        }
    }

    fn pump(&self) -> Self {
        Events {
            events: self.next_events.to_vec(),
            next_events: vec![],
        }
    }

    fn push(&mut self, e: Event) {
        self.next_events.push(e);
    }
}

struct EventPumpS;
impl<'a> System<'a> for EventPumpS {
    type SystemData = Write<'a, Events>;

    fn run(&mut self, data: Self::SystemData) {
        let mut events = data;
        *events = events.pump();
    }
}

fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(AIMoveS, "ai_move", &[])
        .with(PlayerMoveS, "player_move", &["ai_move"])
        .with_thread_local(DisplayS)
        .with_thread_local(EventPumpS)
        .build();
    dispatcher.setup(&mut world.res);

    world.add_resource(Events::new());
    world
        .create_entity()
        .with(Glyph('@'))
        .with(PlayerBrain)
        .with(Location { pos: Pos(7, 9) })
        .build();
    world
        .create_entity()
        .with(Glyph('s'))
        .with(Location { pos: Pos(1, 1) })
        .with(WalkTarget { pos: Pos(3, 5) })
        .build();
    world
        .create_entity()
        .with(Glyph('#'))
        .with(Location { pos: Pos(2, 4) })
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();
    dispatcher.dispatch(&mut world.res);
    world.maintain();
    dispatcher.dispatch(&mut world.res);
    world.maintain();
    dispatcher.dispatch(&mut world.res);
    world.maintain();
    dispatcher.dispatch(&mut world.res);
    world.maintain();
}
