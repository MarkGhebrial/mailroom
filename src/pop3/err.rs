use std::error::Error;
use std::fmt;
use bytes::Bytes;

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

#[derive(PartialEq, Debug)]
pub enum POP3CommandErr {
    InvalidSyntax,
    UnknownCommand(Bytes),
    InvalidArguments,
    IncompleteResponse,
}

impl Error for POP3CommandErr {}

impl fmt::Display for POP3CommandErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use POP3CommandErr::*;

        let err_message = match self {
            InvalidSyntax => "POP3 client command syntax is invalid".to_string(),
            UnknownCommand(b) => format!("POP3 command '{:?}' is unknown or unsupported", &b[..]),
            InvalidArguments => "POP3 client did not supply the required arguments for a command".to_string(),
            IncompleteResponse => "POP3 cient command is incomplete".to_string(),
        };

        write!(f, "{}", err_message)
    }
}

#[derive(PartialEq, Debug)]
pub struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse")
    }
}