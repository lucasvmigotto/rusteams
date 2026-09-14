// User presence model.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

/// Normalized availability. Unknown Graph values map to `Unknown` — never panic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Availability {
    Available,
    Busy,
    Away,
    BeRightBack,
    DoNotDisturb,
    Offline,
    PresenceUnknown,
    Unknown(String),
}

/// Normalized user presence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presence {
    pub user_id: String,
    pub availability: Availability,
    pub activity: String,
}

/// Parse a Graph `availability` string. Case-insensitive; unknown → `Unknown`.
pub fn parse_availability(s: &str) -> Availability {
    match s.to_ascii_lowercase().as_str() {
        "available" => Availability::Available,
        "busy" => Availability::Busy,
        "away" => Availability::Away,
        "berightback" => Availability::BeRightBack,
        "donotdisturb" => Availability::DoNotDisturb,
        "offline" => Availability::Offline,
        "presenceunknown" => Availability::PresenceUnknown,
        _ => Availability::Unknown(s.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values_parse() {
        assert_eq!(parse_availability("Available"), Availability::Available);
        assert_eq!(parse_availability("busy"), Availability::Busy);
        assert_eq!(parse_availability("PresenceUnknown"), Availability::PresenceUnknown);
    }

    #[test]
    fn unknown_values_are_fail_safe() {
        assert_eq!(
            parse_availability("InACustomState"),
            Availability::Unknown("InACustomState".into())
        );
        assert_eq!(parse_availability(""), Availability::Unknown("".into()));
    }
}
