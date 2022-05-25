use bytes::{Bytes, BytesMut};
use std::error::Error;
use std::fmt;

use POP3ResponseStatus::*;

/// POP3 servers reply with only two response codes: "+OK" and "-ERR"
/// The "+OK" code is called the positive status indicator, and the 
/// "-ERR" code is called the negative status indicator.
#[derive(PartialEq, Debug)]
pub enum POP3ResponseStatus {
    Positive,
    Negative,
}

/// Represents a POP3 server response, encapsulating the status indicator
/// and the message.
#[derive(PartialEq, Debug)]
pub struct POP3Response {
    pub status: POP3ResponseStatus,
    pub message: Bytes
}

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

    /// Attempt to convert Bytes to a POP3Response. If the attempt fails,
    /// a POP3ResponseErr will be returned.
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        use POP3ResponseErr::*;

        let msg_start;

        // Parse the response status
        let status = if bytes.len() >= 3 && bytes.slice(..3) == "+OK" {
            msg_start = 4;
            Positive
        } else if bytes.len() >= 4 && bytes.slice(..4) == "-ERR" {
            msg_start = 5;
            Negative
        } else {
            return Err(InvalidSyntax);
        };

        // If there's no message, return before trying to parse it
        if bytes.len() == msg_start - 1 {
            return Ok(Self::new(status, "".into()));
        }

        // Check that there is a space between the status and the message
        if let Some(c) = bytes.get(msg_start-1) {
            if c != &b' ' {
                return Err(InvalidSyntax)
            }
        }

        let mut message = bytes.slice(msg_start..);
        if contains_crlf(&message) {
            if message.slice(message.len()-5..) == "\r\n.\r\n" {
                // Remove the multiline terminating sequence from the message
                message = message.slice(..message.len()-5);
            } else {
                // If the message contains a CRLF sequence, but does not
                // end with the multiline response terminator, the message
                // is incomplete
                return Err(IncompleteResponse);
            }
        }

        Ok(Self::new(status, message))
    }
}

/// Convert a POP3Response to Bytes
impl From<POP3Response> for Bytes {
    /// Convert a POP3Response to Bytes
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

/// Check if a Bytes contains a CRLF sequence ("\r\n")
fn contains_crlf(bytes: &Bytes) -> bool {
    for i in 0..bytes.len()-1 {
        if bytes.slice(i..i+2) == "\r\n" {
            return true;
        }
    }
    false
}

#[derive(PartialEq, Debug)]
pub enum POP3ResponseErr {
    /// Returned if the server's response doesn't start with "+OK" or "-ERR"
    InvalidSyntax,
    /// Returned if the "CRLF.CRLF" sequence terminating a multiline response
    /// hasn't been recieved yet
    IncompleteResponse,
}

impl Error for POP3ResponseErr {}

impl fmt::Display for POP3ResponseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use POP3ResponseErr::*;

        let err_message = match self {
            InvalidSyntax => "POP3 server response is invalid",
            IncompleteResponse => "POP3 server multiline response is incomplete",
        };

        write!(f, "{}", err_message)
    }
}

#[test]
fn bytes_to_pop3_response() {
    // These should parse with no problem
    assert_eq!(
        POP3Response::try_from(Bytes::from("+OK")).unwrap(), 
        POP3Response::positive("".into())
    );
    assert_eq!(
        POP3Response::try_from(Bytes::from("-ERR")).unwrap(), 
        POP3Response::negative("".into())
    );
    assert_eq!(
        POP3Response::try_from(Bytes::from("+OK Hello, world!")).unwrap(), 
        POP3Response::positive("Hello, world!".into())
    );
    assert_eq!(
        POP3Response::try_from(Bytes::from(
            "+OK This\r\nis\r\na\r\nmulti.line\r\n.f\r\nmessage\r\n.\r\n"
        )).unwrap(), 
        POP3Response::positive("This\r\nis\r\na\r\nmulti.line\r\n.f\r\nmessage".into())
    );

    // These will not parse for syntactical reasons
    assert_eq!(
        POP3Response::try_from(Bytes::from("+ok")).err().unwrap(), 
        POP3ResponseErr::InvalidSyntax
    );
    assert_eq!(
        POP3Response::try_from(Bytes::from("-eRr")).err().unwrap(), 
        POP3ResponseErr::InvalidSyntax
    );
    assert_eq!(
        POP3Response::try_from(Bytes::from("+OKHello, World")).err().unwrap(), 
        POP3ResponseErr::InvalidSyntax
    );

    // This will not parse because it excludes the termination sequence "CRLF.CRLF"
    assert_eq!(
        POP3Response::try_from(Bytes::from("+OK 2 messages\r\n1 200\r\n 2 200")).err().unwrap(), 
        POP3ResponseErr::IncompleteResponse
    );
}