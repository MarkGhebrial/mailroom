use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use super::reply::*;
use crate::imf::Mail;

/// Handles an outgoing SMTP connection for sending email to another
/// domain.
pub struct OutgoingSMTPConnection {
    outgoing_message: Mail,
}

impl OutgoingSMTPConnection {
    fn new(message: Mail) {}

    async fn begin(&mut self) {
        // TODO: Port 587
        let mut socket = TcpStream::connect(self.outgoing_message.headers.get("To").unwrap()).await;

        // Await opening message

        // Send EHLO message
        // Await response
        // If the response is "command not recognised", then send HELO

        // Send MAIL_FROM command
        // Await response

        // Send RCPT TO commands
        // Await response

        // Send DATA command
        // Await response
        // Send message contents
        // Await response

        // Send QUIT command
        // Let the other server close the connection
    }

    async fn await_response(&mut self) -> Result<SMTPReply, ()> {
        todo!();
    }
}
