use super::{map::Location, pos::*};
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
