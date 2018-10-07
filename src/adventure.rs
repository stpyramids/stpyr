use super::{def::*, map::*, resources::*, vault::*};

pub struct Adventure<L: ResourceDataLoader> {
    loader:   L,
    bestiary: Bestiary,
}

impl<L: ResourceDataLoader> Adventure<L> {
    pub fn new(loader: L) -> Self {
        let bestiary = Codex::load(&loader).unwrap();
        Adventure { loader, bestiary }
    }

    pub fn first_map(&self) -> TileMap {
        let vault: Vault = self.loader.load("room.vault").expect("couldn't load vault");
        let mut firstmap = TileMap::new(40, 20);
        firstmap.place_vault(&vault).expect("couldn't place vault");
        firstmap
    }

    pub fn actor(&self, id: String) -> Option<ActorDef> {
        self.bestiary.get(id)
    }
}
