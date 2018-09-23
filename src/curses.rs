use super::{events::*, fov::*, log::DebugLog, map::*, player::*, pos::*};
use ncurses::*;
use specs::prelude::*;
use std::char;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Glyph(pub char);

pub struct CursesDisplayS;

impl CursesDisplayS {
    pub fn init() {
        initscr();
        raw();

        keypad(stdscr(), true);
        timeout(0);
        noecho();
    }
    pub fn finish() {
        endwin();
    }
    pub fn getch() -> Option<char> {
        char::from_u32(getch() as u32)
    }
}

impl<'a> System<'a> for CursesDisplayS {
    type SystemData = (
        ReadStorage<'a, Location>,
        ReadStorage<'a, Glyph>,
        ReadStorage<'a, PlayerBrain>,
        ReadStorage<'a, TileMap>,
        ReadStorage<'a, FovMap>,
        Read<'a, Events>,
        Read<'a, GameState>,
        Write<'a, DebugLog>,
    );

    fn run(
        &mut self,
        (position, glyph, player, maps, fovs, events, game, mut log): Self::SystemData,
    ) {
        use specs::Join;

        if !game.active() {
            // Don't rerender except on a turn
            return;
        }

        let (playerpos, &_) = (&position, &player).join().next().unwrap();
        let map = maps.get(playerpos.map).unwrap();
        let (fov, &_) = (&fovs, &player).join().next().unwrap();

        let mut mapbuf: Vec<char> = map
            .tiles
            .as_slice()
            .into_iter()
            .map(|t| t.glyph.0)
            .collect();

        for (position, glyph) in (&position, &glyph).join() {
            let idx = position.pos_to_idx(map.width as usize);
            if fov.visible(position.pos) {
                mapbuf[idx] = glyph.0;
            }
        }

        clear();
        for row in mapbuf.chunks(map.width as usize) {
            let rowstr: String = row.into_iter().collect();
            printw(&format!("{}\n", rowstr));
        }
        printw(&format!("{:?}", fov.visible));
        for evt in &events.events {
            printw(&format!("EVENT: {:?}\n", evt));
        }
        for message in &log.messages {
            printw(&format!("LOG: {}\n", message));
        }
        log.messages.clear();
        refresh();
    }
}
