use crate::core::port::filesystem_port::FilesystemPort;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy)]
pub struct Filesystem;

impl Filesystem {
    pub fn new() -> Self {
        Self
    }
}

impl FilesystemPort for Filesystem {
    fn read_file(&self, path: &Path) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    fn write_file(&self, path: &Path, content: &str) -> Result<(), String> {
        if let Some(parent_path) = path.parent() {
            if !parent_path.exists() {
                self.create_dir_all(parent_path)?;
            }
        }

        fs::write(path, content).map_err(|e| e.to_string())
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|e| e.to_string())
    }
}
