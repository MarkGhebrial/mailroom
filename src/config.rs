use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    domains: Vec<Domain>,
}

#[derive(Deserialize)]
pub struct Domain {
    name: String,
    tls_settings: Option<TlsSettings>,
    users: Vec<String>,
}

#[derive(Deserialize)]
pub struct TlsSettings {
    // TODO
}