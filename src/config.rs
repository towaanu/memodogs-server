use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub port: u16,
    pub images_path: String,
    pub static_base_url: String,
    pub cors_origin: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3030,
            images_path: String::from("assets/images"),
            static_base_url: Default::default(),
            cors_origin: Default::default(),
        }
    }
}

pub fn get_config() -> Config {
    envy::prefixed("MEMODOGS_").from_env().unwrap()
}
