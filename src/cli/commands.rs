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
    Status {
        /// Print JSON instead of text.
        #[arg(long)]
        json: bool,
    },
    /// Check environment, keyring, and Graph reachability.
    Doctor {
        /// Print JSON instead of text.
        #[arg(long)]
        json: bool,
    },
    /// Print resolved non-secret configuration.
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
        /// Print the config file's path only.
        #[arg(long, conflicts_with = "json")]
        path: bool,
        /// Print JSON instead of text.
        #[arg(long)]
        json: bool,
    },
    /// Print version.
    Version,
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    /// Write a commented config file (never overwrites an existing one).
    Init,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parses_login_subcommand() {
        let cli = Cli::try_parse_from(["rusteams", "login"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Login)));
    }

    #[test]
    fn cli_parses_json_path_and_init() {
        let cli = Cli::try_parse_from(["rusteams", "status", "--json"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Status { json: true })));
        let cli = Cli::try_parse_from(["rusteams", "config", "--path"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Config { path: true, .. })));
        let cli = Cli::try_parse_from(["rusteams", "config", "init"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::Config { action: Some(ConfigAction::Init), .. })
        ));
        assert!(Cli::try_parse_from(["rusteams", "config", "--path", "--json"]).is_err());
    }
}
