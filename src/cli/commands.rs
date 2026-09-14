use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rusteams", version, about = "TUI client for Microsoft Teams (MVP: chat)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Increase log verbosity (-v, -vv).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
    /// Override tenant (default: organizations).
    #[arg(long, env = "RUSTEAMS_TENANT_ID", global = true)]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start device-code login (BYO Entra app).
    Login,
    /// Clear stored credentials.
    Logout,
    /// Show auth/config status (never prints secrets).
    Status,
    /// Check environment, keyring, and Graph reachability.
    Doctor,
    /// Print resolved non-secret configuration.
    Config,
    /// Print version.
    Version,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parses_login_subcommand() {
        let cli = Cli::try_parse_from(["rusteams", "login"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Login)));
    }
}
