pub mod map_repository;

pub trait ApiToCode {
    fn add_code(&mut self, api: String, code: String);
    fn get_code(&self, api: String) -> Option<&String>;
}
