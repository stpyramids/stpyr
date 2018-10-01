use specs::prelude::*;

#[derive(Debug, Clone)]
pub struct Glyph {
    ascii: char,
}

impl Glyph {
    pub fn new(ascii: char) -> Glyph { Glyph { ascii } }

    pub fn ascii(&self) -> char { self.ascii }
}

#[derive(Debug, Component, Clone)]
pub struct Appearance {
    pub name:        String,
    pub description: String,
    pub glyph:       Glyph,
}
