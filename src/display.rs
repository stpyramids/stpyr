use super::{events::*, pos::*};
use ncurses::*;
use specs::prelude::*;
use std::char;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Location {
    pub pos: Pos,
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
pub struct Glyph(pub char);

const MAP_WIDTH: usize = 15;
const MAP_HEIGHT: usize = 15;

pub struct DisplayS;

impl DisplayS {
    pub fn init() {
        initscr();
        raw();

        keypad(stdscr(), true);
        noecho();
    }
    pub fn finish() {
        endwin();
    }
    pub fn getch() -> char {
        char::from_u32(getch() as u32).unwrap()
    }
}

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
