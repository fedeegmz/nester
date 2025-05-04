use super::file::File;
use super::templates::Templates;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub templates: Templates,
    pub files: Vec<File>,
}
