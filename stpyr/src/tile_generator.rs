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

    #[derive(Clone)]
    pub enum Picker {
        None,
        Only(Tile),
        Weighted(Vec<(u32, Picker)>),
        Tiled(Grid<Picker>),
        Dig(Box<Picker>),
        Build(Box<Picker>),
        Mask(Box<Picker>, fn(&Tile) -> bool),
    }

    impl std::fmt::Debug for Picker {
        fn fmt(
            &self,
            formatter: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            write!(formatter, "foo")
        }
    }

    impl TilePicker for Picker {
        fn pick(&self, current: &Grid<Tile>, pos: Pos) -> Option<Tile> {
            match self {
                Picker::None => None,
                Picker::Only(tile) => Some(tile.to_owned()),
                Picker::Weighted(choices) => {
                    use rand::Rng;
                    let top: u32 = choices.iter().map(|(w, _)| w).sum();
                    let mut idx = rand::thread_rng().gen_range(0, top);
                    for (w, t) in choices.iter() {
                        if idx < *w {
                            return t.pick(current, pos);
                        } else {
                            idx -= w
                        }
                    }
                    None
                }
                Picker::Tiled(pattern) => pattern
                    .at(Pos(pos.0 % pattern.width, pos.1 % pattern.height))
                    .pick(current, pos),
                Picker::Dig(box picker) => {
                    if current.at(pos).solid {
                        picker.pick(current, pos)
                    } else {
                        None
                    }
                }
                Picker::Build(box picker) => {
                    if current.at(pos).solid {
                        None
                    } else {
                        picker.pick(current, pos)
                    }
                }
                Picker::Mask(box picker, test) => {
                    if test(&current.at(pos)) {
                        picker.pick(current, pos)
                    } else {
                        None
                    }
                }
            }
        }
    }

    pub fn none() -> Picker {
        Picker::None
    }

    pub fn only(tile: Tile) -> Picker {
        Picker::Only(tile)
    }

    pub fn weighted(choices: Vec<(u32, Picker)>) -> Picker {
        Picker::Weighted(choices)
    }

    pub fn tiled(pattern: Grid<Picker>) -> Picker {
        Picker::Tiled(pattern)
    }

    pub fn dig(picker: Picker) -> Picker {
        Picker::Dig(box picker)
    }

    pub fn build(picker: Picker) -> Picker {
        Picker::Build(box picker)
    }

    pub fn mask(picker: Picker, test: fn(&Tile) -> bool) -> Picker {
        Picker::Mask(box picker, test)
    }
}

// pub mod opickers {
// use super::*;
// use std::any::Any;
//
// #[derive(Debug, Clone)]
// struct NoTile;
//
// impl TilePicker for NoTile {
// fn pick(&self, _current: &Grid<Tile>, _pos: Pos) -> Option<Tile> {
// None
// }
// }
//
// pub fn none() -> impl TilePicker + Clone {
// NoTile
// }
//
// impl TilePicker for Tile {
// fn pick(&self, _current: &Grid<Tile>, _pos: Pos) -> Option<Tile> {
// Some(self.to_owned())
// }
// }
//
// pub fn only(tile: Tile) -> impl TilePicker + Clone {
// tile
// }
//
// #[derive(Debug, Clone)]
// struct Weighted<T: Any + TilePicker + Clone>(Vec<(u32, Box<T>)>);
//
// impl<T: Any + TilePicker + Clone> TilePicker for Weighted<T> {
// fn pick(&self, current: &Grid<Tile>, pos: Pos) -> Option<Tile> {
// use rand::Rng;
// let top: u32 = self.0.iter().map(|(w, _)| w).sum();
// let mut idx = rand::thread_rng().gen_range(0, top);
// for (w, t) in self.0.iter() {
// if idx < *w {
// return t.pick(current, pos);
// } else {
// idx -= w
// }
// }
// None
// }
// }
//
// pub fn weighted<T: Any + TilePicker + Clone>(
// choices: Vec<(u32, Box<T>)>,
// ) -> impl TilePicker + Clone {
// Weighted(choices)
// }
//
// #[derive(Debug, Clone)]
// struct Tiled<T: TilePicker + Clone>(Grid<T>);
//
// impl<T: TilePicker + Clone> TilePicker for Tiled<T> {
// fn pick(&self, current: &Grid<Tile>, pos: Pos) -> Option<Tile> {
// self.0
// .at(Pos(pos.0 % self.0.width, pos.1 % self.0.height))
// .pick(current, pos)
// }
// }
//
// pub fn tiled<T: TilePicker + Clone>(pattern: Grid<T>) -> impl TilePicker {
// Tiled(pattern)
// }
// }

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

    #[derive(Debug)]
    struct RectRoom<W: TilePicker, F: TilePicker> {
        wall: W,
        fill: F,
    }

    impl<W: TilePicker, F: TilePicker> TileGenerator for RectRoom<W, F> {
        fn generate(
            &self,
            current: &Grid<Tile>,
            bounds: Bounds,
        ) -> Result<Vec<(Pos, Tile)>, GenError> {
            let (Pos(sx, sy), Pos(ex, ey)) = bounds;
            Ok(bounds
                .0
                .iter_to(bounds.1)
                .filter_map(|pos| {
                    if pos.0 == sx || pos.1 == sy || pos.0 == ex || pos.1 == ey {
                        self.wall.pick(current, pos)
                    } else {
                        self.fill.pick(current, pos)
                    }
                    .and_then(|t| Some((pos, t)))
                })
                .collect())
        }
    }

    pub fn rect<W: TilePicker, F: TilePicker>(wall: W, fill: F) -> impl TileGenerator {
        RectRoom { wall, fill }
    }

}
