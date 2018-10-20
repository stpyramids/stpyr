pub use super::map::Tile;
use super::tile_generator::*;

#[derive(Debug)]
pub struct Vault {
    pub tiles: Grid<Tile>,
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
