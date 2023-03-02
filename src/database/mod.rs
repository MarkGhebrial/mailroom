//pub mod err;

mod mailbox;
pub use crate::database::mailbox::*;

mod models;
pub use models::{prelude::*, *};

pub mod user_database;
