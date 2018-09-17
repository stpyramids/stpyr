extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate ncurses;

use std::char;

mod pos;
use self::pos::*;

use ncurses::*;
use specs::{prelude::*, storage::BTreeStorage};

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

        clear();
        printw("TURN:\n");
        for row in mapbuf.chunks(MAP_WIDTH) {
            let rowstr: String = row.into_iter().collect();
            printw(&format!("{}\n", rowstr));
        }
        for evt in &events.events {
            printw(&format!("LOG: {:?}\n", evt));
        }
        refresh();
    }
}

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
struct PlayerBrain;

struct PlayerMoveS;
impl<'a> System<'a> for PlayerMoveS {
    type SystemData = (
        Read<'a, Input>,
        ReadStorage<'a, PlayerBrain>,
        WriteStorage<'a, Location>,
    );

    fn run(&mut self, (input, player, mut pos): Self::SystemData) {
        use specs::Join;

        for (_, pos) in (&player, &mut pos).join() {
            match input.0 {
                Some(ch) => match ch {
                    'h' => pos.move_pos_xy(-1, 0),
                    'j' => pos.move_pos_xy(0, 1),
                    'k' => pos.move_pos_xy(0, -1),
                    'l' => pos.move_pos_xy(1, 0),
                    _ => (),
                },
                None => (),
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Input(Option<char>);

#[derive(Component, Debug)]
#[storage(BTreeStorage)]
struct HunterBrain(HunterState);
#[derive(Debug)]
enum HunterState {
    Idle,
    Hunting,
    Satisfied(u32),
}

struct HunterBrainS;
impl<'a> System<'a> for HunterBrainS {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PlayerBrain>,
        WriteStorage<'a, HunterBrain>,
        ReadStorage<'a, Location>,
        WriteStorage<'a, WalkTarget>,
        Write<'a, Events>,
    );

    fn run(
        &mut self,
        (entities, player, mut hunter, pos, mut target, mut events): Self::SystemData,
    ) {
        use specs::Join;
        let mut playerpos: Option<&Pos> = None;

        for (player, pos) in (&player, &pos).join() {
            playerpos = Some(pos.pos());
        }
        match playerpos {
            Some(playerpos) => {
                for (entity, hunter, pos) in (&*entities, &mut hunter, &pos).join() {
                    match hunter.0 {
                        HunterState::Idle => {
                            events.push(Event::HunterHunts(entity));
                            hunter.0 = HunterState::Hunting;
                            target.insert(entity, WalkTarget { pos: *playerpos });
                        }
                        HunterState::Hunting => {
                            for evt in &events.events {
                                if let Event::TargetReached(entity) = evt {
                                    hunter.0 = HunterState::Satisfied(5);
                                    target.remove(*entity);
                                }
                            }
                        }
                        HunterState::Satisfied(n) => {
                            if n == 0 {
                                hunter.0 = HunterState::Idle;
                            } else {
                                hunter.0 = HunterState::Satisfied(n - 1);
                            }
                        }
                    }
                }
            }
            None => (),
        }
    }
}

struct AIMoveS;
impl<'a> System<'a> for AIMoveS {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, WalkTarget>,
        WriteStorage<'a, Location>,
        Write<'a, Events>,
    );

    fn run(&mut self, (entities, target, mut pos, mut events): Self::SystemData) {
        use specs::Join;

        for (entity, target, pos) in (&*entities, &target, &mut pos).join() {
            if target.pos == pos.pos {
                events.push(Event::TargetReached(entity));
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
    TargetReached(Entity),
    HunterHunts(Entity),
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

fn display_setup() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();
}

fn main() {
    display_setup();

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
        .with(HunterBrain(HunterState::Idle))
        .with(Location { pos: Pos(1, 1) })
        .build();
    world
        .create_entity()
        .with(Glyph('#'))
        .with(Location { pos: Pos(2, 4) })
        .build();

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        let ch = char::from_u32(getch() as u32).unwrap();
        match ch {
            'q' => break,
            other => {
                let mut input = world.write_resource::<Input>();
                *input = Input(Some(other));
            }
        }
    }

    endwin();
}
