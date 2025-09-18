mod cli;
mod config;
mod config_editor;
mod config_helpers;
mod connection_handler;
mod database;
mod imf;
mod pop3;
mod smtp;

use cli::*;
use config::*;

use crossterm::style::Stylize;

use connection_handler::ConnectionHandler;
use database::user_database::*;
use lazy_static::lazy_static;
use log::{error, info, warn};
use pop3::POP3Connection;
use smtp::IncomingSMTPConnection;
use std::{
    env::{self, current_exe},
    fs,
    process::exit,
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

        // Read the config file from disk
        let config_string = fs::read_to_string(&config_path).unwrap_or_else(|_err| {
            match env::var("CONFIG_PATH") {
                Ok(_) => error!("Couldn't find config file at {}", &config_path),
                Err(_) => error!("Couldn't find config file at {}\nYou can set the location of the config file by setting the CONFIG_PATH environment variable", &config_path),
            }
            // error!("Couldn't find config file at {}\n", &config_path);
            exit(-1);
        });

        // Parse the config file
        let config = toml::from_str(
            &config_string
        ).unwrap_or_else(|_err| {
            error!("Invalid configuration");
            exit(-1)
        });

        log::info!("Configuration loaded sucessfully.");

        config
    };
}

#[tokio::main]
async fn main() {
    // Use clap to parse the command line arguments
    let matches = cli().get_matches();

    // Run different functions based on the subcommand
    match matches.subcommand() {
        // No subcommand provided, so start the server
        None => run().await,
        Some(s) => match s {
            ("config", _args) => config_editor::run_config_editor(),
            (s, _args) => panic!("Subcommand {} not recognized", s),
        },
    }
}

/// Run the server. This function is executed when no command line arguments are provided.
async fn run() {
    // TODO: Handle the case where another instance of mailroom is already running

    init_logger();

    if sudo::with_env(&["CONFIG_PATH"]).is_err() {
        error!("Couldn't escalate privileges. Exiting.");
        return;
    }

    initialize_db().await.unwrap();

    print_gmail_mx_record().await;

    let pop3_handle = POP3Connection::start_listening(110).await;

    let smtp_handle = IncomingSMTPConnection::start_listening(3309).await;

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
    info!("Done printing mx record");
}

fn init_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            use log::Level;

            let date = chrono::Local::now();

            let level = match record.level() {
                Level::Error => "ERROR".red(),
                Level::Warn => "WARN".yellow(),
                Level::Info => "INFO".green(),
                Level::Debug => "DEBUG".magenta(),
                Level::Trace => "TRACE".grey(),
            };

            out.finish(format_args!(
                "{} {} {}\n{}\n",
                date.format("%m/%d/%Y %H:%M:%S").to_string().blue(),
                level,
                record.target().italic(),
                message.to_string().trim()
            ))
        })
        .level(log::LevelFilter::Info) // Set log level for dependencies
        .level_for("mailroom", log::LevelFilter::Trace) // Set log level for the application
        .chain(std::io::stdout())
        // TODO: All the terminal control characters for coloring and formatting text are written to the log file. Fix that
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .expect("failed to set up logger");
}
