use super::{action::*, events::*, map::*, pos::*};
use pathfinding::prelude::astar;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct WalkTarget {
    pub pos: Pos,
}

impl HasPos for WalkTarget {
    fn pos(&self) -> Pos { self.pos }

    fn set_pos(&mut self, pos: Pos) { self.pos = pos; }
}

pub struct AIMoveS;
impl<'a> System<'a> for AIMoveS {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, WalkTarget>,
        ReadStorage<'a, ActiveFlag>,
        ReadStorage<'a, Location>,
        WriteStorage<'a, Turn>,
        ReadStorage<'a, TileMap>,
        Write<'a, Events>,
    );

    fn run(
        &mut self,
        (entities, target, actives, pos, mut turn, maps, mut events): Self::SystemData,
    ) {
        use specs::Join;

        for (entity, target, pos, turn, ..) in
            (&*entities, &target, &pos, &mut turn, &actives).join()
        {
            let map = maps.get(pos.map).unwrap();

            *turn = Turn::wait();
            if target.pos == pos.pos {
                events.push(Event::TargetReached(entity));
            } else {
                let pos = pos.pos;
                let nextstep = astar(
                    &pos,
                    |p| {
                        p.neighbors()
                            .into_iter()
                            .filter(|n| map.contains(*n))
                            .map(|n| (n, if map.at(n).solid { 999 } else { 1 }))
                    },
                    |p| p.distance(target.pos) / 3,
                    |p| *p == target.pos,
                );
                if let Some((path, _cost)) = nextstep {
                    let nextpos = path[1];
                    let PosDiff(dx, dy) = nextpos.diff(pos);
                    *turn = Turn::walk(dx, dy);
                } else {
                    *turn = Turn::wait()
                }
            }
        }
    }
}
