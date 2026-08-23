pub mod loader;
pub mod model;
mod text;

use std::collections::HashMap;

use model::Passage;

pub struct Story {
    passages: Vec<Passage>,
    by_id: HashMap<String, usize>,
}

impl Story {
    pub fn new(passages: Vec<Passage>) -> Self {
        let by_id = passages
            .iter()
            .enumerate()
            .map(|(index, passage)| (passage.id.clone(), index))
            .collect();

        Self { passages, by_id }
    }

    /// A miss here is a bug in the data that should have failed at build time.
    pub fn get(&self, id: &str) -> &Passage {
        let index = self.by_id[id];
        &self.passages[index]
    }

    pub fn start(&self) -> &Passage {
        self.passages
            .iter()
            .find(|passage| passage.act == 1 && passage.location == "start")
            .expect("no starting passage found")
    }
}