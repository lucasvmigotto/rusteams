pub mod commands;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::infra::auth::DeviceCodeClient;
use crate::infra::auth::token_store::{FileStore, KeyringStore, SecretStore, TOKEN_FILE_ENV};
use clap::Parser;
use commands::{Cli, Commands};

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
    let cfg = AppConfig::default().with_env(&|k| std::env::var(k).ok());
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
        Some(Commands::Config) => {
            println!("tenant_id: {}", cfg.tenant_id);
            println!("entra_authority: {}", cfg.entra_authority);
            println!("graph_base_url: {}", cfg.graph_base_url);
            println!("poll_interval_secs: {}", cfg.poll_interval_secs);
            println!("client_id: {}", if cfg.client_id.is_some() { "<set>" } else { "<unset>" });
            Ok(())
        }
        Some(Commands::Status) => {
            let logged_in = token_store().load_refresh_token("default")?.is_some();
            println!("logged_in: {logged_in}");
            println!("tenant_id: {}", cfg.tenant_id);
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
                     then set RUSTEAMS_CLIENT_ID or --client-id"
                        .into(),
                )
            })?;
            login(&cfg.entra_authority, &cfg.tenant_id, &client_id).await
        }
        Some(Commands::Doctor) => {
            println!("doctor: config file: {:?}", AppConfig::default_path());
            println!(
                "doctor: keyring reachable: {}",
                KeyringStore::new("rusteams").load_refresh_token("default").is_ok()
            );
            Ok(())
        }
    }
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
