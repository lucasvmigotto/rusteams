#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Degraded,
    #[default]
    Disconnected,
    Reconnecting,
    Syncing,
}
