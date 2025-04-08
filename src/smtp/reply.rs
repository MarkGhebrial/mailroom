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
        // where "xxx" is the reply code
        //
        // Single line replies can also be of the form
        // "xxx\r\n"
        // Notice that there does not need to be a " " after the reply code.
        //
        // Multi line SMTP replies are in the form
        // "xxx-words words words\r\n
        // xxx-words words words\r\n
        // xxx words words words\r\n"
        // Notice the dashes between the reply codes and the line

        // TODO: Check that the response has a trailing CRLF. If not, return an incomplete response error.

        let mut reply_code: Option<SMTPReplyCode> = None;
        let mut reply_text = String::new();

        // let mut last_line_has_been_seen = false;
        let mut number_of_lines_parsed = 0;

        let lines: Vec<&str> = s.split("\r\n").collect();
        for line in lines.iter() {
            number_of_lines_parsed += 1;

            let code = line.get(..3).ok_or(InvalidSyntax)?;
            let separator = line.as_bytes().get(3);
            let text = line.get(4..);

            match &reply_code {
                None => reply_code = Some(code.try_into()?),
                // Make sure the reply code on this line is the same as the reply codes on previous lines
                Some(reply_code) => {
                    if reply_code != &code.try_into()? {
                        return Err(InvalidMultilineResponse);
                    }
                }
            }

            // Append the text on the current SMTP response line to the return string
            if let Some(s) = text {
                reply_text.push_str(s);
                reply_text.push_str("\r\n");
            }

            match separator {
                None | Some(b' ') => {
                    // A space between the code and the text means that this line is the last one, so we break the loop
                    break;
                }
                Some(b'-') => {
                    // If this is the last line, and the separator is '-'...
                    if number_of_lines_parsed == lines.len() {
                        // ... then the response is incomplete
                        return Err(IncompleteResponse);
                    }
                }
                Some(separator) => return Err(InvalidSeparator(*separator as char)),
            }
        }

        // If we didn't look at all the lines before finding one with a " " separator, then the response is invalid.
        if number_of_lines_parsed < lines.len() {
            return Err(InvalidMultilineResponse)
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
