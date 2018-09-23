use specs::prelude::*;

#[derive(Clone, Debug)]
pub enum Event {
    MoveFailed(Entity),
    TargetReached(Entity),
    HunterHunts(Entity),
}

#[derive(Default)]
pub struct Events {
    pub events: Vec<Event>,
    next_events: Vec<Event>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            events: vec![],
            next_events: vec![],
        }
    }

    pub fn pump(&self) -> Self {
        Events {
            events: self.next_events.to_vec(),
            next_events: vec![],
        }
    }

    pub fn push(&mut self, e: Event) {
        self.next_events.push(e);
    }
}

pub struct EventPumpS;
impl<'a> System<'a> for EventPumpS {
    type SystemData = Write<'a, Events>;

    fn run(&mut self, data: Self::SystemData) {
        let mut events = data;
        *events = events.pump();
    }
}
