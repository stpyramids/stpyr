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
            }).with(super::energy::Energy::new(self.speed))
            .with(super::action::Turn::default())
            .with(super::fov::FovMap::default())
            .with(super::movement::MovementMap::default())
    }
}

#[derive(Deserialize)]
pub struct Bestiary {
    bestiary: Vec<ActorDef>,
}

impl ResourceLoader<Bestiary> for Bestiary {
    fn load(data: String) -> Result<Self> {
        Ok(toml::from_str(&data).unwrap())
    }
}

impl LoadableResource for Bestiary {
    type Loader = Bestiary;
}

impl Codex<ActorDef> for Bestiary {
    type ID = String;

    fn load<L: ResourceDataLoader>(loader: &L) -> Option<Self> {
        loader.load("bestiary.toml").ok()
    }

    fn get(&self, id: Self::ID) -> Option<ActorDef> {
        let it = self.bestiary.iter().find(|e| e.id == id)?;
        Some(it.clone())
    }
}
