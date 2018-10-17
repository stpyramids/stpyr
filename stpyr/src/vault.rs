use super::{map::*, resources::*, tile_generator::*};
use failure::*;

#[derive(Debug)]
pub struct Vault {
    pub tiles: Grid<Tile>,
}

pub struct VaultLoader(pub fn(char, Pos) -> Tile);

impl ResourceLoader<Vault> for VaultLoader {
    fn load(&self, data: String) -> Result<Vault> {
        let lines: Vec<String> = data
            .split(char::is_whitespace)
            .map(|s| s.to_owned())
            .collect();
        let width = lines[0].len() as u32;
        let height = lines.len() as u32;
        let tiles = Grid::load(width, height, &lines.join(""), self.0)
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
    type Loader = VaultLoader;
}
