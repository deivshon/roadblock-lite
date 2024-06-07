use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub regex: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}
