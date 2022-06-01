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

    /// Read a POP3Command from the client
    pub async fn read_command(&mut self) -> Result<POP3Command, io::Error> {
        loop {
            // Write 
            self.stream.read_buf(&mut self.buffer).await?;
            println!("{:?}", self.buffer);
    
            match POP3Command::parse(self.buffer.clone().freeze()) {
                Ok(command) => return Ok(command),
                Err(POP3CommandErr::IncompleteResponse) => (),
                Err(POP3CommandErr::UnknownCommand(_)) => {
                    self.buffer.clear();
                    self.send_response(POP3Response::negative("")).await.unwrap();
                },
                Err(_) => self.buffer.clear(),
            };
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), io::Error> {
        let mut done = false;
        while !done {
            let command = self.read_command().await?;

            match command {
                Username { username: _ } => self.send_response(POP3Response::positive("")).await?,
                Password { password: _ } => self.send_response(POP3Response::positive("")).await?,
                APop { username: _, md5_digest: _ } => self.send_response(POP3Response::positive("")).await?,
                Quit => self.close().await,
                _ => self.send_response(POP3Response::negative("command not valid during authentication")).await?
            }
        }

        Ok(())
    }

    /// Close the connection
    pub async fn close(&self) {

    }
}