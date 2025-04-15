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
            return Err(InvalidMultilineResponse);
        }

        Ok(Self {
            code: reply_code.unwrap(), // TODO: Eliminate this unwrap
            text: reply_text,
        })
    }
}

/// Represents the three digit code in SMTP replies.
/// The u16 member holds the last two digits of the code.
///
/// 2yz codes indicate positive completion (i.e. the request completed successfully)
///
/// 3yz codes indicate positive intermediate (i.e. the request is pending further information)
///
/// 4zy codes indicate transient failure (i.e. the request failed but can be reattempted)
///
/// 5zy codes indicate permanent failure (i.e. the client should not reattempt the request)
///
/// The suggested reply texts for each reply code are specfied in Section 4.4.2 of
/// [RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321#section-4.2.2)
#[derive(PartialEq, Debug)]
pub enum SMTPReplyCode {
    /// Indicates positive completion (i.e. the request completed successfully)
    TwoHundredCode(u16),

    /// Indicates positive intermediate (i.e. the request is pending further information)
    ThreeHundredCode(u16),

    /// Indicates transient failure (i.e. the request failed but can be reattempted)
    FourHundredCode(u16),

    /// Indicates permanent failure (i.e. the client should not reattempt the request)
    FiveHundredCode(u16),
}

impl TryFrom<u16> for SMTPReplyCode {
    type Error = SMTPReplyParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use SMTPReplyCode::*;

        match value / 100 {
            2 => Ok(TwoHundredCode(value - 200)),
            3 => Ok(ThreeHundredCode(value - 300)),
            4 => Ok(FourHundredCode(value - 400)),
            5 => Ok(FiveHundredCode(value - 500)),
            _ => Err(SMTPReplyParseError::InvalidResponseCode(value.into())),
        }
    }
}

impl TryFrom<&str> for SMTPReplyCode {
    type Error = SMTPReplyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let c = match value.parse::<u16>() {
            Ok(c) => c,
            Err(_) => return Err(SMTPReplyParseError::InvalidSyntax),
        };

        Self::try_from(c)
    }
}

#[test]
fn u16_to_smtp_reply_code() {
    let tests: Vec<(u16, Result<SMTPReplyCode, SMTPReplyParseError>)> = vec![
        (200, Ok(SMTPReplyCode::TwoHundredCode(0))),
        (300, Ok(SMTPReplyCode::ThreeHundredCode(0))),
        (400, Ok(SMTPReplyCode::FourHundredCode(0))),
        (500, Ok(SMTPReplyCode::FiveHundredCode(0))),
        (220, Ok(SMTPReplyCode::TwoHundredCode(20))),
        (354, Ok(SMTPReplyCode::ThreeHundredCode(54))),
        (504, Ok(SMTPReplyCode::FiveHundredCode(4))),
        (4, Err(SMTPReplyParseError::InvalidResponseCode(4))),
        (0, Err(SMTPReplyParseError::InvalidResponseCode(0))),
        (31, Err(SMTPReplyParseError::InvalidResponseCode(31))),
        (123, Err(SMTPReplyParseError::InvalidResponseCode(123))),
        (680, Err(SMTPReplyParseError::InvalidResponseCode(680))),
    ];

    for test in tests {
        assert_eq!(test.0.try_into(), test.1);
    }
}

#[test]
fn str_to_smtp_reply_code() {
    todo!()
}

fn parse_smtp_reply() {
    todo!()
}