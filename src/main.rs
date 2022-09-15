use pop3::{POP3Connection};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pop3_listener = TcpListener::bind("192.168.0.138:110").await.unwrap();

    let handle = tokio::spawn(async move {
        loop {
            let (socket, _) = pop3_listener.accept().await.unwrap();

            println!("Accepted POP3 connection");
            let mut connection = POP3Connection::new(socket);

            if let Err(e) = connection.begin().await {
                println!("POP3 Connection ended with error: {}", e)
            }
        }
    });
    handle.await.unwrap();
}