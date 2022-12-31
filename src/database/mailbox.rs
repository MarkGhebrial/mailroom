/*use std::collections::HashMap;

use crate::imf::Mail;
use super::err::MailboxError;
use super::err::MailboxError::*;

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
            db: vec![]
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
}*/
