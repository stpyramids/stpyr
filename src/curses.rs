use super::{
    appearance::*, display::*, events::*, fov::*, grid::*, log::DebugLog, map::*, player::*, pos::*,
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

pub struct CursesDisplayS {
    gamewin: WINDOW,
    logwin:  WINDOW,
}

impl CursesDisplayS {
    pub fn new() -> Self {
        CursesDisplayS {
            gamewin: newwin(30, 30, 0, 0),
            logwin:  newwin(30, 80, 0, 32),
        }
    }
}

impl Default for CursesDisplayS {
    fn default() -> Self {
        Self::new()
    }
}

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

        let mut display: Grid<char> = Grid::new(map.tiles.width, map.tiles.height, ' ');
        for pos in Pos(0, 0).iter_to(Pos(map.tiles.width - 1, map.tiles.height - 1)) {
            if fov.visible(pos) {
                display.set(pos, map.tiles.at(pos).glyph.ascii());
            }
        }

        for (position, appearance) in (&position, &apps).join() {
            if fov.visible(position.pos) {
                display.set(position.pos, appearance.glyph.ascii());
            }
        }

        wclear(self.gamewin);
        for pos in Pos(0, 0).iter_to(Pos(display.width - 1, display.height - 1)) {
            mvwaddch(
                self.gamewin,
                pos.1 as i32,
                pos.0 as i32,
                *display.at(pos) as u32,
            );
        }
        wrefresh(self.gamewin);

        wclear(self.logwin);
        for evt in &events.events {
            wprintw(self.logwin, &format!("EVENT: {:?}\n", evt));
        }
        for message in &log.messages {
            wprintw(self.logwin, &format!("LOG: {}\n", message));
        }
        log.messages.clear();
        wrefresh(self.logwin);
    }
}
