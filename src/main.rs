mod config;
mod config_helpers;
mod database;
mod imf;
mod pop3;
mod smtp;

use crate::config::*;

use database::user_database::*;
use lazy_static::lazy_static;
use log::{info, warn};
use pop3::POP3Connection;
use std::{
    env::{self, current_exe},
    fs,
    path::Path,
};
use tokio::net::TcpListener;

use trust_dns_resolver::config::*;
use trust_dns_resolver::{TokioAsyncResolver, TokioHandle};

lazy_static! {
    // Load the configuration into a global static variable
    static ref CONFIG: Config = {
        let config_path = match env::var("CONFIG_PATH") {
            Ok(path) => path,
            Err(_) => {
                // Look for the file in the same working directory as the executable
                let mut path = current_exe().unwrap();
                path.set_file_name("config.toml");
                path.as_path().to_str().unwrap().to_owned()
            }
        };

        toml::from_str(
            fs::read_to_string(&config_path)
            .expect(&format!("Couldn't find config file at {}", &config_path)).as_str()
        ).expect("Invalid configuration")
    };
}

#[tokio::main]
async fn main() {
    log4rs::init_file(Path::new(&CONFIG.log_4rs_config), Default::default())
        .expect("Couldn't find/load Log4rs configuration file");

    initialize_db().await.unwrap();

    let dns_resolver = TokioAsyncResolver::new(
        ResolverConfig::default(),
        ResolverOpts::default(),
        TokioHandle,
    )
    .unwrap();

    let response = dns_resolver.mx_lookup("gmail.com").await.unwrap();
    for addr in response.iter() {
        warn!("{:?}", addr);
    }
    warn!("Done printing mx record");

    let pop3_listener = TcpListener::bind("0.0.0.0:110").await.unwrap();

    let handle = tokio::spawn(async move {
        loop {
            let (socket, addr) = pop3_listener.accept().await.unwrap();

            info!("Accepted POP3 connection from {}", addr);
            let mut connection = POP3Connection::new(socket);

            // Handle the connection in a new async task
            tokio::spawn(async move {
                if let Err(e) = connection.begin().await {
                    warn!("POP3 connection with {} ended with error: {}", addr, e)
                } else {
                    info!("POP3 connection with {} finished", addr);
                }
            });
        }
    });
    handle.await.unwrap();
}
