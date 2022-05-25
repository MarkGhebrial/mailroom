mod command;
//use command::*;

mod connection;
//use connection::*;

mod response;
use response::*;

use bytes::Bytes;

#[tokio::main]
async fn main() {
    let b1 = Bytes::from("+OK f");
    match POP3Response::try_from(b1) {
        Ok(r) => println!("Valid server response: {:?}", r.message),
        Err(POP3ResponseErr::InvalidSyntax) => println!("Invalid status"),
        Err(POP3ResponseErr::IncompleteResponse) => println!("Waiting for rest of transmission")
    }
}
