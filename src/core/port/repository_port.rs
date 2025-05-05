use std::path::Path;

pub trait RepositoryPort {
    fn clone(&self, url: &str, path: &Path);

    fn pull(&self, path: &Path, remote: &str, branch: String);
}
