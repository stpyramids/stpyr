extern crate stpyrl;

use stpyrl::{scene::*, *};

fn main() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        curses::CursesDisplayS::finish();
        default_panic(panic);
    }));

    run_game();
    curses::CursesDisplayS::finish();
}

fn run_game() {
    curses::CursesDisplayS::init();

    let loader = resources::FileResourceDataLoader::new("res");
    let adventure = adventure::Adventure::new(loader);
    let mut world = scene::AWorld {
        specs_world: specs::World::new(),
        adventure,
    };
    let mut scene = scene::AdventureScene::new();
    scene.setup(&mut world);

    loop {
        match scene.update(&mut world) {
            SceneChange::Exit => break,
            _ => (),
        }
    }
}
