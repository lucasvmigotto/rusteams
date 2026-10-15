// Layered configuration: CLI args > env vars > TOML file > safe defaults.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Secrets (client secrets, tokens) are NEVER read from this config.
//! Only non-secret identifiers (client-id, tenant-id) live here.

use crate::error::AppError;
use serde::{Deserialize, Serialize};

/// Microsoft Entra authority for production sign-in.
pub const ENTRA_AUTHORITY: &str = "https://login.microsoftonline.com";

/// Non-secret application configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    /// Entra public-client application (client) ID — BYO-app model.
    pub client_id: Option<String>,
    /// Tenant ID or `organizations` for multi-tenant work/school sign-in.
    pub tenant_id: String,
    /// Entra authority base URL (override for national clouds / tests).
    pub entra_authority: String,
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
            entra_authority: ENTRA_AUTHORITY.to_string(),
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
        if let Some(a) = table.get("entra_authority").and_then(|x| x.as_str()) {
            cfg.entra_authority = a.to_string();
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
        if let Some(v) = get("RUSTEAMS_ENTRA_AUTHORITY") {
            self.entra_authority = v;
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
        if let Some(v) = o.entra_authority {
            self.entra_authority = v;
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

    /// The config file in use: `RUSTEAMS_CONFIG` when set, else [`Self::default_path`].
    pub fn path(get: &dyn Fn(&str) -> Option<String>) -> Option<std::path::PathBuf> {
        match get(CONFIG_ENV) {
            Some(p) if !p.is_empty() => Some(std::path::PathBuf::from(p)),
            _ => Self::default_path(),
        }
    }

    /// The layered configuration: defaults, then the TOML file at `path` (when
    /// it exists), then `RUSTEAMS_*` environment, then explicit overrides.
    pub fn load(
        path: Option<&std::path::Path>,
        get: &dyn Fn(&str) -> Option<String>,
        overrides: CliOverrides,
    ) -> Result<Self, AppError> {
        let base = match path {
            Some(p) if p.exists() => {
                let text = std::fs::read_to_string(p)
                    .map_err(|e| AppError::Config(format!("cannot read {}: {e}", p.display())))?;
                Self::from_toml_str(&text)
                    .map_err(|e| AppError::Config(format!("{}: {e}", p.display())))?
            }
            _ => Self::default(),
        };
        Ok(base.with_env(get).with_cli_overrides(overrides))
    }
}

/// Environment variable naming an alternative config file.
pub const CONFIG_ENV: &str = "RUSTEAMS_CONFIG";

/// The commented starting point `rusteams config init` writes. Every setting
/// is present and commented out, so the file changes nothing until edited.
pub const CONFIG_TEMPLATE: &str = r#"# rusteams configuration — non-secret settings only. The refresh token lives
# in the OS keyring, never here.
#
# Precedence: this file < RUSTEAMS_* environment variables < command-line flags.
# Uncomment a setting to change it; the values shown are the defaults.

# Your Entra public-client app (client) ID. Required for `rusteams login`;
# see docs/development/entra-setup.md to register one.
# client_id = "00000000-0000-0000-0000-000000000000"

# Tenant ID, or "organizations" for any work or school account.
# tenant_id = "organizations"

# Entra authority and Graph base URL (change only for national clouds).
# entra_authority = "https://login.microsoftonline.com"
# graph_base_url = "https://graph.microsoft.com/v1.0"

# How often the active chat is refreshed, in seconds (minimum 1).
# poll_interval_secs = 15
"#;

/// Write [`CONFIG_TEMPLATE`] to `path`, creating its directory. Never
/// overwrites: returns `Ok(false)` when the file already exists.
pub fn write_template(path: &std::path::Path) -> Result<bool, AppError> {
    if path.exists() {
        return Ok(false);
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .map_err(|e| AppError::Config(format!("cannot create {}: {e}", dir.display())))?;
    }
    std::fs::write(path, CONFIG_TEMPLATE)
        .map_err(|e| AppError::Config(format!("cannot write {}: {e}", path.display())))?;
    Ok(true)
}

/// Highest-precedence overrides, typically from CLI flags.
#[derive(Debug, Default)]
pub struct CliOverrides {
    pub client_id: Option<String>,
    pub tenant_id: Option<String>,
    pub entra_authority: Option<String>,
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
    fn entra_authority_overrides_for_tests_and_national_clouds() {
        let file =
            AppConfig::from_toml_str("entra_authority = \"https://login.example.com\"").unwrap();
        assert_eq!(file.entra_authority, "https://login.example.com");
        let mut m = HashMap::new();
        m.insert("RUSTEAMS_ENTRA_AUTHORITY".to_string(), "https://entra.mock.local".to_string());
        let with_env = file.with_env(&env(m));
        assert_eq!(with_env.entra_authority, "https://entra.mock.local");
        assert_eq!(AppConfig::default().entra_authority, ENTRA_AUTHORITY);
    }

    #[test]
    fn poll_interval_is_clamped_to_minimum_one() {
        let c = AppConfig::from_toml_str("poll_interval_secs = 0").unwrap();
        assert_eq!(c.poll_interval_secs, 1);
    }

    #[test]
    fn template_parses_to_the_defaults() {
        assert_eq!(AppConfig::from_toml_str(CONFIG_TEMPLATE).unwrap(), AppConfig::default());
    }

    #[test]
    fn load_layers_file_then_env_then_flags() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "tenant_id = \"file-tenant\"\npoll_interval_secs = 30\n").unwrap();
        let no_env = |_: &str| None;
        let from_file = AppConfig::load(Some(&path), &no_env, CliOverrides::default()).unwrap();
        assert_eq!(from_file.tenant_id, "file-tenant");
        assert_eq!(from_file.poll_interval_secs, 30);

        let mut m = HashMap::new();
        m.insert("RUSTEAMS_TENANT_ID".to_string(), "env-tenant".to_string());
        let with_env = AppConfig::load(Some(&path), &env(m), CliOverrides::default()).unwrap();
        assert_eq!(with_env.tenant_id, "env-tenant");

        let flags = CliOverrides { tenant_id: Some("flag-tenant".into()), ..Default::default() };
        assert_eq!(AppConfig::load(Some(&path), &no_env, flags).unwrap().tenant_id, "flag-tenant");
    }

    #[test]
    fn load_without_a_file_is_the_defaults_and_a_bad_file_names_itself() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("none.toml");
        let no_env = |_: &str| None;
        assert_eq!(
            AppConfig::load(Some(&missing), &no_env, CliOverrides::default()).unwrap(),
            AppConfig::default()
        );
        let bad = dir.path().join("bad.toml");
        std::fs::write(&bad, "[[[broken").unwrap();
        let err = AppConfig::load(Some(&bad), &no_env, CliOverrides::default()).unwrap_err();
        assert!(err.to_string().contains("bad.toml"), "error names the file: {err}");
    }

    #[test]
    fn config_path_honours_the_override() {
        let mut m = HashMap::new();
        m.insert(CONFIG_ENV.to_string(), "/tmp/x/rusteams.toml".to_string());
        assert_eq!(
            AppConfig::path(&env(m)),
            Some(std::path::PathBuf::from("/tmp/x/rusteams.toml"))
        );
    }

    #[test]
    fn write_template_never_overwrites() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("config.toml");
        assert!(write_template(&path).unwrap());
        std::fs::write(&path, "tenant_id = \"mine\"\n").unwrap();
        assert!(!write_template(&path).unwrap());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "tenant_id = \"mine\"\n");
    }

    #[test]
    fn malformed_toml_is_a_config_error() {
        assert!(AppConfig::from_toml_str("[[[broken").is_err());
    }
}
