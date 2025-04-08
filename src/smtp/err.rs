use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum SMTPReplyParseError {
    InvalidResponseCode(usize),
    InvalidSyntax,

    /// Indicates that the separator between the response code
    /// and the text is something other than ' ' or '-'
    InvalidSeparator(char),

    /// Indicates that the response did not follow the correct
    /// format for a multiline SMTP response.
    InvalidMultilineResponse,

    /// Indicates that the response is incomplete. Either missing
    /// a trailing CRLF or missing the final lines of a multiline
    /// response.
    IncompleteResponse,
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
            InvalidMultilineResponse => "SMTP multiline response is invalid".to_owned(),
            IncompleteResponse => "SMTP response is incomplete".to_owned(),
        };

        write!(f, "{}", err_message)
    }
}
