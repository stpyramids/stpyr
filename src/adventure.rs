use super::{map::*, resources::*, vault::*};

pub struct Adventure<L: ResourceDataLoader> {
    loader: L,
}

impl<L: ResourceDataLoader> Adventure<L> {
    pub fn new(loader: L) -> Self { Adventure { loader } }

    pub fn first_map(&self) -> TileMap {
        let vault: Vault = self.loader.load("room.vault").expect("couldn't load vault");
        let mut firstmap = TileMap::new(40, 20);
        firstmap.place_vault(&vault).expect("couldn't place vault");
        firstmap
    }
}
