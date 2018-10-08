extern crate stpyrl;

use stpyrl::{adventure::*, curses::*, resources::*, scene::*};

fn main() {
    let mut world = AWorld::new(
        Adventure::new(FileResourceDataLoader::new("res")),
        CursesDisplay::init(),
    );
    let mut scene = AdventureScene::new();
    scene.setup(&mut world);

    loop {
        match scene.update(&mut world) {
            SceneChange::Exit => break,
            _ => (),
        }
    }
}
