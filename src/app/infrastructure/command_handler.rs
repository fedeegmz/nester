use crate::app::application::generate_handler::GenerateHandler;
use crate::cfg::domain::config::Config;
use crate::shared::infrastructure::filesystem::Filesystem;
use crate::shared::infrastructure::template_repository::TemplateRepository;

pub struct CommandHandler {
    config: Config,
    fs: Filesystem,
}

impl CommandHandler {
    pub fn new(config: Config, fs: Filesystem) -> Self {
        Self { config, fs }
    }

    pub fn generate(
        &self,
        path: String,
        name: Option<String>,
        pkg: Option<String>,
    ) -> Result<(), String> {
        let templates = TemplateRepository::new(self.config.clone(), self.fs);
        let handler = GenerateHandler::new(self.config.clone(), self.fs, templates);

        handler.handle(path, name, pkg)
    }
}
