use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::logger_port::LoggerPort;
use crate::core::port::templates_port::TemplatesPort;
use std::path::Path;

pub struct GenerateHandler<'a> {
    config: Config,
    fs: &'a dyn FilesystemPort,
    templates: &'a dyn TemplatesPort,
    logger: &'a dyn LoggerPort,
}

impl<'a> GenerateHandler<'a> {
    pub fn new(
        config: Config,
        fs: &'a dyn FilesystemPort,
        templates: &'a dyn TemplatesPort,
        logger: &'a dyn LoggerPort,
    ) -> Self {
        GenerateHandler {
            config,
            fs,
            templates,
            logger,
        }
    }

    pub fn handle(&self, path: &Path, name: Option<String>, pkg: Option<String>) {
        if let Some(file_name_os_str) = path.file_name() {
            if let Some(file_name) = file_name_os_str.to_str() {
                self.logger
                    .info(format!("Generating {} file", file_name).as_ref());
                if let Some(file_config) = self.config.files.iter().find(|f| f.name == file_name) {
                    self.logger
                        .info(format!("Using template {}", file_config.template).as_ref());
                    let content = self.templates.load(&file_config.template, name, pkg);
                    if let Ok(_) = self.fs.write_file(path, &content.unwrap()) {
                        self.logger.success("File generated");
                        return;
                    }
                }
            }
        }
        self.logger.error("Error generating file");
    }
}
