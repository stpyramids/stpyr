use super::action::*;
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
    pub fn active(&self) -> bool { *self != GameState::Idle }

    fn input(&self) -> Option<char> {
        match self {
            GameState::Active(opt) => *opt,
            _ => None,
        }
    }
}
impl Default for GameState {
    fn default() -> GameState { GameState::Idle }
}

pub struct PlayerMoveS;
impl<'a> System<'a> for PlayerMoveS {
    type SystemData = (
        Read<'a, GameState>,
        ReadStorage<'a, PlayerBrain>,
        WriteStorage<'a, Turn>,
    );

    fn run(&mut self, (game, player, mut turn): Self::SystemData) {
        use specs::Join;

        for (_, turn) in (&player, &mut turn).join() {
            *turn = match game.input() {
                Some(ch) => match ch {
                    'h' => Turn::walk(-1, 0),
                    'j' => Turn::walk(0, 1),
                    'k' => Turn::walk(0, -1),
                    'l' => Turn::walk(1, 0),
                    _ => Turn::wait(),
                },
                None => Turn::wait(),
            };
        }
    }
}
