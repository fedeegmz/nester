pub trait FilesystemPort {
    fn read_to_string(&self, path: &str) -> Result<String, String>;

    fn write_string(&self, path: &str, content: &str) -> Result<(), String>;

    fn create_dir_all(&self, path: &str) -> Result<(), String>;
}
