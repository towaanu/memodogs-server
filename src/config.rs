use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 3030 }
    }
}

pub fn get_config() -> Config {
    envy::prefixed("MEMODOGS_").from_env().unwrap()
}
