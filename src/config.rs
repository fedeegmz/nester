use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub templates: Templates,
    pub ktor: Files,
}

#[derive(Deserialize, Serialize)]
pub struct Templates {
    pub repository: String,
    pub branch: String,
    pub remote: String,
}

#[derive(Deserialize, Serialize)]
pub struct Files {
    pub module_files: Vec<ModuleFile>,
}

#[derive(Deserialize, Serialize)]
pub struct ModuleFile {
    pub name: String,
    pub template: String,
}

const DEFAULT_CONFIG: &str = r#"
[templates]
repository = "https://github.com/fedeegmz/nester-templates.git"
branch = "master"
remote = "origin"

[ktor]
module_files = [
    {name = "Injection", template = "injection.tera"},
    {name = "Routing", template = "routing.tera"},
    {name = "Service", template = "service.tera"},
]

"#;

pub fn load_config() -> Config {
    let config: Config = toml::from_str(DEFAULT_CONFIG).unwrap();
    config
}
