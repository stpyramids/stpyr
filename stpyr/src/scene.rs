use super::{adventure::*, display::*, resources::*, *};

pub struct AWorld<L: ResourceDataLoader, D: Display> {
    pub specs_world: specs::World,
    pub display:     D,
    pub adventure:   Adventure<L>,
}

impl<L: ResourceDataLoader, D: Display> AWorld<L, D> {
    pub fn new(adventure: Adventure<L>, display: D) -> AWorld<L, D> {
        AWorld {
            specs_world: specs::World::new(),
            adventure,
            display,
        }
    }
}

pub enum SceneChange<L: ResourceDataLoader, D: Display> {
    None,
    Switch(Box<dyn Scene<L, D>>),
    Push(Box<dyn Scene<L, D>>),
    Pop,
    Exit,
}

pub trait Scene<L: ResourceDataLoader, D: Display> {
    fn setup(&mut self, world: &mut AWorld<L, D>);
    fn update(&mut self, world: &mut AWorld<L, D>) -> SceneChange<L, D>;
}
