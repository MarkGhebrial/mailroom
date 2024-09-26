use std::error::Error;
use std::str::FromStr;

use bytes::Bytes;
use email_address::EmailAddress;

pub enum SMTPCommand {
    // TODO: source routes
    /// `MAIL FROM:`; Initiate transaction and specify the address of the
    /// sender
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.2
    MailFrom { sender: EmailAddress },

    /// `RCPT TO:`; Specify a recipient of the message. There's some extra
    /// nonsense I have to do here to get rid of "source roots" (see RFC 5321.4.1.1.3)
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.3
    Recipient { recipient: EmailAddress },

    /// `DATA`; Indicates that mail data begins on the next line
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.4
    Data { data: String },

    /// `RSET`; Abort the current mail transaction
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.5
    Reset,

    // Verify and expand are optional and must be listed in an EHLO response (RFC 2821 3.5.2)
    /// `VRFY`; Verify that the user exists on the server
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.6
    Verify { address: String },

    /// `EXPN`; List all the recipients on a mailing list
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.7
    Expand { mailing_list: String },

    /// `HELP`; Send helpful information to the client (not required to be implemented)
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.8
    Help,

    /// `NOOP`; Do nothing. May be followed by a string argument (make sure that
    /// doesn't crash the server)
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.9
    Noop,

    /// `QUIT`; End the transaction. Server MUST reply with "221 OK"
    ///
    /// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.10
    Quit,
}

#[derive(PartialEq, Debug)]
pub enum SMTPCommandParseError {
    IncompleteCommand,
    InvalidArguments,
    InvalidCommand,
}

impl Error for SMTPCommandParseError {}

impl TryFrom<&str> for SMTPCommand {
    type Error = SMTPCommandParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = input.trim();
        let words: Vec<&str> = input.split_whitespace().collect();

        match words.get(0) {
            None => return Err(SMTPCommandParseError::IncompleteCommand),
            Some(s) => match s.to_ascii_uppercase().as_str() {
                "MAIL" => {
                    return Ok(SMTPCommand::MailFrom {
                        sender: EmailAddress::from_str("TODO@TODO.com").unwrap(),
                    })
                }
                "RCPT" => {
                    todo!()
                }
                "DATA" => {
                    // Make sure the input ends in "CRLF.CRLF"
                    if &input[input.len() - 5..input.len()] != "\r\n.\r\n" {
                        return Err(SMTPCommandParseError::IncompleteCommand);
                    }

                    let data = input[9..].to_string();

                    return Ok(SMTPCommand::Data { data });
                }
                "RSET" => {
                    return Ok(SMTPCommand::Reset); // TODO: verify that these commands are complete before returning Ok
                }
                "VRFY" => {
                    return Ok(SMTPCommand::Verify {
                        address: "TODO TODO TODO".to_string(),
                    });
                }
                "EXPN" => {
                    todo!()
                }
                "HELP" => return Ok(SMTPCommand::Help),
                "NOOP" => {
                    return Ok(SMTPCommand::Noop);
                }
                "QUIT" => return Ok(SMTPCommand::Quit),
                _ => return Err(SMTPCommandParseError::InvalidCommand),
            },
        }

        panic!("Send Help");
    }
}
