//! Manages the POP3 connection by wrapping a TCP stream and parsing
//! every command from the client. Also responsible for sending responses
//! from the server.

use std::error::Error;
use std::str::{self, FromStr};

use bytes::{Bytes, BytesMut};
use email_address::EmailAddress;
use log::trace;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::connection_handler::ConnectionHandler;
use crate::pop3::{err::POP3CommandErr, POP3Command, POP3Response};
use POP3Command::*;

// use std::future::Future;

use crate::database::*;

pub struct POP3Connection {
    // Socket state
    stream: TcpStream,
    buffer: BytesMut,

    // Connection state
    username: Option<EmailAddress>,
    user: Option<user::Model>,
}

impl ConnectionHandler for POP3Connection {
    fn protocol_name() -> String {
        "POP3".to_owned()
    }

    fn from_stream(socket: TcpStream) -> Self {
        Self {
            stream: socket,
            buffer: BytesMut::new(),
            username: None,
            user: None,
        }
    }

    async fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        let authenticated: bool = self.authenticate().await?;
        if !authenticated {
            trace!("POP3 connection failed to authenticate");
            return Ok(());
        }
        trace!("POP3 connection authenticated");

        self.transaction().await?;
        trace!("POP3 connection finished");

        Ok(())
    }
}

impl POP3Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            stream: socket,
            buffer: BytesMut::new(),
            username: None,
            user: None,
        }
    }

    /// Commence the interaction with the client.
    pub async fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        let authenticated: bool = self.authenticate().await?;
        if !authenticated {
            trace!("POP3 connection failed to authenticate");
            return Ok(());
        }
        trace!("POP3 connection authenticated");

        self.transaction().await?;
        trace!("POP3 connection finished");

        Ok(())
    }

    /// Send a response or greeting to the client
    pub async fn send_response(&mut self, response: POP3Response) -> Result<(), io::Error> {
        let bytes = &Bytes::from(response)[..];
        self.stream.write_all(bytes).await?;

        Ok(())
    }

    /// Read a POP3Command from the client.
    ///
    /// If the client sends two commands in a row without the server
    /// calling this method, Things Will Break.
    pub async fn read_command(&mut self) -> Result<POP3Command, io::Error> {
        loop {
            // Write the bytes from the client into the buffer
            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                self.close().await?;
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }

            match POP3Command::parse(self.buffer.clone().freeze()) {
                Ok(command) => {
                    self.buffer.clear();
                    return Ok(command);
                }
                Err(POP3CommandErr::IncompleteResponse) => (),
                Err(POP3CommandErr::UnknownCommand(_)) => {
                    self.buffer.clear();
                    self.send_response(POP3Response::negative("unknown command"))
                        .await?;
                }
                Err(_) => self.buffer.clear(),
            };
        }
    }

    /// Authentication phase of the POP3 connection. During this phase, the
    /// server verifies the identity of the client.
    ///
    /// TODO: return authenticated user information
    pub async fn authenticate(&mut self) -> Result<bool, Box<dyn Error>> {
        // Greet the client
        self.send_response(POP3Response::positive("hello")).await?;

        loop {
            let command = self.read_command().await?;

            // Handle the commands valid in the authentication state
            match command {
                Username { username } => {
                    // Parse the bytes into a string and remember them
                    self.username = match str::from_utf8(&username) {
                        Ok(s) => EmailAddress::from_str(s).ok(),
                        Err(_) => None,
                    };
                    self.send_response(POP3Response::positive("")).await?
                }
                Password { password } => {
                    if let Ok(password) = str::from_utf8(&password) {
                        // Check the username and password combination
                        self.user = user_database::authenticate_user(
                            &self.username.as_ref().unwrap(),
                            password,
                        )
                        .await?;

                        // User is authenticated, so exit the authentication phase
                        if self.user.is_some() {
                            self.send_response(POP3Response::positive("Authenticated"))
                                .await?;
                            return Ok(true);
                        }
                    }

                    self.send_response(POP3Response::negative("Username or password is not valid"))
                        .await?;
                }
                APop {
                    username: _,
                    md5_digest: _,
                } => {
                    self.send_response(POP3Response::positive("")).await?;
                    return Ok(false);
                }
                Quit => {
                    self.close().await?;
                    return Ok(false);
                }
                // TODO: Update CAPA list as more features are implemented
                Capabilities => {
                    self.send_response(POP3Response::positive("\r\nUSER"))
                        .await?
                }
                _ => {
                    self.send_response(POP3Response::negative(
                        "command not valid during authentication",
                    ))
                    .await?
                }
            }
        }
    }

    pub async fn transaction(&mut self) -> Result<(), io::Error> {
        loop {
            let command = self.read_command().await?;

            self.send_response(match command {
                Stat => self.stat(),
                List { message_number: _ } => self.list(),
                Retrieve { message_number: _ } => self.retrieve(),
                Delete { message_number: _ } => POP3Response::positive(""),
                NoOp => POP3Response::positive(""),
                Reset => POP3Response::positive(""),
                Quit => {
                    // TODO: delete messages marked for deletion
                    self.close().await?;
                    return Ok(());
                }
                Top {
                    message_number: _,
                    n: _,
                } => POP3Response::negative("unsupported"),
                UniqueIDListing { message_number: _ } => POP3Response::negative("unsupported"),
                // TODO: Update CAPA list as more features are implemented
                Capabilities => POP3Response::positive("\r\nUSER"),
                _ => POP3Response::negative("command not valid during transaction"),
            })
            .await?;
        }
    }

    fn stat(&self) -> POP3Response {
        POP3Response::positive("1 100")
    }

    fn list(&self) -> POP3Response {
        POP3Response::positive("\r\n1 100")
    }

    fn retrieve(&self) -> POP3Response {
        POP3Response::positive("\r\nTo: mghebrialjr@localhost\r\nSubject: this is a message")
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<(), io::Error> {
        self.stream.shutdown().await?;
        Ok(())
    }
}
