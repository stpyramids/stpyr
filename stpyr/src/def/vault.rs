use super::terrain::*;
use crate::{grid::*, resources::*, vault::*};
use failure::*;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct VaultDef {
    pub id:     String,
    pub grid:   String,
    pub glyphs: HashMap<char, String>,
}

#[derive(Deserialize)]
pub struct Vaults {
    vaults: Vec<VaultDef>,
}

impl Vaults {
    pub fn get(&self, id: &str) -> Option<VaultDef> {
        self.vaults
            .iter()
            .find(|e| e.id == *id)
            .map(|a| a.to_owned())
    }

    pub fn build(&self, id: &str, terrain: &Terrain) -> Result<Vault, Error> {
        let def = self
            .get(id)
            .ok_or_else(|| format_err!("vault not found: {}", id))?;
        let lines: Vec<String> = def
            .grid
            .split(char::is_whitespace)
            .map(|s| s.to_owned())
            .collect();
        let width = lines[0].len() as u32;
        let height = lines.len() as u32;
        let mut tilemap: HashMap<char, Tile> = HashMap::new();
        for (glyph, t_id) in def.glyphs.iter() {
            let t = terrain
                .get(t_id)
                .ok_or_else(|| format_err!("specified terrain not found: {}", t_id))?;
            tilemap.insert(*glyph, t.into());
        }
        let tiles = Grid::load(width, height, &lines.join(""), |glyph, _| {
            tilemap.get(&glyph).map(|t| t.to_owned())
        })
        .ok_or_else(|| err_msg("vault wrong length"))?;

        Ok(Vault { tiles })
    }
}

pub struct VaultsLoader;
impl ResourceLoader<Vaults> for VaultsLoader {
    fn load(&self, data: String) -> Result<Vaults, Error> {
        Ok(toml::from_str(&data).unwrap())
    }
}

impl LoadableResource for Vaults {
    type Loader = VaultsLoader;
}
