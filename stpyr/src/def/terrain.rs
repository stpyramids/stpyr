use failure::Error;
use crate::resources::*;

#[derive(Deserialize, Clone)]
pub struct TerrainDef {
    pub id: String,
    pub glyph: char,
    #[serde(default)]
    pub opaque: bool,
    #[serde(default)]
    pub solid: bool,
}

#[derive(Deserialize, Clone)]
pub struct Terrain {
    terrain: Vec<TerrainDef>,
}

impl Terrain {
    pub fn get(&self, id: &str) -> Option<TerrainDef> {
        self.terrain
            .iter()
            .find(|e| e.id == *id)
            .map(|a| a.to_owned())
    }
}

pub struct TerrainLoader;
impl ResourceLoader<Terrain> for TerrainLoader {
    fn load(&self, data: String) -> Result<Terrain, Error> {
        Ok(toml::from_str(&data)?)
    }
}

impl LoadableResource for Terrain {
    type Loader = TerrainLoader;
}
