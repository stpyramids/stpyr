pub use super::{grid::*, map::Tile, pos::*};
use failure::Fail;

#[derive(Debug, Fail)]
pub enum GenError {
    #[fail(display = "destination bounds too small")]
    TooSmall,
}

pub trait TilePicker {
    fn pick(&self, current: &Grid<Tile>, pos: Pos) -> Option<Tile>;
}

pub trait TileGenerator {
    fn generate(&self, current: &Grid<Tile>, bounds: Bounds) -> Result<Vec<(Pos, Tile)>, GenError>;
}

pub mod pickers {
    use super::*;

    struct Only(Tile);

    impl TilePicker for Only {
        fn pick(&self, _current: &Grid<Tile>, _pos: Pos) -> Option<Tile> {
            Some(self.0.to_owned())
        }
    }

    pub fn only(tile: Tile) -> impl TilePicker {
        Only(tile)
    }

    struct Weighted(Vec<(u32, Tile)>);

    impl TilePicker for Weighted {
        fn pick(&self, _current: &Grid<Tile>, _pos: Pos) -> Option<Tile> {
            use rand::Rng;

            let top: u32 = self.0.iter().map(|(w, _)| w).sum();
            let mut idx = rand::thread_rng().gen_range(0, top);
            for (w, t) in self.0.iter() {
                if idx < *w {
                    return Some(t.to_owned());
                } else {
                    idx -= w
                }
            }
            None
        }
    }

    pub fn weighted(choices: Vec<(u32, Tile)>) -> impl TilePicker {
        Weighted(choices)
    }
}

pub mod generators {
    use super::*;

    pub struct Fill<T: TilePicker>(pub T);

    impl<T: TilePicker> TileGenerator for Fill<T> {
        fn generate(
            &self,
            current: &Grid<Tile>,
            bounds: Bounds,
        ) -> Result<Vec<(Pos, Tile)>, GenError> {
            Ok(bounds
                .0
                .iter_to(bounds.1)
                .filter_map(|pos| self.0.pick(current, pos).and_then(|t| Some((pos, t))))
                .collect())
        }
    }
}
