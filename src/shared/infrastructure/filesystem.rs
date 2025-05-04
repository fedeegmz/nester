use std::fs;

#[derive(Clone, Copy)]
pub struct Filesystem;

impl Filesystem {
    pub fn read_to_string(&self, path: &str) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    pub fn write_string(&self, path: &str, contents: &str) -> Result<(), String> {
        fs::write(path, contents).map_err(|e| e.to_string())
    }

    pub fn create_dir_all(&self, path: &str) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|e| e.to_string())
    }
}
