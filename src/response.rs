//! 
//! 

use bytes::{Bytes, BytesMut};
use std::error::Error;
use std::fmt;

pub enum POP3ResponseStatus {
    Positive,
    Negative,
}

pub struct POP3Response {
    pub status: POP3ResponseStatus,
    pub message: Bytes
}

use POP3ResponseStatus::*;

impl POP3Response {
    pub fn new(status: POP3ResponseStatus, message: Bytes) -> Self {
        Self {
            status,
            message
        }
    }

    pub fn positive(message: Bytes) -> Self {
        Self::new(Positive, message)
    }

    pub fn negative(message: Bytes) -> Self {
        Self::new(Negative, message)
    }
}

// Convert Bytes to POP3Response
impl TryFrom<Bytes> for POP3Response {
    type Error = POP3ResponseErr;

    fn try_from(_bytes: Bytes) -> Result<Self, Self::Error> {
        //let m = &bytes.clone().into();

        Ok(Self::positive("".into()))
    }
}

/// Convert a POP3Response to Bytes
impl From<POP3Response> for Bytes {
    fn from(response: POP3Response) -> Bytes {
        let mut out = BytesMut::new();

        match response.status {
            Positive => out.extend_from_slice(b"+OK"),
            Negative => out.extend_from_slice(b"-ERR"),
        };

        // Return early if there is no message content
        if response.message.len() == 0 { return out.into() }

        out.extend_from_slice(b" ");
        out.extend_from_slice(&response.message[..]);

        // Check if the message has any CRLF sequences
        for i in 0..response.message.len() {
            if response.message.slice(i..i+2) == "\r\n" {
                // Terminate the multi-line response
                out.extend_from_slice(b"\r\n.\r\n");
            }
        }

        out.into()
    }
}

#[derive(Debug)]
pub enum POP3ResponseErr {
    /// Returned if the server's response doesn't start with "+OK" or "-ERR"
    InvalidStatus,
    /// Returned if the "CRLF.CRLF" sequence terminating a multiline response
    /// hasn't been recieved yet
    IncompleteResponse,
}

impl Error for POP3ResponseErr {}

impl fmt::Display for POP3ResponseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use POP3ResponseErr::*;

        let err_message = match self {
            InvalidStatus => "POP3 server response is invalid",
            IncompleteResponse => "POP3 server multiline response is incomplete",
        };

        write!(f, "{}", err_message)
    }
}