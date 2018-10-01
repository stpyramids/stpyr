use super::{grid::*, map::*, resources::*};

#[derive(Debug)]
pub struct Vault {
    pub tiles: Grid<Tile>,
}

impl ResourceLoader<Vault> for Vault {
    fn load(lines: Vec<String>) -> Result<Vault> {
        let width = lines[0].len() as u32;
        let height = lines.len() as u32;
        let tiles = Grid::load(width, height, &lines.join(""), |c, _| match c {
            '#' => Tile {
                // todo: want to get all curses stuff out of lib
                glyph:  super::curses::Glyph('#'),
                opaque: true,
                solid:  true,
            },
            _ => Tile::default(),
        });
        Ok(Vault { tiles })
    }
}

impl LoadableResource for Vault {
    type Loader = Vault;
}
