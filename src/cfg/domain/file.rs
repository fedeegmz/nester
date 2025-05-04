use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct File {
    pub name: String,
    pub template: String,
}
