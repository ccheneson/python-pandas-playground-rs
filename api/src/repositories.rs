pub mod map_repository;

pub trait ApiToCode {
    fn add_code(&mut self, api: String, code: String) -> Result<(), anyhow::Error>;
    fn get_code(&self, api: String) -> Result<Option<&String>, anyhow::Error>;
}
