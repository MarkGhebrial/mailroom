use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseCfg,
    pub domains: Vec<DomainCfg>,
}

#[derive(Deserialize)]
pub struct DatabaseCfg {
    /// PostgreSQL server hostname
    #[serde(default = "default_postgres_host")]
    pub hostname: String,
    /// PostgreSQL username
    pub user: String,
    /// PostgreSQL password (for specified user)
    pub password: String,
}

fn default_postgres_host() -> String {
    "localhost".into()
}

#[derive(Deserialize)]
pub struct DomainCfg {
    /// Domain name
    pub name: String,
    pub tls_settings: TlsSettings,
    pub users: Vec<String>,
}

#[derive(Deserialize)]
pub enum TlsSettings {
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for TlsSettings {
    fn default() -> Self {
        Self::Disabled
    }
}