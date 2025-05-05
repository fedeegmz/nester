use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Templates {
    pub repository: String,
    pub branch: String,
    pub remote: String,
}
