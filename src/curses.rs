use super::{
    appearance::*, display::*, events::*, fov::*, log::DebugLog, map::*, player::*, pos::*,
};
use ncurses::*;
use specs::prelude::*;
use std::char;

pub struct CursesDisplay();

impl CursesDisplay {
    pub fn init() -> Self {
        initscr();
        raw();

        keypad(stdscr(), true);
        timeout(0);
        noecho();

        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            endwin();
            default_panic(panic);
        }));

        CursesDisplay()
    }
}

impl Display for CursesDisplay {
    fn getch(&self) -> Option<char> {
        char::from_u32(getch() as u32)
    }
}

impl Drop for CursesDisplay {
    fn drop(&mut self) {
        endwin();
    }
}

pub struct CursesDisplayS;

impl<'a> System<'a> for CursesDisplayS {
    type SystemData = (
        ReadStorage<'a, Location>,
        ReadStorage<'a, Appearance>,
        ReadStorage<'a, TileMap>,
        ReadStorage<'a, FovMap>,
        Read<'a, Option<PlayerState>>,
        Read<'a, Events>,
        Read<'a, GameState>,
        Write<'a, DebugLog>,
    );

    fn run(
        &mut self,
        (position, apps, maps, fovs, player, events, game, mut log): Self::SystemData,
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
            .map(|(idx, t)| {
                if fov.visible[idx] {
                    t.glyph.ascii()
                } else {
                    ' '
                }
            }).collect();

        for (position, appearance) in (&position, &apps).join() {
            let idx = position.pos_to_idx(map.tiles.width as usize);
            if fov.visible(position.pos) {
                mapbuf[idx] = appearance.glyph.ascii();
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
