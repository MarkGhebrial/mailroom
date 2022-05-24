//! Manages the POP3 connection by wrapping a TCP stream and parsing
//! every command from the client. Also responsible for sending responses
//! from the server.

use tokio::net::{TcpStream};
use tokio::io::BufWriter;
use bytes::{Buf, BytesMut};

use crate::response::{POP3ResponseStatus, POP3Response};

pub struct POP3Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl POP3Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4096)
        }
    }

    pub async fn send_response(response: POP3Response) -> Result<(), tokio::io::Error> {
        Ok(())
    }
}