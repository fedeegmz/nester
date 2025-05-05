use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::core::port::logger_port::LoggerPort;
use crate::shared::utils::cfg_utils::{get_config_file_path, get_config_path};
use std::path::Path;

const DEFAULT_CONFIG: &str = r#"
[templates]
repository = "https://github.com/fedeegmz/nester-templates.git"
branch = "master"
remote = "origin"

[[files]]
name = "Injection.kt"
template = "injection.tera"

[[files]]
name = "Routing.kt"
template = "routing.tera"

[[files]]
name = "Service.kt"
template = "service.tera"
"#;

pub struct ConfigRepository<'a> {
    fs: &'a dyn FilesystemPort,
    logger: &'a dyn LoggerPort,
}

impl<'a> ConfigRepository<'a> {
    pub fn new(fs: &'a dyn FilesystemPort, logger: &'a dyn LoggerPort) -> Self {
        Self { fs, logger }
    }

    pub fn load(&self) -> Config {
        let config_file_path = get_config_file_path();

        match self.fs.read_file(&config_file_path) {
            Ok(content) => {
                if content.trim().is_empty() {
                    self.save_default_config(&config_file_path)
                } else {
                    match toml::from_str::<Config>(&content) {
                        Ok(config) => config,
                        Err(_) => {
                            self.logger
                                .warn("Error parsing config file, using default config");
                            self.save_default_config(&config_file_path)
                        }
                    }
                }
            }
            Err(_) => self.save_default_config(&config_file_path),
        }
    }

    fn save_default_config(&self, config_file_path: &Path) -> Config {
        let config_path = get_config_path();
        if !config_path.exists() {
            if let Err(_) = self.fs.create_dir_all(&config_path) {
                self.logger.warn("Error creating config directory");
            }
        }
        if let Err(_) = self.fs.write_file(config_file_path, DEFAULT_CONFIG) {
            self.logger.warn("Error writing default config file");
        }
        toml::from_str::<Config>(DEFAULT_CONFIG).unwrap()
    }
}
