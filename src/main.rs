mod config;
mod config_helpers;
mod connection_handler;
mod database;
mod imf;
mod pop3;
mod smtp;

use crate::config::*;

use connection_handler::ConnectionHandler;
use database::user_database::*;
use lazy_static::lazy_static;
use log::{info, warn};
use pop3::POP3Connection;
use smtp::IncomingSMTPConnection;
use std::{
    env::{self, current_exe},
    fs,
    path::Path,
};

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

    if sudo::with_env(&["CONFIG_PATH"]).is_err() {
        println!("Couldn't escalate privileges. Exiting.");
        return
    }

    initialize_db().await.unwrap();

    print_gmail_mx_record().await;

    let pop3_handle = POP3Connection::start_listening(110).await;

    let smtp_handle = IncomingSMTPConnection::start_listening(3309).await;

    // let smtp_listener = TcpListener::bind("0.0.0.0:666").await.unwrap();

    // let smtp_handle = tokio::spawn(async move {
    //     loop {
    //         let (socket, addr) = smtp_listener.accept().await.unwrap();

    //         info!("Accepted SMTP connection from {}", addr);
    //         let mut connection = IncomingSMTPConnection::new(socket);

    //         tokio::spawn(async move {
    //             connection.begin().await;
    //         });
    //     }
    // });

    // Wait for the threads to finish
    pop3_handle.await.unwrap();
    smtp_handle.await.unwrap();
}

/// This function serves no purpose. It will eventually be deleted.
async fn print_gmail_mx_record() {
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
}
