use super::{grid::*, map::Tile, pos::*};

pub trait TileGenerator {
    fn generate(
        &self,
        current: &Grid<Tile>,
        start: Pos,
        end: Pos,
    ) -> Option<Vec<(Pos, Option<Tile>)>>;
}

pub mod generators {
    use super::{
        super::{grid::*, map::Tile, pos::*},
        TileGenerator,
    };

    pub struct Fill(pub Tile);

    impl TileGenerator for Fill {
        fn generate(
            &self,
            _current: &Grid<Tile>,
            start: Pos,
            end: Pos,
        ) -> Option<Vec<(Pos, Option<Tile>)>> {
            let bounds = (start, end);
            Some(
                start
                    .iter_to(end)
                    .map(|pos| (pos, Some(self.0.to_owned())))
                    .filter(|(pos, _)| pos.within(bounds))
                    .collect(),
            )
        }
    }
}
