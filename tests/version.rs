// Version surface: package version reaches users via CLI and TUI status bar,
// always derived from Cargo metadata, never hard-coded.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::app::AppState;
use rusteams::tui::build_read_view;

#[test]
fn package_version_is_present_and_semver_shaped() {
    let v = env!("CARGO_PKG_VERSION");
    assert!(!v.is_empty(), "version must exist");
    let parts: Vec<&str> = v.split('.').collect();
    assert_eq!(parts.len(), 3, "version follows X.Y.Z, got: {v}");
    assert!(parts.iter().all(|p| p.chars().all(|c| c.is_ascii_alphanumeric() || *c == '-')));
}

#[test]
fn status_bar_carries_the_package_version() {
    let view = build_read_view(&AppState::default());
    let expected = format!("rusteams {}", env!("CARGO_PKG_VERSION"));
    assert!(
        view.status.contains(&expected),
        "status bar must show `{expected}`, got: {}",
        view.status
    );
}
