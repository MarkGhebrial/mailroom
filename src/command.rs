use bytes::Bytes;
use crate::err::POP3CommandErr;

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

    // `APOP`; A more secure authentication method.
    APop { username: Bytes, md5_digest: Bytes },
}

impl POP3Command {
    pub fn parse(bytes: Bytes) -> Result<Self, POP3CommandErr> {
        Self::try_from(bytes)
    }
}

// Convert Bytes to POP3Response
impl TryFrom<Bytes> for POP3Command {
    type Error = POP3CommandErr;

    /// Attempt to convert Bytes to a POP3Response. If the attempt fails,
    /// a POP3ResponseErr will be returned.
    fn try_from(_bytes: Bytes) -> Result<Self, Self::Error> {
        
        Ok(POP3Command::NoOp)
    }
}