use git2::Repository;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn init() -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("❌ Error: home dir not found")?;
    let config_path = home.join(".nester");
    let templates_path = config_path.join("templates");

    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    if !templates_path.exists() {
        fs::create_dir_all(&templates_path)?;
    }

    if let Ok(repo) = Repository::open(&templates_path) {
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main"], None, None)?;
    } else {
        clone_templates_repo(&templates_path)?;
    }

    Ok(())
}

pub fn clone_templates_repo(templates_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    Repository::clone(
        "https://github.com/fedeegmz/nester-templates.git",
        &templates_path,
    )?;

    Ok(())
}
