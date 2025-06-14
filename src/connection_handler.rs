use log::{info, warn};
use std::error::Error;
use std::future::Future;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

use crate::CONFIG;

/// A trait for structs that handle incoming TCP connections.
pub trait ConnectionHandler
where
    Self: Send + Sized + 'static,
{
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
    ///
    /// TODO: Think of a more descriptive function name?
    async fn start_listening(port: u16) -> JoinHandle<()> {
        // TODO: Consider not using `unwrap()`. Are the errors worth crashing the whole server?
        let listener = TcpListener::bind((CONFIG.bind_address, port))
            .await
            .unwrap();

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
                            "{} connection closed with error: {}",
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
    /// The return type is ugly because traits don't directly support async functions.
    fn begin(&mut self) -> impl Future<Output = Result<(), Box<dyn Error>>> + Send;
}
