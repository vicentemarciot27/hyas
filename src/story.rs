pub mod loader;
pub mod model;
mod text;

use std::collections::HashMap;

use model::Passage;

pub struct Story {
    passages: Vec<Passage>,
    maps: Vec<model::MapEntry>,
    passages_by_id: HashMap<String, usize>,
    maps_by_id: HashMap<String, usize>,
}

impl Story {
    pub fn new(passages: Vec<Passage>, maps: Vec<model::MapEntry>) -> Self {
        let passages_by_id = passages
            .iter()
            .enumerate()
            .map(|(index, passage)| (passage.id.clone(), index))
            .collect();

        let maps_by_id = maps
            .iter()
            .enumerate()
            .map(|(index, map)| (map.id.clone(), index))
            .collect();
        Self { passages, maps, passages_by_id, maps_by_id }
    }

    /// A miss here is a bug in the data that should have failed at build time.
    pub fn get(&self, id: &str) -> &Passage {
        let index = self.passages_by_id[id];
        &self.passages[index]
    }

    pub fn start(&self) -> &Passage {
        self.passages
            .iter()
            .find(|passage| passage.act == 1 && passage.location == "start")
            .expect("no starting passage found")
    }
}