extern crate stpyrl;

use stpyrl::{adventure::*, curses::*, resources::*, scene::*};

fn main() {
    let display = CursesDisplay::init();

    let loader = FileResourceDataLoader::new("res");
    let adventure = Adventure::new(loader);
    let mut world = AWorld {
        specs_world: specs::World::new(),
        adventure,
        display,
    };
    let mut scene = AdventureScene::new();
    scene.setup(&mut world);

    loop {
        match scene.update(&mut world) {
            SceneChange::Exit => break,
            _ => (),
        }
    }
}
