use super::{appearance::*, def::*, labyrinth::*, map::*, pos::*, resources::*, vault::*};

pub struct Adventure<L: ResourceDataLoader> {
    loader:   L,
    bestiary: Bestiary,
    terrain:  Terrain,
}

impl<L: ResourceDataLoader> Adventure<L> {
    pub fn new(loader: L) -> Self {
        let bestiary = Codex::load(&loader).unwrap();
        let terrain = loader.load("terrain.toml", TerrainLoader).unwrap();
        Adventure {
            loader,
            bestiary,
            terrain,
        }
    }

    pub fn first_map(&self) -> TileMap {
        let gazebo: Vault = self
            .loader
            .load(
                "gazebo.vault",
                VaultLoader(|c, _| match c {
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
                }),
            )
            .expect("couldn't load vault");
        let mut firstmap = TileMap::new(40, 20);
        let dirt: Tile = self.terrain.get("dirt floor").unwrap().into();
        let wall: Tile = self.terrain.get("brick wall").unwrap().into();
        let tgrass: Tile = self.terrain.get("tall grass").unwrap().into();
        let grass: Tile = self.terrain.get("grass").unwrap().into();
        let rubble: Tile = self.terrain.get("crumbled bricks").unwrap().into();
        let mut rows = Grid::new(
            2,
            2,
            pickers::weighted(vec![(1, tgrass.to_owned()), (3, grass.to_owned())]),
        );
        rows.set(Pos(0, 0), pickers::weighted(vec![(1, dirt.to_owned())]));
        rows.set(Pos(1, 0), pickers::weighted(vec![(1, dirt.to_owned())]));

        firstmap
            .place(
                Pos(0, 0),
                Pos(19, 19),
                &generators::fill(pickers::tiled(rows)),
            )
            .expect("couldn't fill grass");
        firstmap
            .place_vault(Pos(5, 5), &gazebo)
            .expect("couldn't place gazebo");
        firstmap
            .place(
                Pos(20, 0),
                Pos(39, 19),
                &mazes::recursive_backtracking(pickers::weighted(vec![(5, wall), (1, rubble)])),
            )
            .expect("couldn't make maze");

        firstmap
    }

    pub fn actor(&self, id: String) -> Option<ActorDef> {
        self.bestiary.get(id)
    }
}
