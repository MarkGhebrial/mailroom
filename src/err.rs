use std::error::Error;
use std::fmt;

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
    /// Returned if the server's response doesn't start with "+OK" or "-ERR"
    InvalidSyntax,
    /// Returned if the "CRLF.CRLF" sequence terminating a multiline response
    /// hasn't been recieved yet
    IncompleteResponse,
}

impl Error for POP3CommandErr {}

impl fmt::Display for POP3CommandErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use POP3CommandErr::*;

        let err_message = match self {
            InvalidSyntax => "POP3 server response is invalid",
            IncompleteResponse => "POP3 server multiline response is incomplete",
        };

        write!(f, "{}", err_message)
    }
}

#[derive(PartialEq, Debug)]
pub enum MailboxError {
    MessageDoesNotExist,
}

impl Error for MailboxError {}

impl fmt::Display for MailboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MailboxError::*;

        let err_message = match self {
            MessageDoesNotExist => "specified message does not exist",
        };

        write!(f, "{}", err_message)
    }
}