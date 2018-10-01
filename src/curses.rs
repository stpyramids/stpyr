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

    pub fn finish() { endwin(); }

    pub fn getch() -> Option<char> { char::from_u32(getch() as u32) }
}

impl<'a> System<'a> for CursesDisplayS {
    type SystemData = (
        ReadStorage<'a, Location>,
        ReadStorage<'a, Glyph>,
        ReadStorage<'a, TileMap>,
        ReadStorage<'a, FovMap>,
        Read<'a, Option<PlayerState>>,
        Read<'a, Events>,
        Read<'a, GameState>,
        Write<'a, DebugLog>,
    );

    fn run(
        &mut self,
        (position, glyph, maps, fovs, player, events, game, mut log): Self::SystemData,
    ) {
        use specs::Join;

        if !game.active() {
            // Don't rerender except on a turn
            return;
        }

        let map = maps.get(player.unwrap().map).unwrap();
        let fov = fovs.get(player.unwrap().entity).unwrap();

        let mut mapbuf: Vec<char> = map
            .tiles
            .iter()
            .enumerate()
            .map(|(idx, t)| if fov.visible[idx] { t.glyph.0 } else { ' ' })
            .collect();

        for (position, glyph) in (&position, &glyph).join() {
            let idx = position.pos_to_idx(map.tiles.width as usize);
            if fov.visible(position.pos) {
                mapbuf[idx] = glyph.0;
            }
        }

        clear();
        for row in mapbuf.chunks(map.tiles.width as usize) {
            let rowstr: String = row.into_iter().collect();
            printw(&format!("{}\n", rowstr));
        }
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
