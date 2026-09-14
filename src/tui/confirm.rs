// Destructive-action confirmations: explicit ask, explicit answer.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Pure model for edit/delete confirmations. Wiring into key handling is a
//! follow-up; the model guarantees no destructive path proceeds on ambiguity.

/// A pending yes/no question. Unresolved until `resolve` runs exactly once
/// semantics: first answer wins, later calls are ignored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confirm {
    prompt: String,
    answer: Option<bool>,
}

impl Confirm {
    pub fn ask(prompt: &str) -> Self {
        Self { prompt: prompt.into(), answer: None }
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn answer(&self) -> Option<bool> {
        self.answer
    }

    /// Record the answer; subsequent calls are ignored (no flip-flopping).
    pub fn resolve(&mut self, yes: bool) {
        if self.answer.is_none() {
            self.answer = Some(yes);
        }
    }
}
