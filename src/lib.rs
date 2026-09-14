// rusteams — terminal client for Microsoft Teams via Microsoft Graph.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Library root: strict module boundaries.
//!
//! ```text
//! TUI -> app (use-cases/state) -> provider traits -> infra/graph (Microsoft Graph)
//! ```
//! The TUI must never call Microsoft Graph directly.

pub mod app;
pub mod cli;
pub mod config;
pub mod domain;
pub mod error;
pub mod infra;
pub mod provider;
pub mod sanitize;
pub mod telemetry;
pub mod tui;
