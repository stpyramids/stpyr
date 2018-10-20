use super::{action::*, map::*, pos::*, wizard::*};
use specs::prelude::*;

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct PlayerBrain;

#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    Idle,
    Starting,
    Active(Option<char>),
}

impl GameState {
    pub fn active(&self) -> bool {
        *self != GameState::Idle
    }

    fn input(&self) -> Option<char> {
        match self {
            GameState::Active(opt) => *opt,
            _ => None,
        }
    }
}
impl Default for GameState {
    fn default() -> GameState {
        GameState::Idle
    }
}

#[derive(Copy, Clone)]
pub struct PlayerState {
    pub entity: Entity,
    pub map:    Entity,
    pub pos:    Pos,
}

pub struct PlayerStateS;
impl<'a> System<'a> for PlayerStateS {
    type SystemData = (
        Entities<'a>,
        Write<'a, Option<PlayerState>>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, PlayerBrain>,
    );

    fn run(&mut self, (entities, mut state, locs, brains): Self::SystemData) {
        *state =
            (&*entities, &brains, &locs)
                .join()
                .next()
                .map(|(entity, _, Location { pos, map })| PlayerState {
                    entity,
                    map: *map,
                    pos: *pos,
                });
    }
}
pub struct PlayerMoveS;
impl<'a> System<'a> for PlayerMoveS {
    type SystemData = (
        Read<'a, GameState>,
        ReadStorage<'a, PlayerBrain>,
        WriteStorage<'a, Turn>,
        Write<'a, WizardFlags>,
    );

    fn run(&mut self, (game, player, mut turn, mut wizard): Self::SystemData) {
        use specs::Join;

        for (_, turn) in (&player, &mut turn).join() {
            *turn = match game.input() {
                Some(ch) => match ch {
                    'h' => Turn::walk(-1, 0),
                    'j' => Turn::walk(0, 1),
                    'k' => Turn::walk(0, -1),
                    'l' => Turn::walk(1, 0),
                    '#' => {
                        wizard.toggle_xray();
                        Turn::wait()
                    }
                    _ => Turn::wait(),
                },
                None => Turn::wait(),
            };
        }
    }
}
