use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub groups: HashMap<Group>,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub packages: Vec<String>,
}
