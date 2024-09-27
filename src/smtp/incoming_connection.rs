use bytes::BytesMut;
use tokio::io::{self, AsyncWriteExt};
use tokio::{io::AsyncReadExt, net::TcpStream};

use super::SMTPCommand;

pub struct IncomingSMTPConnection {
    // Socket state
    stream: TcpStream,
    buffer: BytesMut,
}

impl IncomingSMTPConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::new(),
        }
    }

    pub async fn begin(&mut self) {}

    pub async fn read_command(&mut self) -> Result<SMTPCommand, io::Error> {
        loop {
            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                // Connection aborted
                self.close().await?;
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }

            let last_two_bytes = self.buffer.split_last_chunk::<2>();
            if let Some((_, bytes)) = last_two_bytes {
                if bytes == "\r\n".as_bytes() {}
            }
        }

        Ok(SMTPCommand::Noop)
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<(), io::Error> {
        self.stream.shutdown().await?;
        Ok(())
    }
}
