use serde::{Serialize, Deserialize};

use figment::{
    Figment,
    providers,
    providers::Format,
};

#[derive(Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn new(overrides: Option<Config>) -> Self {
        Figment::from(providers::Serialized::defaults(Config::default()))
            .merge(providers::Toml::file("config.toml"))
            .merge(providers::Env::prefixed("FF_"))
            .merge(providers::Serialized::defaults(overrides.unwrap_or(Config{})))
            .extract()
            .expect("Error parsing configuration")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {}
    }
}
