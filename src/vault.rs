use super::{appearance::Glyph, grid::*, map::*, pos::*, resources::*};

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

impl MapGenerator for Vault {
    fn generate(
        &self,
        _current: &Grid<Tile>,
        start: Pos,
        end: Pos,
    ) -> Option<Vec<(Pos, Option<Tile>)>> {
        let bounds = (start, end);
        Some(
            self.tiles
                .iter()
                .enumerate()
                .map(|(idx, entry)| {
                    let pos = start + self.tiles.idx_to_pos(idx);
                    (pos, Some(entry.to_owned()))
                }).filter(|(pos, _)| pos.within(bounds))
                .collect(),
        )
    }
}

impl LoadableResource for Vault {
    type Loader = Vault;
}
