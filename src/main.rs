mod cli;
mod config;
mod config_helpers;
mod connection_handler;
mod database;
mod imf;
mod pop3;
mod smtp;

use cli::*;
use config::*;

// use ratatui::style::Stylize;
// use colored::Colorize;
use crossterm::style::Stylize;

use connection_handler::ConnectionHandler;
use crossterm::event::{self, Event, KeyCode};
use database::user_database::*;
use lazy_static::lazy_static;
use log::{debug, info, trace, warn};
use pop3::POP3Connection;
use ratatui::{
    layout::{Constraint, Layout},
    text::Text,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use smtp::IncomingSMTPConnection;
use std::{
    env::{self, current_exe},
    fs,
    time::SystemTime,
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
    // Use clap to parse the command line arguments
    let matches = cli().get_matches();

    // Run different functions based on the subcommand
    match matches.subcommand() {
        // No subcommand provided, so start the server
        None => run().await,
        Some(s) => match s {
            ("config", _args) => run_config_editor(),
            (s, _args) => panic!("Subcommand {} not recognized", s),
        },
    }
}

/// Run the server. This function is executed when no command line arguments are provided.
async fn run() {
    // TODO: Handle the case where another instance of mailroom is already running

    init_logger();

    // log4rs::init_file(Path::new(&CONFIG.log_4rs_config), Default::default())
    //     .expect("Couldn't find/load Log4rs configuration file");

    if sudo::with_env(&["CONFIG_PATH"]).is_err() {
        println!("Couldn't escalate privileges. Exiting.");
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

/// Run the configuration editor. This function runs when the "config" subcommand is provided.
///
/// The config editor will be a TUI application made with ratatui.
fn run_config_editor() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
            let [main_area, footer] = vertical.areas(frame.area());

            // Draw the footer
            Text::raw("Press q to quit; arrow keys to navigate; space or enter to select")/*.on_white()*/.render(footer, frame.buffer_mut());

            let horizontal = Layout::horizontal([Constraint::Length(25), Constraint::Fill(1)]);
            let [left_area, right_area] = horizontal.areas(main_area);

            let info = Paragraph::new("This is the configuration editor for mailroom. You can also change mailroom's configuration by directly editing config.toml.");

            let block = Block::bordered().border_type(BorderType::Thick);

            frame.render_widget(block, right_area);
            frame.render_widget(info, left_area);
        }).expect("failed to draw frame");

        // Handle terminal events
        match event::read().expect("failed to read terminal event") {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char('q') => {
                        // TODO: Save the configuration
                        // Exit the application
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Restore the terminal to its original state
    ratatui::restore();
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
            // let time = SystemTime::now();
            // humantime::format_rfc3339_seconds(time)
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
