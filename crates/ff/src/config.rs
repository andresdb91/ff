use serde::Deserialize;

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};

pub struct Config {
}

impl Config {
    pub fn new() -> Self {
        Config {
        }
    }
}
