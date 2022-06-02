use pop3::{POP3Connection, POP3Response, POP3Command::{self, *}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pop3_listener = TcpListener::bind("localhost:110").await.unwrap();

    let handle = tokio::spawn(async move {
        loop {
            let (socket, _) = pop3_listener.accept().await.unwrap();

            println!("Accepted POP3 connection");
            let mut connection = POP3Connection::new(socket);

            connection.authenticate().await.unwrap();
            println!("Authenticated");

            connection.transaction().await.unwrap();
            println!("Finished");
        }
    });
    handle.await.unwrap();
}