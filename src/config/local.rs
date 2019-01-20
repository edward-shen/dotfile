use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub groups: HashMap<String, Group>,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub packages: Vec<String>,
}
