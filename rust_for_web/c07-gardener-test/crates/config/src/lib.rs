use std::net::SocketAddr;

use figment::providers::Format;
use figment::providers::Toml;
use figment::Figment;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mongo: String,
    pub listen: SocketAddr,
    pub database: String,
}

pub fn config() -> Result<Config, figment::Error> {
    Figment::new().merge(Toml::file("config.toml")).extract()
}
