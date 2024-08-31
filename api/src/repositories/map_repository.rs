use std::collections::HashMap;

use super::ApiToCode;

pub struct MapRepository {
    pub repository: HashMap<String, String>,
}

impl MapRepository {
    pub fn new(repository: HashMap<String, String>) -> Self {
        Self { repository }
    }
}

impl ApiToCode for MapRepository {
    fn add_code(&mut self, api: String, code: String) {
        self.repository.insert(api, code);
    }

    fn get_code(&self, api: String) -> Option<&String> {
        self.repository.get(&api)
    }
}
