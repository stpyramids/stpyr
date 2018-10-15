use super::pos::*;
use std::{
    ops::{Index, IndexMut},
    slice::Iter,
};

#[derive(Debug)]
pub struct Grid<T> {
    pub width:  u32,
    pub height: u32,
    grid:       Vec<T>,
}

#[derive(Debug, Fail)]
pub enum BlitError {
    #[fail(display = "origin out of bounds")]
    OutOfBounds,
    #[fail(display = "source grid too large for destination")]
    TooLarge,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: u32, height: u32, default: T) -> Self {
        Grid {
            width,
            height,
            grid: vec![default; (width * height) as usize],
        }
    }

    pub fn new_from_vec(width: u32, height: u32, grid: Vec<T>) -> Option<Self> {
        if grid.len() == (width * height) as usize {
            Some(Grid {
                width,
                height,
                grid,
            })
        } else {
            None
        }
    }

    pub fn load(width: u32, height: u32, text: &str, loader: fn(char, Pos) -> T) -> Option<Self> {
        Self::new_from_vec(
            width,
            height,
            text.chars()
                .filter(|c| !c.is_whitespace())
                .enumerate()
                .map(|(idx, glyph)| {
                    let idx = idx as u32;
                    let x = idx % width;
                    let y = idx / width;
                    let pos = Pos(x, y);
                    loader(glyph, pos)
                })
                .collect(),
        )
    }

    pub fn at(&self, pos: Pos) -> &T {
        &self.grid[pos.to_idx(self.width)]
    }

    pub fn set(&mut self, pos: Pos, value: T) {
        self.grid[pos.to_idx(self.width)] = value;
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.grid.iter()
    }

    pub fn idx_to_pos(&self, idx: usize) -> Pos {
        let idx = idx as u32;
        let x = idx % self.width;
        let y = idx / self.width;
        Pos(x, y)
    }

    pub fn blit(&mut self, x: u32, y: u32, other: &Self) -> Result<(), BlitError> {
        if !self.contains(Pos(x, y)) {
            return Err(BlitError::OutOfBounds);
        }
        if !self.contains(Pos(x + other.width, y + other.height)) {
            return Err(BlitError::TooLarge);
        }
        for (idx, entry) in other.grid.iter().enumerate() {
            let pos = other.idx_to_pos(idx);
            self.set(Pos(x + pos.0, y + pos.1), entry.clone());
        }
        Ok(())
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.grid[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.grid[index]
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &T {
        &self.grid[index.to_idx(self.width)]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut T {
        &mut self.grid[index.to_idx(self.width)]
    }
}
