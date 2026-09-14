// Scripted event-loop harness: key streams in, actions out, quit terminates.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! The live loop (Stage B) reads crossterm events from the real terminal and
//! feeds them through the same [`map_key`] + fold implemented here. This
//! harness substitutes a scripted slice so the whole flow is deterministic.

use super::keys::{KeyAction, map_key};
use crossterm::event::KeyEvent;

/// Fold a scripted key stream into actions. Stops after `Quit` — keys past it
/// are never processed. `_selected` names the chat context for future use
/// (open/navigation scoping lands with the live loop).
pub fn run_scripted(_selected: &str, keys: &[KeyEvent]) -> Vec<KeyAction> {
    let mut out = Vec::new();
    for key in keys {
        match map_key(*key) {
            Some(KeyAction::Quit) => {
                out.push(KeyAction::Quit);
                break;
            }
            Some(action) => out.push(action),
            None => {}
        }
    }
    out
}
