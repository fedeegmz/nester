use crate::config::Config;
use crate::templates::{clone_templates_repo, fetch_templates_repo};
use crate::utils::{get_config_path, get_templates_path};
use std::error::Error;
use std::fs;

pub fn init(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_path = get_config_path();
    let templates_path = get_templates_path();

    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    if !templates_path.exists() {
        fs::create_dir_all(&templates_path)?;
    }

    if let Err(_) = fetch_templates_repo(
        &config.templates.remote.as_str(),
        &config.templates.branch.as_str(),
    ) {
        clone_templates_repo(config.templates.repository.as_str())?;
    }

    Ok(())
}
