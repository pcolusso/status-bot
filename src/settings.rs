use std::error::Error;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Settings {
    pub api_key: String
}

pub fn load() -> Result<Settings, Box<Error>> {
    let f = std::fs::File::open("settings.yml")?;
    let d: Settings = serde_yaml::from_reader(f)?;

    Ok(d)
}