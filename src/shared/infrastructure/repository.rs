use crate::core::port::repository_port::RepositoryPort;
use git2::Repository as GitRepository;
use std::path::Path;

pub struct Repository {}

impl Repository {
    pub fn new() -> Self {
        Self {}
    }
}

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

    fn pull(&self, path: &Path, remote: &str, branch: String) -> Result<(), String> {
        if let Ok(repo) = GitRepository::open(path) {
            let mut remote = repo.find_remote(remote).expect("Failed to find remote");
            remote
                .fetch(&[branch], None, None)
                .expect("Failed to fetch");
        }

        Ok(())
    }
}
