use crate::pop3::err::POP3ResponseErr;
use bytes::{Bytes, BytesMut};

use POP3ResponseStatus::*;

/// Represents the possible POP3 server status indicators
///
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
    pub message: Bytes,
}

impl POP3Response {
    /// Create a new POP3Response
    pub fn new(status: POP3ResponseStatus, message: Bytes) -> Self {
        Self { status, message }
    }

    /// Create a positive POP3Response
    pub fn positive<T: Into<Bytes>>(message: T) -> Self {
        Self::new(Positive, message.into())
    }

    /// Create a negative POP3Response
    pub fn negative<T: Into<Bytes>>(message: T) -> Self {
        Self::new(Negative, message.into())
    }

    /// Parse Bytes into a multiline POP3Response
    pub fn parse_multiline(mut bytes: Bytes) -> Result<Self, POP3ResponseErr> {
        use POP3ResponseErr::*;

        if bytes.len() >= 5 && bytes.slice(bytes.len() - 5..) == "\r\n.\r\n" {
            bytes.truncate(bytes.len() - 5);
        } else {
            return Err(IncompleteResponse);
        }

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
        if let Some(c) = bytes.get(msg_start - 1) {
            if c != &b' ' {
                return Err(InvalidSyntax);
            }
        }

        let message = bytes.slice(msg_start..);

        Ok(Self::new(status, message))
    }

    /// Parse Bytes into an one-line POP3Response
    pub fn parse_oneline(mut bytes: Bytes) -> Result<Self, POP3ResponseErr> {
        use POP3ResponseErr::*;

        if bytes.len() >= 2 && bytes.slice(bytes.len() - 2..) == "\r\n" {
            bytes.truncate(bytes.len() - 2);
        } else {
            return Err(IncompleteResponse);
        }

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
        if let Some(c) = bytes.get(msg_start - 1) {
            if c != &b' ' {
                return Err(InvalidSyntax);
            }
        }

        let message = bytes.slice(msg_start..);

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
        if response.message.len() != 0 {
            out.extend_from_slice(b" ");
            out.extend_from_slice(&response.message[..]);
        }

        // Check if the message is multiline...
        if contains_crlf(&response.message) {
            // ...if so, add the multiline terminator
            out.extend_from_slice(b"\r\n.\r\n");
        } else {
            // ...otherwise, add a CRLF pair
            out.extend_from_slice(b"\r\n");
        }

        out.into()
    }
}

/// Check if a Bytes contains a CRLF sequence ("\r\n")
fn contains_crlf(bytes: &Bytes) -> bool {
    if bytes.len() < 2 {
        return false;
    }

    for i in 0..bytes.len() - 1 {
        if bytes.slice(i..i + 2) == "\r\n" {
            return true;
        }
    }
    false
}

#[test]
fn bytes_to_pop3_response() {
    // These should parse with no problem
    assert_eq!(
        POP3Response::parse_oneline(Bytes::from("+OK\r\n")).unwrap(),
        POP3Response::positive("")
    );
    assert_eq!(
        POP3Response::parse_oneline(Bytes::from("-ERR\r\n")).unwrap(),
        POP3Response::negative("")
    );
    assert_eq!(
        POP3Response::parse_oneline(Bytes::from("+OK Hello, world!\r\n")).unwrap(),
        POP3Response::positive("Hello, world!")
    );
    assert_eq!(
        POP3Response::parse_multiline(Bytes::from(
            "+OK This\r\nis\r\na\r\nmulti.line\r\n.f\r\nmessage\r\n.\r\n"
        ))
        .unwrap(),
        POP3Response::positive("This\r\nis\r\na\r\nmulti.line\r\n.f\r\nmessage")
    );

    // These will not parse for syntactical reasons
    assert_eq!(
        POP3Response::parse_oneline(Bytes::from("+ok\r\n"))
            .err()
            .unwrap(),
        POP3ResponseErr::InvalidSyntax
    );
    assert_eq!(
        POP3Response::parse_oneline(Bytes::from("-eRr\r\n"))
            .err()
            .unwrap(),
        POP3ResponseErr::InvalidSyntax
    );
    assert_eq!(
        POP3Response::parse_oneline(Bytes::from("+OKHello, World\r\n"))
            .err()
            .unwrap(),
        POP3ResponseErr::InvalidSyntax
    );

    // This will not parse because it excludes the termination sequence "CRLF.CRLF"
    assert_eq!(
        POP3Response::parse_multiline(Bytes::from("+OK 2 messages\r\n1 200\r\n 2 200"))
            .err()
            .unwrap(),
        POP3ResponseErr::IncompleteResponse
    );
}

#[test]
fn pop3_response_to_bytes() {
    assert_eq!(
        Bytes::from(POP3Response::positive("this is a test")),
        Bytes::from("+OK this is a test\r\n")
    );
    assert_eq!(
        Bytes::from(POP3Response::negative("this is a\r\nmultiline test")),
        Bytes::from("-ERR this is a\r\nmultiline test\r\n.\r\n")
    );
}
