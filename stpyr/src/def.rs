use super::resources::*;

pub trait Definition {
    fn mint(self, builder: specs::EntityBuilder<'_>) -> specs::EntityBuilder<'_>;
}

pub trait Codex<D: Definition>: LoadableResource {
    type ID;

    fn load<L: ResourceDataLoader>(loader: &L) -> Option<Self>;
    fn get(&self, id: Self::ID) -> Option<D>;
}

#[derive(Deserialize, Clone)]
pub struct ActorDef {
    pub id:          String,
    pub glyph:       char,
    pub name:        String,
    pub description: String,
    pub speed:       f32,
}

impl Definition for ActorDef {
    fn mint(self, builder: specs::EntityBuilder<'_>) -> specs::EntityBuilder<'_> {
        use specs::Builder;

        builder
            .with(super::appearance::Appearance {
                glyph:       super::appearance::Glyph::new(self.glyph),
                name:        self.name,
                description: self.description,
            })
            .with(super::energy::Energy::new(self.speed))
            .with(super::action::Turn::default())
            .with(super::fov::FovMap::default())
            .with(super::movement::MovementMap::default())
    }
}

#[derive(Deserialize)]
pub struct Bestiary {
    bestiary: Vec<ActorDef>,
}

pub struct BestiaryLoader;

impl ResourceLoader<Bestiary> for BestiaryLoader {
    fn load(&self, data: String) -> Result<Bestiary> {
        Ok(toml::from_str(&data).unwrap())
    }
}

impl LoadableResource for Bestiary {
    type Loader = BestiaryLoader;
}

impl Codex<ActorDef> for Bestiary {
    type ID = String;

    fn load<L: ResourceDataLoader>(loader: &L) -> Option<Self> {
        loader.load("bestiary.toml", BestiaryLoader).ok()
    }

    fn get(&self, id: Self::ID) -> Option<ActorDef> {
        let it = self.bestiary.iter().find(|e| e.id == id)?;
        Some(it.clone())
    }
}

#[derive(Deserialize, Clone)]
pub struct TerrainDef {
    pub id: String,
    pub glyph: char,
    #[serde(default)]
    pub opaque: bool,
    #[serde(default)]
    pub solid: bool,
}

#[derive(Deserialize)]
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
    fn load(&self, data: String) -> Result<Terrain> {
        Ok(toml::from_str(&data).unwrap())
    }
}

impl LoadableResource for Terrain {
    type Loader = TerrainLoader;
}
