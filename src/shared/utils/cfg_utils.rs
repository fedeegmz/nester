use std::path::PathBuf;

fn get_home_dir() -> PathBuf {
    dirs::home_dir().expect("❌ Failed to get home directory")
}

pub fn get_config_path() -> PathBuf {
    let home = get_home_dir();
    home.join(".nester")
}

pub fn get_config_file_path() -> PathBuf {
    let config_path = get_config_path();
    config_path.join("cfg.toml")
}

pub fn get_templates_path() -> PathBuf {
    let config_path = get_config_path();
    config_path.join("templates")
}
