//! Manages the POP3 connection by wrapping a TCP stream and parsing
//! every command from the client. Also responsible for sending responses
//! from the server.

use tokio::net::{TcpStream};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use bytes::{Bytes, BytesMut, BufMut};

use crate::{POP3Response, POP3Command, err::POP3CommandErr};
use POP3Command::*;

pub struct POP3Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl POP3Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            stream: socket,
            buffer: BytesMut::with_capacity(4096)
        }
    }

    /// Send a response or greeting to the client
    pub async fn send_response(&mut self, response: POP3Response) -> Result<(), io::Error> {
        self.stream.write_all(&Bytes::from(response)[..]).await?;

        Ok(())
    }

    /// Read a POP3Command from the client.
    /// 
    /// If the client sends two commands in a row without the server
    /// calling this method, Things Will Break.
    pub async fn read_command(&mut self) -> Result<POP3Command, io::Error> {
        loop {
            // Write the bytes from the client into the buffer
            self.stream.read_buf(&mut self.buffer).await?;
            println!("{:?}", self.buffer);
    
            match POP3Command::parse(self.buffer.clone().freeze()) {
                Ok(command) => {
                    self.buffer.clear();
                    return Ok(command);
                },
                Err(POP3CommandErr::IncompleteResponse) => (),
                Err(POP3CommandErr::UnknownCommand(_)) => {
                    self.buffer.clear();
                    self.send_response(POP3Response::negative("unknown command")).await?;
                },
                Err(_) => self.buffer.clear(),
            };
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), io::Error> {
        // Greet the client
        self.send_response(POP3Response::positive("good morning")).await?;

        loop {
            let command = self.read_command().await?;

            // Handle the commands valid in the authentication state
            match command {
                Username { username: _ } => self.send_response(POP3Response::positive("")).await?,
                Password { password: _ } => {
                    self.send_response(POP3Response::positive("")).await?;
                    return Ok(());
                },
                APop { username: _, md5_digest: _ } => {
                    self.send_response(POP3Response::positive("")).await?;
                    return Ok(());
                }
                Quit => {
                    self.close().await;
                    return Ok(());
                },
                _ => self.send_response(POP3Response::negative("command not valid during authentication")).await?
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
                    // TODO: delete messaged marked for deletion
                    self.close().await;
                    return Ok(());
                },
                Top { message_number: _, n: _ } => POP3Response::negative("unsupported"),
                UniqueIDListing { message_number: _ } => POP3Response::negative("unsupported"),
                _ => POP3Response::negative("command not valid during transaction"),
            }).await?;
        }
    }

    fn stat(&self) -> POP3Response {
        POP3Response::positive("1 100")
    }

    fn list(&self) -> POP3Response {
        POP3Response::positive("\r\n1 100")
    }

    fn retrieve(&self) -> POP3Response {
        POP3Response::positive("this is a message")
    }

    /// Close the connection
    pub async fn close(&self) {

    }
}