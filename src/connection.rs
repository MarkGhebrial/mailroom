//! Manages the POP3 connection by wrapping a TCP stream and parsing
//! every command from the client. Also responsible for sending responses
//! from the server.

use tokio::net::{TcpStream};
use tokio::io::{self, AsyncWriteExt};
use bytes::{Bytes, BytesMut};

use crate::response::{POP3Response};

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
}