use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::repository_port::RepositoryPort;
use crate::core::port::templates_port::TemplatesPort;
use crate::shared::utils::cfg_utils::get_templates_path;
use tera::{Context, Tera};

pub struct Templates<'a> {
    config: Config,
    fs: &'a dyn FilesystemPort,
    repository: &'a dyn RepositoryPort,
}

impl<'a> Templates<'a> {
    pub fn new(
        config: Config,
        fs: &'a dyn FilesystemPort,
        repository: &'a dyn RepositoryPort,
    ) -> Self {
        Self {
            config,
            fs,
            repository,
        }
    }
}

impl<'a> TemplatesPort for Templates<'a> {
    fn load(&self, template: &str, name: Option<String>, pkg: Option<String>) -> String {
        // TODO: receive name and pkg in a single struct
        let templates_path = get_templates_path();
        if !templates_path.exists() {
            self.fs
                .create_dir_all(&templates_path)
                .expect("Failed to create templates path");
            self.repository
                .clone(&*self.config.templates.repository, &templates_path)
                .expect("Error cloning repository");
        }
        let template_files = &format!("{}/tera/*.tera", templates_path.to_str().unwrap());

        let tera = Tera::new(&template_files).expect("❌ Error loading templates");

        let mut context = Context::new();
        context.insert("pkg", pkg.as_deref().unwrap_or_default());
        context.insert("name", name.as_deref().unwrap_or_default());

        let rendered = tera
            .render(template, &context)
            .expect("❌ Error rendering template");

        rendered
    }
}
