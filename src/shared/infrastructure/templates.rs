use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::templates_port::TemplatesPort;
use crate::shared::utils::cfg_utils::get_templates_path;
use git2::Repository;
use std::path::PathBuf;
use tera::{Context, Tera};

pub struct Templates<'a> {
    config: Config,
    fs: &'a dyn FilesystemPort,
}

impl<'a> Templates<'a> {
    pub fn new(config: Config, fs: &'a dyn FilesystemPort) -> Self {
        Self { config, fs }
    }
}

impl<'a> TemplatesPort for Templates<'a> {
    fn load(&self, template: &str, name: Option<String>, pkg: Option<String>) -> String {
        // TODO: receive name and pkg in a single struct
        let templates_path = get_templates_path();
        if !templates_path.exists() {
            let path = templates_path.to_str().unwrap();
            self.fs
                .create_dir_all(&path)
                .expect("Failed to create templates path");
            clone_templates_repo(&templates_path, &*self.config.templates.repository);
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

fn clone_templates_repo(path: &PathBuf, repository: &str) {
    println!("Cloning templates repository: {}", &repository);
    if path.is_dir() && path.exists() {
        Repository::clone(repository, &path).expect("❌ Error cloning templates repository");
    }
}
