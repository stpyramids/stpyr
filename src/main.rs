extern crate stpyr;

use fern;
#[macro_use]
extern crate log;

use stpyr::{adventure::*, curses::*, resources::*, scene::*};

fn main() {
    setup_logger().unwrap();

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

fn setup_logger() -> std::result::Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
