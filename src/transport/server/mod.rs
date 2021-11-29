use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

pub type Sending = (Vec<u8>, Option<Uuid>);

pub trait Error: 'static + std::error::Error + Clone + Sync + Send {}

#[derive(Clone, Debug)]
pub enum Events<E: Error> {
    Ready,
    Shutdown,
    Connected(Uuid),
    Disconnected(Uuid),
    Received(Uuid, Vec<u8>),
    Error(Option<Uuid>, String),
    ConnectionError(Option<Uuid>, E),
    ServerError(E),
}

impl<E: Error> std::fmt::Display for Events<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Events::Ready => String::from("Ready"),
            Events::Shutdown => String::from("Shutdown"),
            Events::Connected(uuid) => format!("Connected({})", uuid),
            Events::Disconnected(uuid) => format!("Disconnected({})", uuid),
            Events::Received(uuid, buf) => format!("Received({}, {} bytes)", uuid, buf.len()),
            Events::Error(uuid, err) => format!("Error({:?}): {}", uuid, err),
            Events::ConnectionError(uuid, err) => format!("ConnectionError({:?}): {}", uuid, err),
            Events::ServerError(err) => format!("ServerError({})", err),
        };
        write!(f, "{}", output)
    }
}

#[async_trait]
pub trait Control<E: Error>: Send + Clone {
    async fn shutdown(&self) -> Result<(), E>;
    async fn send(&self, buffer: Vec<u8>, client: Option<Uuid>) -> Result<(), E>;
    async fn disconnect(&self, client: Uuid) -> Result<(), E>;
    async fn disconnect_all(&self) -> Result<(), E>;
}

#[async_trait]
pub trait Impl<E: Error + Clone, C: Control<E> + Send + Clone>: Send {
    async fn listen(&mut self) -> Result<(), E>;
    fn observer(&mut self) -> Result<UnboundedReceiver<Events<E>>, E>;
    fn control(&self) -> C;
}
