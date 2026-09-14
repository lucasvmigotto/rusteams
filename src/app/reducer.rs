// Application commands, domain events, and pure reducers.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! UI intent enters as [`Command`]; state changes are recorded as [`Event`].
//! [`AppState::apply`] is a pure reducer: same state + event → same state.

use super::app_state::ConnectionState;
use super::connection::{ConnectionEvent, transition};
use crate::domain::{Chat, ChatMessage};

/// User or system intent. Carries no secrets and performs no I/O.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    SelectChat {
        chat_id: String,
    },
    ChatsLoaded {
        chats: Vec<Chat>,
    },
    MessagesLoaded {
        chat_id: String,
        messages: Vec<ChatMessage>,
    },
    MessageSent {
        message: ChatMessage,
    },
    MessageReceived {
        message: ChatMessage,
    },
    /// Server echo for an optimistic send: swap the temp-id message for the
    /// confirmed one, or append when no temp entry exists (e.g. after restart).
    MessageConfirmed {
        temp_id: String,
        message: ChatMessage,
    },
    SyncDiffApplied {
        chat_id: String,
        changed: Vec<ChatMessage>,
        deleted: Vec<String>,
    },
    ConnectionEvent(ConnectionEvent),
}

/// Something that happened, recorded for state evolution and tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    ChatSelected { chat_id: String },
    ChatsReplaced { count: usize },
    MessagesReplaced { chat_id: String, count: usize },
    MessageAppended { id: String },
    MessageUpserted { id: String },
    MessagesDeleted { ids: Vec<String> },
    ConnectionChanged { state: ConnectionState },
    Rejected { reason: &'static str },
}

/// Central UI/domain state. All evolution goes through [`AppState::apply`].
#[derive(Debug, Default)]
pub struct AppState {
    pub chats: Vec<Chat>,
    pub messages: Vec<ChatMessage>,
    pub selected_chat: Option<String>,
    pub connection: ConnectionState,
}

impl AppState {
    /// Pure reducer: apply a command, mutate state, return resulting events.
    pub fn apply(&mut self, cmd: Command) -> Vec<Event> {
        match cmd {
            Command::SelectChat { chat_id } => {
                self.selected_chat = Some(chat_id.clone());
                vec![Event::ChatSelected { chat_id }]
            }
            Command::ChatsLoaded { chats } => {
                let count = chats.len();
                self.chats = chats;
                vec![Event::ChatsReplaced { count }]
            }
            Command::MessagesLoaded { chat_id, messages } => {
                let count = messages.len();
                self.messages.retain(|m| m.chat_id != chat_id);
                self.messages.extend(messages);
                crate::domain::order_messages(&mut self.messages);
                vec![Event::MessagesReplaced { chat_id, count }]
            }
            Command::MessageSent { message } => {
                if message.body.trim().is_empty() {
                    return vec![Event::Rejected { reason: "empty body" }];
                }
                let id = message.id.clone();
                self.messages.push(message);
                crate::domain::order_messages(&mut self.messages);
                vec![Event::MessageAppended { id }]
            }
            Command::MessageReceived { message } => {
                let id = message.id.clone();
                match self.messages.iter_mut().find(|m| m.id == id) {
                    Some(existing) => {
                        *existing = message;
                        crate::domain::order_messages(&mut self.messages);
                        vec![Event::MessageUpserted { id }]
                    }
                    None => {
                        self.messages.push(message);
                        crate::domain::order_messages(&mut self.messages);
                        vec![Event::MessageAppended { id }]
                    }
                }
            }
            Command::MessageConfirmed { temp_id, message } => {
                let id = message.id.clone();
                match self.messages.iter_mut().find(|m| m.id == temp_id) {
                    Some(slot) => {
                        *slot = message;
                        crate::domain::order_messages(&mut self.messages);
                        vec![Event::MessageUpserted { id }]
                    }
                    None => {
                        self.messages.push(message);
                        crate::domain::order_messages(&mut self.messages);
                        vec![Event::MessageAppended { id }]
                    }
                }
            }
            Command::SyncDiffApplied { chat_id, changed, deleted } => {
                let mut events = Vec::new();
                for m in changed {
                    debug_assert_eq!(m.chat_id, chat_id);
                    let id = m.id.clone();
                    match self.messages.iter_mut().find(|e| e.id == id) {
                        Some(existing) => {
                            *existing = m;
                            events.push(Event::MessageUpserted { id });
                        }
                        None => {
                            self.messages.push(m);
                            events.push(Event::MessageAppended { id });
                        }
                    }
                }
                if !deleted.is_empty() {
                    self.messages.retain(|m| !deleted.contains(&m.id));
                    events.push(Event::MessagesDeleted { ids: deleted });
                }
                crate::domain::order_messages(&mut self.messages);
                events
            }
            Command::ConnectionEvent(ev) => {
                self.connection = transition(self.connection, ev);
                vec![Event::ConnectionChanged { state: self.connection }]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn msg(id: &str) -> ChatMessage {
        let t = Utc.timestamp_opt(10, 0).unwrap();
        ChatMessage {
            id: id.into(),
            chat_id: "c1".into(),
            created: t,
            modified: t,
            sender: "alice".into(),
            body: "hi".into(),
            reply_to_id: None,
            reactions: vec![],
            mentions: vec![],
            is_read: false,
        }
    }

    #[test]
    fn select_chat_records_selection() {
        let mut s = AppState::default();
        let ev = s.apply(Command::SelectChat { chat_id: "c1".into() });
        assert_eq!(ev, vec![Event::ChatSelected { chat_id: "c1".into() }]);
        assert_eq!(s.selected_chat.as_deref(), Some("c1"));
    }

    #[test]
    fn sent_message_appends_and_rejects_empty_body() {
        let mut s = AppState::default();
        let ev = s.apply(Command::MessageSent { message: msg("m1") });
        assert_eq!(ev, vec![Event::MessageAppended { id: "m1".into() }]);
        assert_eq!(s.messages.len(), 1);

        let mut empty = msg("m2");
        empty.body = "   ".into();
        let ev = s.apply(Command::MessageSent { message: empty });
        assert_eq!(ev, vec![Event::Rejected { reason: "empty body" }]);
        assert_eq!(s.messages.len(), 1);
    }

    #[test]
    fn received_message_upserts_by_id() {
        let mut s = AppState::default();
        s.apply(Command::MessageReceived { message: msg("m1") });
        let mut edited = msg("m1");
        edited.body = "edited".into();
        let ev = s.apply(Command::MessageReceived { message: edited });
        assert_eq!(ev, vec![Event::MessageUpserted { id: "m1".into() }]);
        assert_eq!(s.messages.len(), 1);
        assert_eq!(s.messages[0].body, "edited");
    }

    #[test]
    fn connection_events_drive_state_machine() {
        let mut s = AppState::default();
        assert_eq!(s.connection, ConnectionState::Disconnected);
        let ev = s.apply(Command::ConnectionEvent(ConnectionEvent::ReconnectAttempt));
        assert_eq!(s.connection, ConnectionState::Reconnecting);
        assert_eq!(ev, vec![Event::ConnectionChanged { state: ConnectionState::Reconnecting }]);
    }

    #[test]
    fn reducer_is_deterministic() {
        let run = || {
            let mut s = AppState::default();
            s.apply(Command::SelectChat { chat_id: "c1".into() });
            s.apply(Command::MessageReceived { message: msg("m1") });
            (s.selected_chat.clone(), s.messages.len())
        };
        assert_eq!(run(), run());
    }

    #[test]
    fn messages_loaded_replaces_chat_scope_only() {
        let mut s = AppState::default();
        let mut other = msg("other");
        other.chat_id = "c2".into();
        s.apply(Command::MessageReceived { message: other });
        s.apply(Command::MessagesLoaded { chat_id: "c1".into(), messages: vec![msg("m1")] });
        assert!(s.messages.iter().any(|m| m.id == "other"));
        assert!(s.messages.iter().any(|m| m.id == "m1"));
    }

    #[test]
    fn confirmed_send_swaps_temp_for_real_message() {
        let mut s = AppState::default();
        let mut temp = msg("temp-1");
        temp.body = "draft".into();
        s.apply(Command::MessageSent { message: temp });
        let mut real = msg("m-9");
        real.body = "draft".into();
        let ev = s.apply(Command::MessageConfirmed { temp_id: "temp-1".into(), message: real });
        assert_eq!(ev, vec![Event::MessageUpserted { id: "m-9".into() }]);
        assert_eq!(s.messages.len(), 1);
        assert_eq!(s.messages[0].id, "m-9");
    }

    #[test]
    fn confirmed_send_without_temp_appends() {
        let mut s = AppState::default();
        let ev = s.apply(Command::MessageConfirmed { temp_id: "gone".into(), message: msg("m-9") });
        assert_eq!(ev, vec![Event::MessageAppended { id: "m-9".into() }]);
        assert_eq!(s.messages.len(), 1);
    }
}
