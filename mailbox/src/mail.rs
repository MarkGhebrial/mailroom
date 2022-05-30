use bytes::Bytes;

pub struct Mail {
    content: Bytes,
}

impl Mail {
    pub fn new(content: Bytes) -> Self {
        Self { content }
    }

    pub fn content(&self) -> Bytes {
        self.content.clone()
    }

    /// Return the length of the message in octets.
    pub fn content_len(&self) -> usize {
        self.content.len()
    }
}