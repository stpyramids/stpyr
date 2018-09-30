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

#[derive(Debug)]
pub enum BlitError {
    OutOfBounds,
    TooLarge,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: u32, height: u32, default: T) -> Grid<T> {
        Grid {
            width,
            height,
            grid: vec![default; (width * height + 1) as usize],
        }
    }

    pub fn at(&self, pos: Pos) -> &T { &self.grid[pos.to_idx(self.width)] }

    pub fn set(&mut self, pos: Pos, value: T) { self.grid[pos.to_idx(self.width)] = value; }

    pub fn contains(&self, pos: Pos) -> bool { pos.0 < self.width && pos.1 < self.height }

    pub fn iter(&self) -> Iter<T> { self.grid.iter() }

    fn pos_to_idx(&self, pos: Pos) -> usize { pos.to_idx(self.width) }

    fn idx_to_pos(&self, idx: usize) -> Pos {
        let idx = idx as u32;
        let x = idx % self.width;
        let y = idx / self.width;
        Pos(x, y)
    }

    pub fn blit(&mut self, x: u32, y: u32, other: &Self) -> Result<(), BlitError> {
        if !self.contains(Pos(x, y)) {
            return Result::Err(BlitError::OutOfBounds);
        }
        if !self.contains(Pos(x + other.width, y + other.height)) {
            return Result::Err(BlitError::TooLarge);
        }
        for (idx, entry) in other.grid.iter().enumerate() {
            let pos = other.idx_to_pos(idx);
            self.set(Pos(x + pos.0, y + pos.1), entry.clone());
        }
        Result::Ok(())
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a T { &self.grid[index] }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T { &mut self.grid[index] }
}
