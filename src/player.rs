use super::action::*;
use specs::prelude::*;

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct PlayerBrain;

#[derive(Clone, Debug, Default)]
pub struct Input(pub Option<char>);

pub struct PlayerMoveS;
impl<'a> System<'a> for PlayerMoveS {
    type SystemData = (
        Read<'a, Input>,
        ReadStorage<'a, PlayerBrain>,
        WriteStorage<'a, Turn>,
    );

    fn run(&mut self, (input, player, mut turn): Self::SystemData) {
        use specs::Join;

        for (_, turn) in (&player, &mut turn).join() {
            *turn = match input.0 {
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
