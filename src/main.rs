extern crate stpyrl;

use stpyrl::{adventure::*, curses::*, resources::*, scene::*};

fn main() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        CursesDisplayS::finish();
        default_panic(panic);
    }));

    run_game();
    CursesDisplayS::finish();
}

fn run_game() {
    CursesDisplayS::init();

    let loader = FileResourceDataLoader::new("res");
    let adventure = Adventure::new(loader);
    let mut world = AWorld {
        specs_world: specs::World::new(),
        adventure,
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
