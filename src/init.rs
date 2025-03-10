use crate::config::Config;
use crate::templates;
use crate::utils;
use std::error::Error;
use std::fs;

pub fn init(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_path = utils::get_config_path();
    let templates_path = utils::get_templates_path();

    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    if !templates_path.exists() {
        fs::create_dir_all(&templates_path)?;
    }

    templates::clone_templates_repo(config.templates.repository.as_str())?;

    Ok(())
}
