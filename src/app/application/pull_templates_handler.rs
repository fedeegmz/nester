use crate::cfg::domain::config::Config;
use crate::core::port::repository_port::RepositoryPort;
use crate::shared::utils::cfg_utils::get_templates_path;

pub struct PullTemplatesHandler<'a> {
    config: Config,
    repository: &'a dyn RepositoryPort,
}

impl<'a> PullTemplatesHandler<'a> {
    pub fn new(config: Config, repository: &'a dyn RepositoryPort) -> Self {
        Self { config, repository }
    }

    pub fn handle(&self) -> Result<(), String> {
        let templates_path = get_templates_path();
        let remote = self.config.templates.remote.clone();
        let branch = self.config.templates.branch.clone();
        self.repository
            .pull(templates_path.as_path(), &*remote, branch)?;

        Ok(())
    }
}
