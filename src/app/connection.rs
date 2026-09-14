// Connection lifecycle state machine + retry policy.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use super::app_state::ConnectionState;
use std::time::Duration;

/// Events that drive the connection state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionEvent {
    Success,
    SoftFailure,
    HardFailure,
    ReconnectAttempt,
    SyncComplete,
    GiveUp,
}

/// Pure state transition function (deterministic, fully tested).
pub fn transition(state: ConnectionState, ev: ConnectionEvent) -> ConnectionState {
    use ConnectionEvent as E;
    use ConnectionState as S;
    match (state, ev) {
        (S::Connected, E::SoftFailure) => S::Degraded,
        (S::Connected, E::HardFailure) => S::Disconnected,
        (S::Degraded, E::Success) => S::Connected,
        (S::Degraded, E::SoftFailure) => S::Degraded,
        (S::Degraded, E::HardFailure) => S::Disconnected,
        (S::Disconnected, E::ReconnectAttempt) => S::Reconnecting,
        (S::Reconnecting, E::SyncComplete) => S::Syncing,
        (S::Reconnecting, E::HardFailure) => S::Disconnected,
        (S::Reconnecting, E::GiveUp) => S::Disconnected,
        (S::Syncing, E::Success) => S::Connected,
        (S::Syncing, E::HardFailure) => S::Reconnecting,
        (S::Syncing, E::SoftFailure) => S::Degraded,
        (s, _) => s,
    }
}

/// Deterministic exponential backoff ceiling: `base * 2^attempt`, capped.
/// `jitter_ms` (0..=1000, supplied by the caller from any RNG) spreads retries.
pub fn backoff_delay(attempt: u32, base: Duration, cap: Duration, jitter_ms: u64) -> Duration {
    let exp = base.saturating_mul(1 << attempt.min(10));
    let capped = exp.min(cap);
    capped + Duration::from_millis(jitter_ms.min(1_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_and_recovery() {
        use ConnectionEvent as E;
        use ConnectionState as S;
        assert_eq!(transition(S::Connected, E::SoftFailure), S::Degraded);
        assert_eq!(transition(S::Degraded, E::Success), S::Connected);
        assert_eq!(transition(S::Connected, E::HardFailure), S::Disconnected);
        assert_eq!(transition(S::Disconnected, E::ReconnectAttempt), S::Reconnecting);
        assert_eq!(transition(S::Reconnecting, E::SyncComplete), S::Syncing);
        assert_eq!(transition(S::Syncing, E::Success), S::Connected);
    }

    #[test]
    fn sync_failure_returns_to_reconnecting() {
        use ConnectionEvent as E;
        use ConnectionState as S;
        assert_eq!(transition(S::Syncing, E::HardFailure), S::Reconnecting);
    }

    #[test]
    fn backoff_grows_exponentially_and_caps() {
        let base = Duration::from_secs(1);
        let cap = Duration::from_secs(60);
        assert_eq!(backoff_delay(0, base, cap, 0), Duration::from_secs(1));
        assert_eq!(backoff_delay(3, base, cap, 0), Duration::from_secs(8));
        assert_eq!(backoff_delay(20, base, cap, 0), Duration::from_secs(60));
    }

    #[test]
    fn jitter_is_bounded() {
        let d = backoff_delay(1, Duration::from_secs(2), Duration::from_secs(60), 5_000);
        assert_eq!(d, Duration::from_secs(4) + Duration::from_secs(1));
    }
}
