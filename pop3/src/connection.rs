//! Manages the POP3 connection by wrapping a TCP stream and parsing
//! every command from the client. Also responsible for sending responses
//! from the server.

use tokio::net::{TcpStream};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use bytes::{Bytes, BytesMut};

use crate::{POP3Response, POP3Command};

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

    pub async fn read_command(&mut self) -> POP3Command {
        let _n = self.stream.read_buf(&mut self.buffer).await;

        POP3Command::NoOp
    }
}