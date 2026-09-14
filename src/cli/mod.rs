pub mod commands;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::infra::auth::token_store::{KeyringStore, SecretStore};
use clap::Parser;
use commands::{Cli, Commands};

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
            println!("graph_base_url: {}", cfg.graph_base_url);
            println!("poll_interval_secs: {}", cfg.poll_interval_secs);
            println!("client_id: {}", if cfg.client_id.is_some() { "<set>" } else { "<unset>" });
            Ok(())
        }
        Some(Commands::Status) => {
            let store = KeyringStore::new("rusteams");
            let logged_in = store.load_refresh_token("default")?.is_some();
            println!("logged_in: {logged_in}");
            println!("tenant_id: {}", cfg.tenant_id);
            Ok(())
        }
        Some(Commands::Logout) => {
            KeyringStore::new("rusteams").clear_refresh_token("default")?;
            println!("Logged out: credentials cleared.");
            Ok(())
        }
        Some(Commands::Login) => {
            println!("Login uses device-code flow (Phase 2).");
            println!("1. Register a public-client app in Entra ID.");
            println!("2. Set RUSTEAMS_CLIENT_ID and run `rusteams login` again once implemented.");
            Ok(())
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
