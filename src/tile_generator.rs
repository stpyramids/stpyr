pub use super::{grid::*, map::Tile, pos::*};
use failure::Fail;

#[derive(Debug, Fail)]
pub enum GenError {
    #[fail(display = "destination bounds too small")]
    TooSmall,
}

pub trait TilePicker: std::fmt::Debug {
    fn pick(&self, current: &Grid<Tile>, pos: Pos) -> Option<Tile>;
}

pub trait TileGenerator: std::fmt::Debug {
    fn generate(&self, current: &Grid<Tile>, bounds: Bounds) -> Result<Vec<(Pos, Tile)>, GenError>;
}

pub mod pickers {
    use super::*;

    impl TilePicker for Tile {
        fn pick(&self, _current: &Grid<Tile>, _pos: Pos) -> Option<Tile> {
            Some(self.to_owned())
        }
    }

    pub fn only(tile: Tile) -> impl TilePicker {
        tile
    }

    #[derive(Debug)]
    struct Weighted<T: TilePicker>(Vec<(u32, T)>);

    impl<T: TilePicker> TilePicker for Weighted<T> {
        fn pick(&self, current: &Grid<Tile>, pos: Pos) -> Option<Tile> {
            use rand::Rng;

            let top: u32 = self.0.iter().map(|(w, _)| w).sum();
            let mut idx = rand::thread_rng().gen_range(0, top);
            for (w, t) in self.0.iter() {
                if idx < *w {
                    return t.pick(current, pos);
                } else {
                    idx -= w
                }
            }
            None
        }
    }

    pub fn weighted<T: TilePicker>(choices: Vec<(u32, T)>) -> impl TilePicker {
        Weighted(choices)
    }

    #[derive(Debug)]
    struct Tiled<T: TilePicker + Clone>(Grid<T>);

    impl<T: TilePicker + Clone> TilePicker for Tiled<T> {
        fn pick(&self, current: &Grid<Tile>, pos: Pos) -> Option<Tile> {
            self.0
                .at(Pos(pos.0 % self.0.width, pos.1 % self.0.height))
                .pick(current, pos)
        }
    }

    pub fn tiled(pattern: Grid<Tile>) -> impl TilePicker {
        Tiled(pattern)
    }
}

pub mod generators {
    use super::*;

    #[derive(Debug)]
    struct Fill<T: TilePicker>(pub T);

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

    pub fn fill<T: TilePicker>(picker: T) -> impl TileGenerator {
        Fill(picker)
    }
}
