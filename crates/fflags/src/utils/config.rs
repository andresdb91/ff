use serde::{Deserialize, Serialize};

use figment::{Figment, providers, providers::Format};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub bind_ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
}

impl Config {
    pub fn new(overrides: Option<&str>) -> Self {
        Figment::from(providers::Serialized::defaults(Config::default()))
            .merge(providers::Toml::file(overrides.unwrap_or("config.toml")))
            .merge(providers::Env::prefixed("FF_"))
            .extract()
            .expect("Error parsing configuration")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig {
                bind_ip: String::from("0.0.0.0"),
                port: 3000,
            },
        }
    }
}
