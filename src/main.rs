mod command;
//use command::*;

mod connection;
//use connection::*;

mod response;
use response::*;

use bytes::Bytes;

#[tokio::main]
async fn main() {
    let b1 = Bytes::from("+OK fds\r\nfdsafdsa\r\nfdsa.");
    match POP3Response::try_from(b1) {
        Ok(_) => println!("Valid server response"),
        Err(POP3ResponseErr::InvalidStatus) => println!("Invalid status"),
        Err(POP3ResponseErr::IncompleteResponse) => println!("Waiting for rest of transmission")
    }
}
