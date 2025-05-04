use std::path::Path;

/// Port for interacting with a Git repository.
pub trait RepositoryPort {
    /// Clones a repository from a URL to a specified path.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the repository to clone.
    /// * `path` - The local path where the repository should be cloned.
    ///
    /// # Returns
    ///
    /// `Ok(())` if cloning is successful, otherwise an error string.
    fn clone(&self, url: &str, path: &Path) -> Result<(), String>;

    // Add other repository operations here if needed in the future,
    // e.g., pull, checkout, get_current_branch, etc.
    // fn pull(&self, path: &Path) -> Result<(), String>;
}
