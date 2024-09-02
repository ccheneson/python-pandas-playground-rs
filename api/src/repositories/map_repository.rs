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
    fn add_code(&mut self, api: String, code: String) -> Result<(), anyhow::Error> {
        self.repository.insert(api, code);
        Ok(())
    }

    fn get_code(&self, api: String) -> Result<Option<&String>, anyhow::Error> {
        Ok(self.repository.get(&api))
    }
}
