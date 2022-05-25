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

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        use POP3ResponseErr::*;

        let msg_start;

        // Parse the response status
        let status = if bytes.slice(0..3) == "+OK" {
            msg_start = 4;
            Positive
        } else if bytes.slice(0..4) == "-ERR" {
            msg_start = 5;
            Negative
        } else {
            return Err(InvalidStatus);
        };

        if bytes.len() == msg_start - 1 {
            return Ok(Self::new(status, "".into()));
        }

        // Check that there is a space between the status and the message
        if let Some(c) = bytes.get(msg_start-1) {
            if c != &b' ' {
                return Err(InvalidStatus)
            }
        }

        let mut message = bytes.slice(msg_start..);
        if contains_crlf(&message) {
            if message.slice(message.len()-5..) == "\r\n.\r\n" {
                message = message.slice(..message.len()-5); // Remove the terminating sequence from the message
            } else {
                return Err(IncompleteResponse);
            }
        }

        Ok(Self::new(status, message))
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

        // Check if the message is multiline
        if contains_crlf(&response.message) {
            out.extend_from_slice(b"\r\n.\r\n");
        }

        out.into()
    }
}

fn contains_crlf(bytes: &Bytes) -> bool {
    for i in 0..bytes.len()-1 {
        if bytes.slice(i..i+2) == "\r\n" {
            return true;
        }
    }
    false
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