mod config;
mod database;
mod pop3;
mod imf;

use crate::config::*;

use std::{fs, env};
use pop3::{POP3Connection};
use tokio::net::TcpListener;
use database::user_database::*;
use lazy_static::lazy_static;

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
    initialize_db().await.unwrap();

    let pop3_listener = TcpListener::bind("192.168.0.138:110").await.unwrap();

    let handle = tokio::spawn(async move {
        loop {
            let (socket, _) = pop3_listener.accept().await.unwrap();

            println!("Accepted POP3 connection");
            let mut connection = POP3Connection::new(socket);

            if let Err(e) = connection.begin().await {
                println!("POP3 Connection ended with error: {}", e)
            }
        }
    });
    handle.await.unwrap();
}