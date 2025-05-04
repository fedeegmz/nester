use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::templates_port::TemplatesPort;
use std::path::Path;

pub struct GenerateHandler<'a> {
    config: Config,
    fs: &'a dyn FilesystemPort,
    templates: &'a dyn TemplatesPort,
}

impl<'a> GenerateHandler<'a> {
    pub fn new(
        config: Config,
        fs: &'a dyn FilesystemPort,
        templates: &'a dyn TemplatesPort,
    ) -> Self {
        GenerateHandler {
            config,
            fs,
            templates,
        }
    }

    pub fn handle(
        &self,
        path: &Path,
        name: Option<String>,
        pkg: Option<String>,
    ) -> Result<(), String> {
        let file_name_os_str = path
            .file_name()
            .ok_or_else(|| format!("Could not extract filename from path: {}", path.display()))?;

        let file_name = file_name_os_str
            .to_str()
            .ok_or_else(|| format!("Filename contains invalid UTF-8: {:?}", file_name_os_str))?;

        let file_config = self
            .config
            .files
            .iter()
            .find(|f| f.name == file_name)
            .ok_or_else(|| format!("Configuration for file '{}' not found", file_name))?;

        let content = self.templates.load(&file_config.template, name, pkg);
        self.fs.write_file(path, &content)?;

        Ok(())
    }
}
