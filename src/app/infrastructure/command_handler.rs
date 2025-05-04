use crate::app::application::generate_handler::GenerateHandler;
use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::templates_port::TemplatesPort;

pub struct CommandHandler<'a> {
    config: Config,
    fs: &'a dyn FilesystemPort,
    templates: &'a dyn TemplatesPort,
}

impl<'a> CommandHandler<'a> {
    pub fn new(
        config: Config,
        fs: &'a dyn FilesystemPort,
        templates: &'a dyn TemplatesPort,
    ) -> Self {
        Self {
            config,
            fs,
            templates,
        }
    }

    pub fn generate(
        &self,
        path: String,
        name: Option<String>,
        pkg: Option<String>,
    ) -> Result<(), String> {
        let handler = GenerateHandler::new(self.config.clone(), self.fs, self.templates);

        handler.handle(path, name, pkg)
    }
}
