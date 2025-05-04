use std::path::Path;

pub trait FilesystemPort {
    fn read_file(&self, path: &Path) -> Result<String, String>;

    fn write_file(&self, path: &Path, content: &str) -> Result<(), String>;

    fn create_dir_all(&self, path: &Path) -> Result<(), String>;
}
