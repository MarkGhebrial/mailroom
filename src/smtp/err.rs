use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum SMTPReplyParseError {
    InvalidResponseCode(usize),
}

impl Error for SMTPReplyParseError {}

impl fmt::Display for SMTPReplyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SMTPReplyParseError::*;

        let err_message = match self {
            InvalidResponseCode(n) => format!("SMTP response code \"{}\" not recognized", n.to_string()),
            // IncompleteResponse => "POP3 server multiline response is incomplete",
        };

        write!(f, "{}", err_message)
    }
}