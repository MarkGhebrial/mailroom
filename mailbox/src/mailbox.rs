use std::collections::HashMap;

use super::Mail;
use crate::err::MailboxError;
use MailboxError::*;

pub trait MailBox {
    fn get_messages(&self) -> HashMap<usize, &Mail>;
    fn delete_message(&mut self, message: usize) -> Result<(), MailboxError>;
}

pub struct TestMailBox {
    db: Vec<Mail>,
}

impl TestMailBox {
    pub fn new() -> Self {
        Self {
            db: vec![Mail::new("This is a plaintext, non MIME message".into())]
        }
    }
}

impl MailBox for TestMailBox {
    fn get_messages(&self) -> HashMap<usize, &Mail> {
        let mut out = HashMap::new();
        for (i, m) in self.db.iter().enumerate() {
            out.insert(i, m.clone());
        }
        out
    }

    fn delete_message(&mut self, message: usize) -> Result<(), MailboxError> {
        if message >= self.db.len() {
            return Err(MessageDoesNotExist);
        }

        self.db.remove(message);

        Ok(())
    }
}