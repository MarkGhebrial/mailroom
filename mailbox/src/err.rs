use std::error::Error;
use std::fmt;

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