use crate::app::application::generate_handler::GenerateHandler;
use crate::app::application::pull_templates_handler::PullTemplatesHandler;
use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::repository_port::RepositoryPort;
use crate::core::port::templates_port::TemplatesPort;

pub struct CommandHandler<'a> {
    config: Config,
    fs: &'a dyn FilesystemPort,
    templates: &'a dyn TemplatesPort,
    repository: &'a dyn RepositoryPort,
}

impl<'a> CommandHandler<'a> {
    pub fn new(
        config: Config,
        fs: &'a dyn FilesystemPort,
        templates: &'a dyn TemplatesPort,
        repository: &'a dyn RepositoryPort,
    ) -> Self {
        Self {
            config,
            fs,
            templates,
            repository,
        }
    }

    pub fn generate(
        &self,
        path: String,
        name: Option<String>,
        pkg: Option<String>,
    ) -> Result<(), String> {
        let handler = GenerateHandler::new(self.config.clone(), self.fs, self.templates);
        handler.handle(path.as_ref(), name, pkg)
    }

    pub fn pull_templates(&self) -> Result<(), String> {
        let handler = PullTemplatesHandler::new(self.config.clone(), self.repository);
        handler.handle()
    }
}
