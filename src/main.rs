use pop3::{POP3Connection, POP3Response, POP3Command::{self, *}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pop3_listener = TcpListener::bind("localhost:110").await.unwrap();

    loop {
        let (socket, _) = pop3_listener.accept().await.unwrap();
        println!("Accepted POP3 connection");
        let mut connection = POP3Connection::new(socket);

        connection.send_response(POP3Response::positive("good morning")).await.unwrap();
        println!("Greeted client");

        let command = connection.read_command().await.unwrap();
        println!("COMMAND: {:?}", command);
        match command {
            Username { username: _ } => {
                connection.send_response(POP3Response::positive("")).await.unwrap();
            }
            _ => connection.send_response(POP3Response::positive("")).await.unwrap(),
        };
    }
}