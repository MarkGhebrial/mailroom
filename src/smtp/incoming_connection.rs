use bytes::BytesMut;
use tokio::io::{self, AsyncWriteExt};
use tokio::{io::AsyncReadExt, net::TcpStream};

use std::error::Error;

use crate::connection_handler::ConnectionHandler;

use super::{SMTPCommand, SMTPCommandParseError};

/// Handles an incoming SMTP connection from another email server or an email client.
pub struct IncomingSMTPConnection {
    // Socket state
    stream: TcpStream,
    buffer: String,
}

impl ConnectionHandler for IncomingSMTPConnection {
    fn protocol_name() -> &'static str {
        "SMTP"
    }

    fn from_stream(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: String::new(),
        }
    }

    async fn begin(&mut self) -> Result<(), Box<dyn Error>> {
        let command = self.read_command().await.unwrap();

        println!("RECEIVED SMTP COMMAND: {:?}", command);

        Ok(())
    }
}

impl IncomingSMTPConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: String::new(),
        }
    }

    pub async fn read_command(&mut self) -> Result<SMTPCommand, io::Error> {
        loop {
            if self.stream.read_to_string(&mut self.buffer).await? == 0 {
                // Connection aborted
                self.close().await?;
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            };

            match SMTPCommand::try_from(self.buffer.as_str()) {
                Ok(command) => {
                    self.buffer.clear();
                    return Ok(command);
                }
                Err(SMTPCommandParseError::IncompleteCommand) => {
                    continue;
                }
                Err(e) => println!("SMTP command parse error: {:?}", e),
            }
        }

        // TODO: Fix unreachable
        // Ok(SMTPCommand::Noop)
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<(), io::Error> {
        self.stream.shutdown().await?;
        Ok(())
    }
}
