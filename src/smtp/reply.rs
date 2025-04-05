use crate::smtp::{reply, SMTPReplyParseError};

/// Represents an SMTP reply. See Section 4.2 of [RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321#section-4.2)
pub struct SMTPReply {
    /// The three digit numeric code
    code: SMTPReplyCode,

    text: String,
}

impl TryFrom<&str> for SMTPReply {
    type Error = SMTPReplyParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use SMTPReplyParseError::*;

        // Single line SMTP replies are in the form
        // "xxx words words words\r\n"
        //
        // Single line replies can also be of the form
        // "xxx\r\n"
        // Notice that there's no " " after the reply code.
        //
        // Multi line SMTP replies are in the form
        // "xxx-words words words\r\n
        // xxx-words words words\r\n
        // xxx words words words\r\n"

        let mut reply_code: Option<SMTPReplyCode> = None;
        let reply_text = String::new();

        let lines: Vec<&str> = s.split("\r\n").collect();
        for line in lines {
            // Split the line into two parts, the code, and the text.
            // Return an error if the line is shorter than three characters
            // (the length of the code).
            let (code, text) = line.split_at_checked(3).ok_or(InvalidSyntax)?;

            match separator {
                None | Some(' ') => {
                    // Handle the single line case
                }
                Some('-') => {
                    // Handle the multiline case
                }
                Some(separator) => return Err(InvalidSeparator(separator)),
            }

            match &reply_code {
                None => reply_code = Some(code.try_into()?),
                // Make sure the reply code on this line is the same as the reply codes on previous lines
                Some(reply_code) => {
                    if reply_code != &code.try_into()? {
                        return Err(InvalidSyntax);
                    }
                }
            }
        }

        Ok(Self {
            code: reply_code.unwrap(), // TODO: Eliminate this unwrap
            text: reply_text,
        })
    }
}

/// Represents the three digit code in SMTP replies.
///
/// 2yz codes indicate positive completion (i.e. the request completed sucessfully)
///
/// 3yz codes indicate positive intermediate (i.e. the request is pending further information)
///
/// 4zy codes indicate transient failure (i.e. the request failed but can be reattempted)
///
/// 5zy codes indicate permanent failure (i.e. the client should not reattempt the request)
///
/// The doc comments for each type in this enum are the suggested reply text specfied in Section 4.4.2 of
/// [RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321#section-4.2.2)
#[derive(PartialEq)]
pub enum SMTPReplyCode {
    /// Syntax error, command unrecognized (This may include errors such
    /// as command line too long)
    Code500 = 500,

    /// Syntax error in parameters or arguments
    Code501 = 501,

    /// Command not implemented (see Section 4.2.4)
    Code502 = 502,

    /// Bad sequence of commands
    Code503 = 503,

    /// Command parameter not implemented
    Code504 = 504,

    /// System status, or system help reply
    Code211 = 211,

    /// Help message (Information on how to use the receiver or the
    /// meaning of a particular non-standard command; this reply is useful
    /// only to the human user)
    Code214 = 214,

    /// <domain> Service ready
    Code220 = 220,

    /// <domain> Service closing transmission channel
    Code221 = 221,

    /// <domain> Service not available, closing transmission channel
    /// (This may be a reply to any command if the service knows it must
    /// shut down)
    Code421 = 421,

    /// Requested mail action okay, completed
    Code250 = 250,

    /// User not local; will forward to <forward-path> (See Section 3.4 of [RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321))
    Code251 = 251,

    /// Cannot VRFY user, but will accept message and attempt delivery
    /// (See Section 3.5.3 of [RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321))
    Code252 = 252,

    /// Server unable to accommodate parameters
    Code455 = 455,

    /// MAIL FROM/RCPT TO parameters not recognized or not implemented
    Code555 = 555,

    /// Requested mail action not taken: mailbox unavailable (e.g.,
    /// mailbox busy or temporarily blocked for policy reasons)
    Code450 = 450,

    /// Requested action not taken: mailbox unavailable (e.g., mailbox
    /// not found, no access, or command rejected for policy reasons)
    Code550 = 550,

    /// Requested action aborted: error in processing
    Code451 = 451,

    /// User not local; please try <forward-path> (See Section 3.4 of [RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321))
    Code551 = 551,

    /// Requested action not taken: insufficient system storage
    Code452 = 452,

    /// Requested mail action aborted: exceeded storage allocation
    Code552 = 552,

    /// Requested action not taken: mailbox name not allowed (e.g.,
    /// mailbox syntax incorrect)
    Code553 = 553,

    /// Start mail input; end with <CRLF>.<CRLF>
    Code354 = 354,

    /// Transaction failed (Or, in the case of a connection-opening
    /// response, "No SMTP service here")
    Code554 = 554,
}

impl TryFrom<usize> for SMTPReplyCode {
    type Error = SMTPReplyParseError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&str> for SMTPReplyCode {
    type Error = SMTPReplyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let c = match value.parse::<usize>() {
            Ok(c) => c,
            Err(_) => return Err(SMTPReplyParseError::InvalidSyntax),
        };

        Self::try_from(c)
    }
}
