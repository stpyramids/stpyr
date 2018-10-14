pub use crate::tile_generator::*;
use rand::prelude::*;

pub mod mazes {
    use super::*;

    pub fn recursive_backtracking<T: TilePicker>(picker: T) -> impl TileGenerator {
        super::RecursiveBacktracking(picker)
    }
}

struct RecursiveBacktracking<T: TilePicker>(pub T);

#[derive(Copy, Clone)]
enum Dir {
    N = 1,
    E = 2,
    S = 4,
    W = 8,
}

type RazorMaze = Grid<u8>;

#[inline(always)]
fn d(dir: Dir) -> PosDiff {
    match dir {
        Dir::N => PosDiff(0, -1),
        Dir::E => PosDiff(1, 0),
        Dir::S => PosDiff(0, 1),
        Dir::W => PosDiff(-1, 0),
    }
}

#[inline(always)]
fn opp(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::S,
        Dir::E => Dir::W,
        Dir::S => Dir::N,
        Dir::W => Dir::E,
    }
}

fn carve_passages_from<R: Rng + ?Sized>(c: Pos, maze: &mut RazorMaze, rng: &mut R) {
    let mut directions = [Dir::N, Dir::E, Dir::S, Dir::W];
    rng.shuffle(&mut directions);

    for dir in directions.into_iter() {
        let n = c + d(*dir);
        if maze.contains(n) && maze[n] == 0 {
            maze[c] |= *dir as u8;
            maze[n] |= opp(*dir) as u8;
            carve_passages_from(n, maze, rng);
        }
    }
}

impl<T: TilePicker> TileGenerator for RecursiveBacktracking<T> {
    fn generate(&self, current: &Grid<Tile>, bounds: Bounds) -> Result<Vec<(Pos, Tile)>, GenError> {
        let (start, end) = bounds;
        let (width, height) = ((end.0 - start.0), (end.1 - start.1));
        if width < 6 || height < 6 {
            // Too small for even 2x2
            return Err(GenError::TooSmall);
        }
        let mut rng = rand::thread_rng();
        let mut maze: RazorMaze = Grid::new(width / 3 + 1, height / 3 + 1, 0);

        carve_passages_from(Pos(0, 0), &mut maze, &mut rng);

        Ok(start
            .iter_to(end)
            .filter_map(|pos| {
                self.0.pick(current, pos).and_then(|t| {
                    let raz = maze[Pos(pos.0 / 3, pos.1 / 3)];
                    match (pos.0 % 3, pos.1 % 3) {
                        (1, 1) => None,
                        (1, 0) if (raz & Dir::N as u8) > 0 => None,
                        (1, 2) if (raz & Dir::S as u8) > 0 => None,
                        (0, 1) if (raz & Dir::W as u8) > 0 => None,
                        (2, 1) if (raz & Dir::E as u8) > 0 => None,
                        _ => Some((pos, t)),
                    }
                })
            })
            .collect())
    }
}
