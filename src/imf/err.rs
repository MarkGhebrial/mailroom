use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct MailParseError;

impl Error for MailParseError {}

impl fmt::Display for MailParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "couldn't parse RFC 5322 mail message")
    }
}
