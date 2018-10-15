use super::{appearance::Glyph, map::*, resources::*, tile_generator::*};
use failure::*;

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
            '%' => Tile {
                glyph:  Glyph::new('%'),
                opaque: true,
                solid:  false,
            },
            _ => Tile::default(),
        })
        .ok_or_else(|| err_msg("vault wrong length"))?;
        Ok(Vault { tiles })
    }
}

impl TileGenerator for Vault {
    fn generate(
        &self,
        _current: &Grid<Tile>,
        bounds: Bounds,
    ) -> std::result::Result<Vec<(Pos, Tile)>, GenError> {
        Ok(self
            .tiles
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let pos = bounds.0 + self.tiles.idx_to_pos(idx);
                (pos, entry.to_owned())
            })
            .filter(|(pos, _)| pos.within(bounds))
            .collect())
    }
}

impl LoadableResource for Vault {
    type Loader = Vault;
}
