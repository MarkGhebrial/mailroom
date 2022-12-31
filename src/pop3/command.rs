use crate::pop3::err::{POP3CommandErr, ParseError};
use bytes::{Bytes, BytesMut};
use POP3Command::*;
use POP3CommandErr::*;

#[derive(PartialEq, Debug)]
pub enum POP3Command {
    /// `QUIT`; If issued during the AUTHORIZATION  state, then close the
    /// connection. If issued during the TRANSACTION state, delete all
    /// messages marked as deleted.
    Quit,

    /// `STAT`; Return the number of messages and the their summative length
    /// in octets.
    Stat,

    /// `LIST`; List the messages in the mailbox and their sizes. If the
    /// `message_number` field is `Some`, then return the information for
    /// the specified message.
    List { message_number: Option<usize> },

    /// `RETR`; Retrieve the content of the requested message to the client.
    Retrieve { message_number: usize },

    /// `DELE`; Mark the specified message as deleted from the server.
    Delete { message_number: usize },

    /// `NOOP`; Reply with a positive response.
    NoOp,

    /// `RSET`; Unmark all messages that have been marked as deleted.
    Reset,

    /*
     * The above commands are required to be implemented by minimal POP3
     * implementations.
     *
     * The below commands are optional or extensions.
     */
    /// `CAPA`; List the extensions supported by this server.
    Capabilities,

    /// `TOP`; Get the header and first `n` lines of the specified message.
    Top { message_number: usize, n: usize },

    /// `UIDL`; Return the "unique-id listing" of the specified message. The
    /// UID can simply be a hash of the message contents.
    UniqueIDListing { message_number: Option<usize> },

    /// `USER`; Login to the specified mailbox. Only allowed in
    /// AUTHORIZATION state. Must be followed by `PASS` command.
    Username { username: Bytes },

    /// `PASS`; Supply the password to the mailbox. Only allowed in
    /// AUTHORIZATION state. Must be preceeded by `USER` command.
    Password { password: Bytes },

    /// `APOP`; A more secure authentication method.
    APop { username: Bytes, md5_digest: Bytes },
}

impl POP3Command {
    pub fn parse(bytes: Bytes) -> Result<Self, POP3CommandErr> {
        Self::try_from(bytes)
    }
}

// Convert Bytes to POP3Command
impl TryFrom<Bytes> for POP3Command {
    type Error = POP3CommandErr;

    /// Attempt to convert Bytes to a POP3Command. If the attempt fails,
    /// a POP3CommandErr will be returned.
    fn try_from(mut bytes: Bytes) -> Result<Self, Self::Error> {
        if bytes.len() < 2 {
            return Err(IncompleteResponse);
        }

        // Make sure the command ends with a CRLF pair
        if &bytes.slice(bytes.len() - 2..)[..] != b"\r\n" {
            return Err(IncompleteResponse);
        }
        bytes.truncate(bytes.len() - 2);

        // Parse the commands and its arguments into a vector
        let mut args = vec![];
        let mut last_space = 0;
        let iter = bytes.clone().into_iter();
        for (i, c) in iter.enumerate() {
            if c == b' ' {
                args.push(bytes.slice(last_space..i));
                last_space = i + 1;
            } else if i == bytes.len() - 1 {
                args.push(bytes.slice(last_space..));
            }
        }

        // A closure to parse ASCII arguments
        let bytes_arg = |index: usize| -> Result<Bytes, POP3CommandErr> {
            match args.get(index) {
                Some(n) => Ok(n.clone()),
                None => return Err(InvalidArguments),
            }
        };

        // A closure to parse a numeric argument
        let numeric_arg = |index: usize| -> Result<usize, POP3CommandErr> {
            match args.get(index) {
                Some(b) => match bytes_to_uint(&b) {
                    Ok(n) => return Ok(n),
                    Err(_) => return Err(InvalidArguments),
                },
                None => return Err(InvalidArguments),
            }
        };

        let command = match args.get(0) {
            Some(s) => match &uppercase(&s)[..] {
                b"QUIT" => Quit,
                b"STAT" => Stat,
                b"LIST" => List {
                    message_number: numeric_arg(1).ok(),
                },
                b"RETR" => Retrieve {
                    message_number: numeric_arg(1)?,
                },
                b"DELE" => Delete {
                    message_number: numeric_arg(1)?,
                },
                b"NOOP" => NoOp,
                b"RSET" => Reset,
                b"CAPA" => Capabilities,
                b"TOP" => Top {
                    message_number: numeric_arg(1)?,
                    n: numeric_arg(2)?,
                },
                b"UIDL" => UniqueIDListing {
                    message_number: numeric_arg(1).ok(),
                },
                b"USER" => Username {
                    username: bytes_arg(1)?,
                },
                b"PASS" => Password {
                    password: bytes_arg(1)?,
                },
                b"APOP" => APop {
                    username: bytes_arg(1)?,
                    md5_digest: bytes_arg(2)?,
                },
                _ => return Err(UnknownCommand(s.clone())),
            },
            None => return Err(InvalidSyntax),
        };

        Ok(command)
    }
}

/// Parse a Bytes into an unsigned integer. If it contains any non-numeric
/// characters, a ParseError will be returned
fn bytes_to_uint(bytes: &Bytes) -> Result<usize, ParseError> {
    if bytes.len() == 0 {
        return Err(ParseError);
    }

    let mut out: usize = 0;

    for (i, c) in bytes.into_iter().rev().enumerate() {
        let c = *c as char;
        let value = match c.to_digit(10) {
            Some(v) => v as usize,
            None => return Err(ParseError),
        };

        let base: usize = 10;
        out += value * base.pow((i).try_into().unwrap());
    }

    Ok(out)
}

fn uppercase(bytes: &Bytes) -> Bytes {
    let mut out = BytesMut::new();

    for c in bytes {
        let mut c = *c as char;
        c.make_ascii_uppercase();
        out.extend_from_slice(&[c as u8]);
    }

    out.into()
}

#[test]
fn parse_command() {
    assert_eq!(
        POP3Command::try_from(Bytes::from("NOOP 1 2 3 4\r\n")).unwrap(),
        NoOp
    );
    assert_eq!(
        POP3Command::try_from(Bytes::from("quit\r\n")).unwrap(),
        Quit
    );
    assert_eq!(
        POP3Command::try_from(Bytes::from("LIST 1\r\n")).unwrap(),
        List {
            message_number: Some(1)
        }
    );
    assert_eq!(
        POP3Command::try_from(Bytes::from("LIST \r\n")).unwrap(),
        List {
            message_number: None
        }
    );
    assert_eq!(
        POP3Command::try_from(Bytes::from("RETR 1234321\r\n")).unwrap(),
        Retrieve {
            message_number: 1234321
        }
    );
    assert_eq!(
        POP3Command::try_from(Bytes::from("TOP 5 10\r\n")).unwrap(),
        Top {
            message_number: 5,
            n: 10
        }
    );
    assert_eq!(
        POP3Command::try_from(Bytes::from("usEr mghebrial\r\n")).unwrap(),
        Username {
            username: "mghebrial".into()
        }
    );
    assert_eq!(
        POP3Command::try_from(Bytes::from(
            "APOP mrose c4c9334bac560ecc979e58001b3e22fb\r\n"
        ))
        .unwrap(),
        APop {
            username: "mrose".into(),
            md5_digest: "c4c9334bac560ecc979e58001b3e22fb".into(),
        }
    );
}

#[test]
fn uint_parse() {
    assert_eq!(bytes_to_uint(&Bytes::from("01230450")).unwrap(), 1230450);
    assert_eq!(bytes_to_uint(&Bytes::from("0")).unwrap(), 0);
    assert_eq!(bytes_to_uint(&Bytes::from("1127")).unwrap(), 1127);
    assert_eq!(bytes_to_uint(&Bytes::from("")).err().unwrap(), ParseError);
    assert_eq!(
        bytes_to_uint(&Bytes::from("Hell0, w0r1d!")).err().unwrap(),
        ParseError
    );
    assert_eq!(
        bytes_to_uint(&Bytes::from("-100")).err().unwrap(),
        ParseError
    );
    assert_eq!(
        bytes_to_uint(&Bytes::from("+100")).err().unwrap(),
        ParseError
    );
}

#[test]
fn bytes_uppercase() {
    assert_eq!(
        uppercase(&"abcdefghijklmnopqrstuvwxyz".into()),
        Bytes::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    );
}
