//pub mod err;

mod mailbox;
pub use crate::database::mailbox::*;

mod models;
use models::{prelude::*, *};

pub mod user_database;
