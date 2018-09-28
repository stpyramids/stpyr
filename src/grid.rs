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
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a T { &self.grid[index] }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T { &mut self.grid[index] }
}
