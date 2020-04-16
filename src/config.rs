use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub default_sets: Option<Vec<String>>,
    pub init: Option<Configurable>,
    pub sets: Option<HashMap<String, Set>>,
    pub groups: Option<HashMap<String, String>>,
    pub config: Option<HashMap<String, Configurable>>,
}

#[derive(Deserialize, Debug)]
pub struct Configurable {
    pub before: Option<Vec<String>>,
    pub after: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Set {
    pub sets: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}
