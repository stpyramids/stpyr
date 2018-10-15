use super::{appearance::*, def::*, labyrinth::*, map::*, pos::*, resources::*, vault::*};

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
        let dirt = Tile {
            glyph:  Glyph::new('.'),
            opaque: false,
            solid:  false,
        };
        let wall = Tile {
            glyph:  Glyph::new('#'),
            opaque: true,
            solid:  true,
        };
        let grass = Tile {
            glyph:  Glyph::new(','),
            opaque: true,
            solid:  false,
        };
        let rubble = Tile {
            glyph:  Glyph::new('%'),
            opaque: false,
            solid:  false,
        };
        let mut rows: Grid<Tile> = Grid::new(2, 2, grass);
        rows.set(Pos(0, 0), dirt.to_owned());
        rows.set(Pos(1, 0), dirt.to_owned());

        firstmap
            .place(
                Pos(0, 0),
                Pos(19, 19),
                &generators::fill(pickers::tiled(rows)),
            )
            .expect("couldn't fill grass");
        firstmap
            .place_vault(Pos(5, 5), &vault)
            .expect("couldn't place vault");
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
