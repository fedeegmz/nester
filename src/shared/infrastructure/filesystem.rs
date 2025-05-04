use crate::core::port::filesystem_port::FilesystemPort;
use std::fs;

#[derive(Clone, Copy)]
pub struct Filesystem;

impl FilesystemPort for Filesystem {
    fn read_to_string(&self, path: &str) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    fn write_string(&self, path: &str, content: &str) -> Result<(), String> {
        fs::write(path, content).map_err(|e| e.to_string())
    }

    fn create_dir_all(&self, path: &str) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|e| e.to_string())
    }
}
