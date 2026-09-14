#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Degraded,
    Disconnected,
    Reconnecting,
    Syncing,
}
