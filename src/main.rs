mod config;
mod config_helpers;
mod database;
mod pop3;
mod imf;

use crate::config::*;

use std::{fs, env, path::Path};
use pop3::{POP3Connection};
use tokio::net::TcpListener;
use database::user_database::*;
use lazy_static::lazy_static;
use log::{info, warn};

lazy_static! {
    // Load the configuration into a global static variable
    static ref CONFIG: Config = {
        let config_path = env::var("CONFIG_PATH")
            .expect(
                "No configuration file path specified. Ensure that the `CONFIG_PATH` environment variable is set"
            );
    
        toml::from_str(
            fs::read_to_string(config_path)
            .expect("Couldn't find config file").as_str()
        ).expect("Invalid configuration")
    };
}

#[tokio::main]
async fn main() {
    log4rs::init_file(
        Path::new(&CONFIG.log_4rs_config),
        Default::default()
    ).expect("Couldn't find/load Log4rs configuration file");

    initialize_db().await.unwrap();

    let pop3_listener = TcpListener::bind("192.168.0.138:110").await.unwrap();

    let handle = tokio::spawn(async move {
        loop {
            let (socket, addr) = pop3_listener.accept().await.unwrap();

            info!("Accepted POP3 connection from {}", addr);
            let mut connection = POP3Connection::new(socket);

            if let Err(e) = connection.begin().await {
                warn!("POP3 connection with {} ended with error: {}", addr, e)
            } else {
                info!("POP3 connection with {} finished", addr);
            }
        }
    });
    handle.await.unwrap();
}