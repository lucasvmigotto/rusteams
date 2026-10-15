pub mod commands;

use crate::config::{AppConfig, CliOverrides};
use crate::error::AppError;
use crate::infra::auth::DeviceCodeClient;
use crate::infra::auth::token_store::{FileStore, KeyringStore, SecretStore, TOKEN_FILE_ENV};
use clap::Parser;
use commands::{Cli, Commands, ConfigAction};
use serde_json::{Value, json};
use std::path::Path;

/// Microsoft Entra authority for production sign-in (default; see `AppConfig`).
pub use crate::config::ENTRA_AUTHORITY;

/// Refresh-token store: OS keyring by default, file-backed when
/// `RUSTEAMS_TOKEN_FILE` is set (headless/simulation use — never production).
fn token_store() -> Box<dyn SecretStore> {
    match std::env::var(TOKEN_FILE_ENV) {
        Ok(path) => {
            tracing::warn!("file token store in use ({TOKEN_FILE_ENV}) — not for production use");
            Box::new(FileStore::new(path))
        }
        Err(_) => Box::new(KeyringStore::new("rusteams")),
    }
}

pub async fn run() -> Result<(), AppError> {
    crate::telemetry::init();
    let cli = Cli::parse();
    let env = |k: &str| std::env::var(k).ok();
    let path = AppConfig::path(&env);
    let overrides = CliOverrides { tenant_id: cli.tenant_id.clone(), ..Default::default() };
    let cfg = AppConfig::load(path.as_deref(), &env, overrides)?;
    match cli.command {
        None => {
            println!(
                "rusteams {} — launch TUI (Phase 6). Configure first: `rusteams config`.",
                env!("CARGO_PKG_VERSION")
            );
            Ok(())
        }
        Some(Commands::Version) => {
            println!("rusteams {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some(Commands::Config { action: Some(ConfigAction::Init), .. }) => {
            let path = path.ok_or_else(|| {
                AppError::Config(format!("no config directory; set {}", crate::config::CONFIG_ENV))
            })?;
            if crate::config::write_template(&path)? {
                println!("Wrote {} (every setting commented out).", path.display());
            } else {
                println!("{} already exists; left unchanged.", path.display());
            }
            Ok(())
        }
        Some(Commands::Config { path: true, .. }) => {
            match path {
                Some(p) => println!("{}", p.display()),
                None => println!("<no config directory>"),
            }
            Ok(())
        }
        Some(Commands::Config { json, .. }) => {
            emit(&config_report(&cfg, path.as_deref()), json);
            Ok(())
        }
        Some(Commands::Status { json }) => {
            let logged_in = token_store().load_refresh_token("default")?.is_some();
            emit(&json!({ "logged_in": logged_in, "tenant_id": cfg.tenant_id }), json);
            Ok(())
        }
        Some(Commands::Logout) => {
            token_store().clear_refresh_token("default")?;
            println!("Logged out: credentials cleared.");
            Ok(())
        }
        Some(Commands::Login) => {
            let client_id = cfg.client_id.clone().ok_or_else(|| {
                AppError::Config(
                    "client-id is not set; register a public-client app in Entra ID, \
                     then set RUSTEAMS_CLIENT_ID or client_id in the config file \
                     (`rusteams config init` writes one)"
                        .into(),
                )
            })?;
            login(&cfg.entra_authority, &cfg.tenant_id, &client_id).await
        }
        Some(Commands::Doctor { json }) => {
            let keyring = KeyringStore::new("rusteams").load_refresh_token("default").is_ok();
            let report = doctor_report(path.as_deref(), keyring);
            if json {
                emit(&report, true);
            } else {
                // Text stays `doctor: <check>: <result>`, as before --json.
                for line in text_lines(&report) {
                    let (k, v) = line.split_once(": ").unwrap_or((&line, ""));
                    println!("doctor: {}: {v}", k.replace('_', " "));
                }
            }
            Ok(())
        }
    }
}

/// Non-secret configuration as a report: the client ID only as set/unset.
fn config_report(cfg: &AppConfig, path: Option<&Path>) -> Value {
    json!({
        "tenant_id": cfg.tenant_id,
        "entra_authority": cfg.entra_authority,
        "graph_base_url": cfg.graph_base_url,
        "poll_interval_secs": cfg.poll_interval_secs,
        "client_id_set": cfg.client_id.is_some(),
        "config_file": path.map(|p| p.display().to_string()),
        "config_file_exists": path.is_some_and(Path::exists),
    })
}

fn doctor_report(path: Option<&Path>, keyring_reachable: bool) -> Value {
    json!({
        "config_file": path.map(|p| p.display().to_string()),
        "config_file_exists": path.is_some_and(Path::exists),
        "keyring_reachable": keyring_reachable,
    })
}

/// Print a report as pretty JSON, or as `key: value` lines (keys sorted).
fn emit(report: &Value, as_json: bool) {
    if as_json {
        println!("{}", serde_json::to_string_pretty(report).unwrap_or_default());
        return;
    }
    for line in text_lines(report) {
        println!("{line}");
    }
}

fn text_lines(report: &Value) -> Vec<String> {
    report
        .as_object()
        .map(|map| {
            map.iter()
                .map(|(k, v)| match v {
                    Value::String(s) => format!("{k}: {s}"),
                    Value::Null => format!("{k}: <none>"),
                    other => format!("{k}: {other}"),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Run the device-code login: print the user code, poll Entra, persist the
/// refresh token (OS keyring, or file store when `RUSTEAMS_TOKEN_FILE` is set).
/// Access tokens stay in memory only.
async fn login(authority: &str, tenant: &str, client_id: &str) -> Result<(), AppError> {
    let client = DeviceCodeClient::new(authority, tenant, client_id)?;
    let code = client.request_code(&DeviceCodeClient::scopes()).await?;
    // The message comes from Entra and contains the code + verification URL.
    // It is display text, not a secret — but sanitize defensively anyway.
    println!("{}", crate::sanitize::sanitize(&code.message));
    let token = client.poll_for_token(&code, 60).await?;
    match token.refresh_token {
        Some(rt) => {
            token_store().save_refresh_token("default", &rt)?;
            println!("Logged in: refresh token stored.");
            Ok(())
        }
        None => Err(AppError::Auth("no refresh token issued".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_report_never_carries_the_client_id() {
        let cfg = AppConfig { client_id: Some("secret-ish-app-id".into()), ..AppConfig::default() };
        let report = config_report(&cfg, None);
        assert_eq!(report["client_id_set"], json!(true));
        assert!(!report.to_string().contains("secret-ish-app-id"));
        assert_eq!(report["config_file"], Value::Null);
    }

    #[test]
    fn doctor_report_has_stable_keys() {
        let report = doctor_report(Some(Path::new("/nonexistent/config.toml")), false);
        assert_eq!(report["config_file"], json!("/nonexistent/config.toml"));
        assert_eq!(report["config_file_exists"], json!(false));
        assert_eq!(report["keyring_reachable"], json!(false));
    }
}
