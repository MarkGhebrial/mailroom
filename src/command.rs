use bytes::{Bytes, BytesMut};
use crate::err::{POP3CommandErr, ParseError};
use POP3CommandErr::*;
use POP3Command::*;

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
     * The below commands are optional.
     */

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

    /// Attempt to convert Bytes to a POP3Response. If the attempt fails,
    /// a POP3ComnmandErr will be returned.
    fn try_from(mut bytes: Bytes) -> Result<Self, Self::Error> {

        // Make sure the command ends with a CRLF pair
        if &bytes.slice(bytes.len() - 2..)[..] != b"\r\n" {
            return Err(IncompleteResponse)
        }
        bytes.truncate(bytes.len() - 2);

        // Parse the commands and its arguments into a vector
        let mut args = vec![];
        let mut last_space = 0;
        let iter = bytes.clone().into_iter();
        for (i, c) in iter.enumerate() {
            if c == b' ' || i == bytes.len() - 1 {
                args.push(bytes.slice(last_space..i));
                last_space = i+1;
            }
        }
        
        println!("{:?}", args);

        let command = match args.get(0) {
            Some(s) => match &uppercase(&s)[..] {
                b"QUIT" => Quit,
                b"STAT" => Stat,
                b"LIST" => {
                    let message_number = match args.get(1) {
                        Some(bytes) => {
                            let mut string = String::new();
                            for b in bytes {
                                string.push(*b as char);
                            }
                            None // TODO: incomplete!
                        },
                        None => None,
                    };
                    List { message_number }
                },
                b"RETR" => NoOp,
                b"DELE" => NoOp,
                b"NOOP" => NoOp,
                b"RSET" => Reset,
                b"TOP"  => NoOp,
                b"UIDL" => UniqueIDListing { message_number: None },
                b"USER" => Username { username: match args.get(1) {
                    Some(n) => n.clone(),
                    None => return Err(InvalidArguments),
                }},
                b"PASS" => Password { password: match args.get(1) {
                    Some(n) => n.clone(),
                    None => return Err(InvalidArguments),
                }},
                b"APOP" => APop {
                    username: match args.get(1) {
                        Some(n) => n.clone(),
                        None => return Err(InvalidArguments),
                    },
                    md5_digest: match args.get(2) {
                        Some(n) => n.clone(),
                        None => return Err(InvalidArguments),
                    }
                },
                _ => return Err(UnknownCommand(s.clone()))
            },
            None => return Err(InvalidSyntax),
        };
        
        Ok(command)
    }
}

fn bytes_to_uint(bytes: &Bytes) -> Result<usize, ParseError> {
    if bytes.len() == 0 { return Err(ParseError) }

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
    POP3Command::try_from(Bytes::from("NOOP 1 2 3 4\r\n")).unwrap();
}

#[test]
fn uint_parse() {
    assert_eq!(bytes_to_uint(&Bytes::from("01230450")).unwrap(), 1230450);
    assert_eq!(bytes_to_uint(&Bytes::from("0")).unwrap(), 0);
    assert_eq!(bytes_to_uint(&Bytes::from("1127")).unwrap(), 1127);
    assert_eq!(bytes_to_uint(&Bytes::from("")).err().unwrap(), ParseError);
    assert_eq!(bytes_to_uint(&Bytes::from("Hell0, w0r1d!")).err().unwrap(), ParseError);
    assert_eq!(bytes_to_uint(&Bytes::from("-100")).err().unwrap(), ParseError);
    assert_eq!(bytes_to_uint(&Bytes::from("+100")).err().unwrap(), ParseError);
}