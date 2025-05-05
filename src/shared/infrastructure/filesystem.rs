use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::logger_port::LoggerPort;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy)]
pub struct Filesystem<'a> {
    logger: &'a dyn LoggerPort,
}

impl<'a> Filesystem<'a> {
    pub fn new(logger: &'a dyn LoggerPort) -> Self {
        Self { logger }
    }
}

impl<'a> FilesystemPort for Filesystem<'a> {
    fn read_file(&self, path: &Path) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    fn write_file(&self, path: &Path, content: &str) -> Result<(), String> {
        if let Some(parent_path) = path.parent() {
            if !parent_path.exists() {
                self.create_dir_all(parent_path)?;
            }
        }
        if path.is_file() {
            self.logger.warn("File already exists. Overwriting it.");
            // TODO: Implement a confirmation prompt asking the user if they wish to continue.
        }
        fs::write(path, content).map_err(|e| e.to_string())
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|e| e.to_string())
    }
}
