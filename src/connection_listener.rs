use log::{info, warn};
use std::error::Error;
use std::future::Future;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

pub trait ConnectionHandler {
    /// Return the name of the protocol this handler implements. Used for
    /// logging messages about connection status.
    ///
    /// Examples: "POP3", "SMTP", "IMAP", "HTTP", etc.
    fn protocol_name() -> String;

    /// Bind to a port and start listening for connections. The default
    /// implementation handles each connection separately in its own tokio
    /// thread.
    ///
    /// Returns a handle to the listener thread. The handle only joins when the
    /// connection listener encounters a fatal error.
    async fn bind(port: u16) -> JoinHandle<()>
    where
        Self: ConnectionHandler + Send + Sized + 'static,
    {
        // TODO: Evaluate the unwraps in this function. Are the errors worth crashing the whole server?
        let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();

        let handle = tokio::spawn(async move {
            loop {
                let (socket, addr) = listener.accept().await.unwrap();

                info!(
                    "Accepted {} connection from {}",
                    Self::protocol_name(),
                    addr
                );
                let mut connection = Self::from_stream(socket);

                tokio::spawn(async move {
                    match connection.begin().await {
                        Ok(()) => {}
                        Err(e) => warn!(
                            "{} Connection closed with error: {}",
                            Self::protocol_name(),
                            e
                        ),
                    };
                });
            }
        });

        handle
    }

    /// Create a connection handler from a tokio `TcpStream`. The handler has
    /// ownership of the stream.
    fn from_stream(stream: TcpStream) -> Self;

    /// Async function. Begin the transaction with the client.
    ///
    /// The return type is ugly because the
    fn begin(&mut self) -> impl Future<Output = Result<(), impl Error>> + Send;
}
