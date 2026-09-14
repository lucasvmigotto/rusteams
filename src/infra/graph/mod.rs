pub mod client;
pub mod html;
pub use crate::domain::RichSegment;
pub use client::{
    GraphClient, RetryHint, chats_url, classify_status, messages_since_url, messages_url,
};
pub use html::render_html_body;
