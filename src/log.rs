use specs::prelude::*;

pub struct DebugLog {
    pub messages: Vec<String>,
}

impl DebugLog {
    pub fn new() -> DebugLog {
        DebugLog { messages: vec![] }
    }

    pub fn log(&mut self, message: String) {
        self.messages.push(message);
    }
}

impl Default for DebugLog {
    fn default() -> Self {
        return DebugLog::new();
    }
}
