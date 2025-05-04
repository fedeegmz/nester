use crate::cfg::domain::config::Config;
use crate::shared::infrastructure::filesystem::Filesystem;
use crate::shared::infrastructure::template_repository::TemplateRepository;

pub struct GenerateHandler {
    config: Config,
    fs: Filesystem,
    templates: TemplateRepository,
}

impl GenerateHandler {
    pub fn new(config: Config, fs: Filesystem, templates: TemplateRepository) -> Self {
        GenerateHandler {
            config,
            fs,
            templates,
        }
    }

    pub fn handle(
        &self,
        path: String,
        name: Option<String>,
        pkg: Option<String>,
    ) -> Result<(), String> {
        let file_name = path.split("/").last().unwrap().to_string();
        let file = self
            .config
            .files
            .iter()
            .filter(|x| x.name == file_name)
            .next()
            .unwrap();
        let content = self.templates.load(&*file.template, name, pkg);
        self.fs
            .write_string(&*path, &*content)
            .expect("Error writing file");
        Ok(())
    }
}
