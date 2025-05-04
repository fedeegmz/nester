use crate::core::port::repository_port::RepositoryPort;
use git2::Repository as GitRepository;
use std::path::Path;

pub struct Repository;

impl RepositoryPort for Repository {
    fn clone(&self, repository: &str, path: &Path) -> Result<(), String> {
        println!("Cloning repository: {} into {}", repository, path.display());
        match GitRepository::clone(repository, path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "❌ Error cloning repository '{}' to '{}': {}",
                repository,
                path.display(),
                e
            )),
        }
    }
}
