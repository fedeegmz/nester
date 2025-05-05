use crate::core::port::logger_port::LoggerPort;
use crate::core::port::repository_port::RepositoryPort;
use git2::Repository as GitRepository;
use std::path::Path;

pub struct Repository<'a> {
    logger: &'a dyn LoggerPort,
}

impl<'a> Repository<'a> {
    pub fn new(logger: &'a dyn LoggerPort) -> Self {
        Self { logger }
    }
}

impl<'a> RepositoryPort for Repository<'a> {
    fn clone(&self, repository: &str, path: &Path) {
        self.logger
            .info(format!("Cloning repository: {} into {}", repository, path.display()).as_str());
        if let Err(_) = GitRepository::clone(repository, path) {
            self.logger.error("Error cloning repository");
        }
    }

    fn pull(&self, path: &Path, remote: &str, branch: String) {
        if let Ok(repo) = GitRepository::open(path) {
            if let Ok(mut remote) = repo.find_remote(remote) {
                if let Err(_) = remote.fetch(&[branch], None, None) {
                    self.logger.error("Error fetching remote");
                } else {
                    self.logger.success("Remote fetched");
                }
            }
        }
    }
}
