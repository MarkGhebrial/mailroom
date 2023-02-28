use serde::Deserialize;
use std::default::Default;
use std::env::current_exe;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_log_4rs_config")]
    pub log_4rs_config: String,
    pub database: DatabaseCfg,
    pub domains: Vec<DomainCfg>,
}

/// Looks for a file named "log4rs.yaml" in the same directory as the
/// server executable
fn default_log_4rs_config() -> String {
    let mut dir = current_exe().unwrap();
    dir.set_file_name("log4rs.yaml");
    dir.as_path().to_str().unwrap().to_owned()
}

#[derive(Deserialize)]
pub struct PostgresCfg {
    /// PostgreSQL server hostname
    #[serde(default = "default_postgres_host")]
    pub hostname: String,
    /// PostgreSQL username
    pub user: String,
    /// PostgreSQL password (for specified user)
    pub password: String,
}

#[derive(Deserialize)]
pub struct DatabaseCfg {
    /// Database URL
    pub url: String,
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
