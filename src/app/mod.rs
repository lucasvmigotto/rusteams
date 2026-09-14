// Application state, commands/events, and connection lifecycle.
pub mod app_state;
pub mod connection;
pub mod reducer;
pub use reducer::{AppState, Command, Event};
