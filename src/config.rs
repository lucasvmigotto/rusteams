// Layered configuration: CLI args > env vars > TOML file > safe defaults.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Secrets (client secrets, tokens) are NEVER read from this config.
//! Only non-secret identifiers (client-id, tenant-id) live here.

use crate::error::AppError;
use serde::{Deserialize, Serialize};

/// Non-secret application configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    /// Entra public-client application (client) ID — BYO-app model.
    pub client_id: Option<String>,
    /// Tenant ID or `organizations` for multi-tenant work/school sign-in.
    pub tenant_id: String,
    /// Graph base URL (override for national clouds / tests).
    pub graph_base_url: String,
    /// Poll interval for active-chat refresh, in seconds.
    pub poll_interval_secs: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            client_id: None,
            tenant_id: "organizations".to_string(),
            graph_base_url: "https://graph.microsoft.com/v1.0".to_string(),
            poll_interval_secs: 15,
        }
    }
}

impl AppConfig {
    /// Parse a TOML fragment (the `[rusteams]` table or bare keys) over defaults.
    pub fn from_toml_str(s: &str) -> Result<Self, AppError> {
        let mut cfg = Self::default();
        if s.trim().is_empty() {
            return Ok(cfg);
        }
        let v: toml::Value = s.parse().map_err(|e| AppError::Config(format!("bad TOML: {e}")))?;
        let table = v.get("rusteams").unwrap_or(&v);
        if let Some(t) = table.get("tenant_id").and_then(|x| x.as_str()) {
            cfg.tenant_id = t.to_string();
        }
        if let Some(c) = table.get("client_id").and_then(|x| x.as_str()) {
            cfg.client_id = Some(c.to_string());
        }
        if let Some(u) = table.get("graph_base_url").and_then(|x| x.as_str()) {
            cfg.graph_base_url = u.to_string();
        }
        if let Some(p) = table.get("poll_interval_secs").and_then(|x| x.as_integer()) {
            cfg.poll_interval_secs = p.max(1) as u64;
        }
        Ok(cfg)
    }

    /// Overlay environment variables (`RUSTEAMS_*`) over `self`.
    pub fn with_env(mut self, get: &dyn Fn(&str) -> Option<String>) -> Self {
        if let Some(v) = get("RUSTEAMS_CLIENT_ID") {
            self.client_id = Some(v);
        }
        if let Some(v) = get("RUSTEAMS_TENANT_ID") {
            self.tenant_id = v;
        }
        if let Some(v) = get("RUSTEAMS_GRAPH_BASE_URL") {
            self.graph_base_url = v;
        }
        if let Some(v) = get("RUSTEAMS_POLL_INTERVAL_SECS").and_then(|s| s.parse::<u64>().ok()) {
            self.poll_interval_secs = v.max(1);
        }
        self
    }

    /// Overlay explicit CLI arguments (highest precedence) over `self`.
    pub fn with_cli_overrides(mut self, o: CliOverrides) -> Self {
        if let Some(v) = o.client_id {
            self.client_id = Some(v);
        }
        if let Some(v) = o.tenant_id {
            self.tenant_id = v;
        }
        if let Some(v) = o.graph_base_url {
            self.graph_base_url = v;
        }
        if let Some(v) = o.poll_interval_secs {
            self.poll_interval_secs = v.max(1);
        }
        self
    }

    /// Default config file path: `$XDG_CONFIG_HOME/rusteams/config.toml`.
    pub fn default_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|d| d.join("rusteams").join("config.toml"))
    }
}

/// Highest-precedence overrides, typically from CLI flags.
#[derive(Debug, Default)]
pub struct CliOverrides {
    pub client_id: Option<String>,
    pub tenant_id: Option<String>,
    pub graph_base_url: Option<String>,
    pub poll_interval_secs: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn env(map: HashMap<String, String>) -> impl Fn(&str) -> Option<String> {
        move |k: &str| map.get(k).cloned()
    }

    #[test]
    fn defaults_are_safe() {
        let c = AppConfig::default();
        assert_eq!(c.tenant_id, "organizations");
        assert!(c.graph_base_url.starts_with("https://"));
        assert!(c.client_id.is_none());
    }

    #[test]
    fn precedence_cli_over_env_over_file_over_defaults() {
        let file = AppConfig::from_toml_str(
            "[rusteams]\ntenant_id = \"file-tenant\"\npoll_interval_secs = 30\n",
        )
        .unwrap();
        let mut m = HashMap::new();
        m.insert("RUSTEAMS_TENANT_ID".to_string(), "env-tenant".to_string());
        let with_env = file.with_env(&env(m));
        assert_eq!(with_env.tenant_id, "env-tenant");
        assert_eq!(with_env.poll_interval_secs, 30);
        let cli = with_env.with_cli_overrides(CliOverrides {
            tenant_id: Some("cli-tenant".to_string()),
            ..Default::default()
        });
        assert_eq!(cli.tenant_id, "cli-tenant");
        assert_eq!(cli.poll_interval_secs, 30);
    }

    #[test]
    fn poll_interval_is_clamped_to_minimum_one() {
        let c = AppConfig::from_toml_str("poll_interval_secs = 0").unwrap();
        assert_eq!(c.poll_interval_secs, 1);
    }

    #[test]
    fn malformed_toml_is_a_config_error() {
        assert!(AppConfig::from_toml_str("[[[broken").is_err());
    }
}
