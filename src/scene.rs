use super::{adventure::*, pos::*, resources::*, *};
use specs::prelude::*;

pub struct AWorld<L: ResourceDataLoader> {
    pub specs_world: specs::World,
    pub adventure:   Adventure<L>,
}

pub struct AdventureScene {
    dispatcher: Dispatcher<'static, 'static>,
}

pub enum SceneChange<L: ResourceDataLoader> {
    None,
    Switch(Box<Scene<L>>),
    Push(Box<Scene<L>>),
    Pop,
    Exit,
}

pub trait Scene<L: ResourceDataLoader> {
    fn setup(&mut self, world: &mut AWorld<L>);
    fn update(&mut self, world: &mut AWorld<L>) -> SceneChange<L>;
}

impl AdventureScene {
    pub fn new() -> AdventureScene {
        AdventureScene {
            dispatcher: DispatcherBuilder::new()
                .with(player::PlayerStateS, "player", &[])
                .with(action::ActiveS, "active", &["player"])
                .with(movement::MovementS, "movement", &["active"])
                .with(fov::FovS, "fov", &["active"])
                .with(energy::EnergyS, "energy", &["active"])
                .with(behavior::HunterBrainS, "hunter_brain", &["energy"])
                .with(ai::AIMoveS, "ai_move", &["hunter_brain"])
                .with(player::PlayerMoveS, "player_move", &["energy"])
                .with_barrier()
                .with(action::TurnS, "turn", &["player_move"])
                .with_thread_local(curses::CursesDisplayS)
                .with_thread_local(events::EventPumpS)
                .build(),
        }
    }
}

impl Default for AdventureScene {
    fn default() -> Self {
        Self::new()
    }
}

fn make_actor(
    world: &mut World,
    map: Entity,
    glyph: char,
    name: &'static str,
    description: &'static str,
    speed: f32,
    pos: Pos,
    extra: fn(EntityBuilder) -> EntityBuilder,
) {
    let builder = world
        .create_entity()
        .with(appearance::Appearance {
            glyph:       appearance::Glyph::new(glyph),
            name:        String::from(name),
            description: String::from(description),
        }).with(energy::Energy::new(speed))
        .with(action::Turn::default())
        .with(map::Location { map, pos })
        .with(fov::FovMap::default())
        .with(movement::MovementMap::default());
    extra(builder).build();
}

fn make_player(world: &mut World, map: Entity) {
    make_actor(
        world,
        map,
        '@',
        "player",
        "A very confused looking being",
        1.0,
        Pos(7, 9),
        |builder| builder.with(player::PlayerBrain),
    );
}

impl<L: ResourceDataLoader> Scene<L> for AdventureScene {
    fn setup(&mut self, aworld: &mut AWorld<L>) {
        let mut world = &mut aworld.specs_world;
        self.dispatcher.setup(&mut world.res);

        let firstmap = aworld.adventure.first_map();
        let map = world.create_entity().with(firstmap).build();

        world.add_resource(events::Events::new());
        world.add_resource(player::GameState::Starting);

        make_player(&mut world, map);
        make_actor(
            &mut world,
            map,
            's',
            "snake",
            "A lazy fat garden snake",
            0.2,
            Pos(1, 1),
            |b| b.with(behavior::HunterBrain::new(1)),
        );
        make_actor(
            &mut world,
            map,
            'c',
            "cat",
            "A playful cat",
            1.1,
            Pos(13, 12),
            |b| b.with(behavior::HunterBrain::new(3)),
        );
    }

    fn update(&mut self, aworld: &mut AWorld<L>) -> SceneChange<L> {
        let world = &mut aworld.specs_world;
        self.dispatcher.dispatch(&world.res);
        world.maintain();

        let ch = curses::CursesDisplayS::getch();
        match ch {
            Some(ch) => match ch {
                'q' => return SceneChange::Exit,
                other => {
                    let mut state = world.write_resource::<player::GameState>();
                    *state = player::GameState::Active(Some(other));
                }
            },
            None => {
                let mut state = world.write_resource::<player::GameState>();
                *state = player::GameState::Idle;
            }
        }

        SceneChange::None
    }
}
