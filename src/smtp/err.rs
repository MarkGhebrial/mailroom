use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum SMTPReplyParseError {
    InvalidResponseCode(usize),
    InvalidSyntax,

    /// Indicates that the separator between the response code
    /// and the text is something other than ' ' or '-'
    InvalidSeparator(char),
}

impl Error for SMTPReplyParseError {}

impl fmt::Display for SMTPReplyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SMTPReplyParseError::*;

        let err_message = match self {
            InvalidResponseCode(n) => {
                format!("SMTP response code \"{}\" not recognized", n.to_string())
            }
            InvalidSyntax => "SMTP reply syntax is invalid".to_owned(),
            InvalidSeparator(c) => format!("SMTP response separator \'{}\' not valid", c),
        };

        write!(f, "{}", err_message)
    }
}
