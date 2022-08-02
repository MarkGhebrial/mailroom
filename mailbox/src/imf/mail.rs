use bytes::Bytes;
use crate::err::MailParseError;
use std::collections::HashMap;

/// Represents an email message.
pub struct Mail {
    headers: HashMap<String, String>,
    content: String,
}

impl Mail {
    pub fn new(headers: HashMap<String, String>, content: String) -> Self {
        Self { headers, content }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// Return the length of the message in octets.
    pub fn content_len(&self) -> usize {
        self.content.len()
    }
}

impl TryFrom<Bytes> for Mail {
    type Error = MailParseError;

    fn try_from(mut bytes: Bytes) -> Result<Self, Self::Error> {
        Err(MailParseError)
    }
}