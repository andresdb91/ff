use argon2::password_hash::rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use figment::{Figment, providers, providers::Format};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub bind_ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: [u8; 32],
    pub use_session_cookie: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub auth: AuthConfig,
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
        let mut rng = OsRng {};
        let mut jwt_secret = [0u8; 32];
        rng.fill_bytes(&mut jwt_secret);

        Config {
            api: ApiConfig {
                bind_ip: String::from("0.0.0.0"),
                port: 3000,
            },
            auth: AuthConfig {
                jwt_secret,
                use_session_cookie: true,
            }
        }
    }
}
