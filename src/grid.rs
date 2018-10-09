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
    T: Clone + Default,
{
    pub fn new(width: u32, height: u32, default: T) -> Self {
        Grid {
            width,
            height,
            grid: vec![default; (width * height + 1) as usize],
        }
    }

    pub fn load(width: u32, height: u32, text: &str, loader: fn(char, Pos) -> T) -> Self {
        let mut grid = Self::new(width, height, T::default());
        for (idx, glyph) in text.chars().filter(|c| !c.is_whitespace()).enumerate() {
            let pos = grid.idx_to_pos(idx);
            grid.set(pos, loader(glyph, pos))
        }
        grid
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

    pub fn iter(&self) -> Iter<T> {
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
