use crate::cfg::domain::config::Config;
use crate::core::port::filesystem_port::FilesystemPort;
use crate::shared::utils::cfg_utils::{get_config_file_path, get_config_path};

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
}

impl<'a> ConfigRepository<'a> {
    pub fn new(fs: &'a dyn FilesystemPort) -> Self {
        Self { fs }
    }

    pub fn load(&self) -> Config {
        let config_file = get_config_file_path();
        let config_file_path = config_file.as_path().to_str().unwrap();

        match self.fs.read_to_string(config_file_path) {
            Ok(content) => {
                if content.trim().is_empty() {
                    self.save_default_config(config_file_path)
                } else {
                    match toml::from_str::<Config>(&content) {
                        Ok(config) => config,
                        Err(_) => {
                            println!("Error parsing config file, using default config");
                            self.save_default_config(config_file_path)
                        }
                    }
                }
            }
            Err(_) => self.save_default_config(config_file_path),
        }
    }

    fn save_default_config(&self, config_file_path: &str) -> Config {
        let config_path = get_config_path();
        if !config_path.exists() {
            self.fs
                .create_dir_all(config_path.to_str().unwrap())
                .expect("Error creating config directory");
        }
        self.fs
            .write_string(config_file_path, DEFAULT_CONFIG)
            .expect("Error writing default config file");
        toml::from_str::<Config>(DEFAULT_CONFIG).unwrap()
    }
}
