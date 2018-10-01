use super::{appearance::Glyph, grid::*, map::*, resources::*};

#[derive(Debug)]
pub struct Vault {
    pub tiles: Grid<Tile>,
}

impl ResourceLoader<Vault> for Vault {
    fn load(data: String) -> Result<Vault> {
        let lines: Vec<String> = data
            .split(char::is_whitespace)
            .map(|s| s.to_owned())
            .collect();
        let width = lines[0].len() as u32;
        let height = lines.len() as u32;
        let tiles = Grid::load(width, height, &lines.join(""), |c, _| match c {
            '#' => Tile {
                glyph:  Glyph::new('#'),
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
