use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::templates_port::TemplatesPort;

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
