use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum DbError {
    UserDoesNotExist,
}

impl Error for DbError {}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DbError::*;

        let err_message = match self {
            UserDoesNotExist => "specified user is not in database",
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