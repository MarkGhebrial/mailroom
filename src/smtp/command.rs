use std::str::FromStr;

use bytes::Bytes;
use email_address::EmailAddress;

pub enum SMTPCommand {
    // TODO: source routes
    /// `MAIL FROM:`; Initiate transaction and specify the address of the
    /// sender
    MailFrom {
        sender: EmailAddress,
    },

    /// `RCPT:`: Specify a recipient of the message. There's some extra 
    /// nonsense I have to do here to get rid of "source roots" (see RFC 5321.4.1.1.3)
    Recipient { recipient: EmailAddress },

    /// 
    Data,

    // Verify and expand are optional and must be listed in an EHLO response (RFC 2821 3.5.2)
    /// `VRFY`; Verify that the user exists on the server
    Verify {
        address: String,
    },

    /// `EXPN`; List all the recipients on a mailing list
    Expand {
        mailing_list: String,
    },
}

pub struct SMTPCommandParseError;

impl TryFrom<&str> for SMTPCommand {
    type Error = SMTPCommandParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = input.trim();
        let words: Vec<&str> = input.split_whitespace().collect();

        match words.get(0) {
            None => return Err(SMTPCommandParseError),
            Some(s) => match s.to_ascii_uppercase().as_str() {
                "MAIL" => {
                    return Ok(SMTPCommand::MailFrom {
                        sender: EmailAddress::from_str("TODO@TODO.com").unwrap(),
                    })
                }
                _ => return Err(SMTPCommandParseError),
            },
        }

        Err(SMTPCommandParseError)
    }
}
