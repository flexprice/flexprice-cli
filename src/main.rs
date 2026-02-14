mod api;
mod cli;
mod config;
mod tui;
mod utils;

use clap::{Parser, Subcommand};

/// ⚡ FlexPrice CLI — Usage-based billing, from your terminal.
#[derive(Parser)]
#[command(
    name = "flexprice",
    version,
    about = "⚡ FlexPrice CLI — Usage-based billing, from your terminal.",
    long_about = None,
    arg_required_else_help = true,
    styles = get_styles(),
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Override the API base URL
    #[arg(long, global = true)]
    api_url: Option<String>,

    /// Override the API key
    #[arg(long, global = true)]
    api_key: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with FlexPrice (login, API key, status)
    Auth {
        #[command(subcommand)]
        command: cli::auth::AuthCommands,
    },
    /// Manage customers
    Customers {
        #[command(subcommand)]
        command: cli::customers::CustomerCommands,
    },
    /// Manage pricing plans
    Plans {
        #[command(subcommand)]
        command: cli::plans::PlanCommands,
    },
    /// Manage subscriptions
    Subscriptions {
        #[command(subcommand)]
        command: cli::subscriptions::SubscriptionCommands,
    },
    /// Manage invoices
    Invoices {
        #[command(subcommand)]
        command: cli::invoices::InvoiceCommands,
    },
    /// Manage meters
    Meters {
        #[command(subcommand)]
        command: cli::meters::MeterCommands,
    },
    /// Ingest and query events
    Events {
        #[command(subcommand)]
        command: cli::events::EventCommands,
    },
    /// Manage wallets and credit balances
    Wallets {
        #[command(subcommand)]
        command: cli::wallets::WalletCommands,
    },
    /// Manage features
    Features {
        #[command(subcommand)]
        command: cli::features::FeatureCommands,
    },
    /// Manage entitlements
    Entitlements {
        #[command(subcommand)]
        command: cli::entitlements::EntitlementCommands,
    },
    /// Show current configuration
    Config,
    /// Launch the interactive TUI dashboard
    Dashboard,
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(
            clap::builder::styling::AnsiColor::Cyan
                .on_default()
                .bold(),
        )
        .usage(
            clap::builder::styling::AnsiColor::Cyan
                .on_default()
                .bold(),
        )
        .literal(
            clap::builder::styling::AnsiColor::Green.on_default().bold(),
        )
        .placeholder(clap::builder::styling::AnsiColor::BrightBlue.on_default())
}

#[tokio::main]
async fn main() {
    // Load .env from cwd if it exists
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Auth { command } => cli::auth::handle(command).await,
        Commands::Customers { command } => cli::customers::handle(command).await,
        Commands::Plans { command } => cli::plans::handle(command).await,
        Commands::Subscriptions { command } => cli::subscriptions::handle(command).await,
        Commands::Invoices { command } => cli::invoices::handle(command).await,
        Commands::Meters { command } => cli::meters::handle(command).await,
        Commands::Events { command } => cli::events::handle(command).await,
        Commands::Wallets { command } => cli::wallets::handle(command).await,
        Commands::Features { command } => cli::features::handle(command).await,
        Commands::Entitlements { command } => cli::entitlements::handle(command).await,
        Commands::Config => handle_config(),
        Commands::Dashboard => handle_dashboard().await,
    };

    if let Err(e) = result {
        utils::output::error(&format!("{:#}", e));
        std::process::exit(1);
    }
}

fn handle_config() -> anyhow::Result<()> {
    let creds = config::Credentials::load(None, None)?;
    println!();
    utils::output::info(&format!("API URL:     {}", if creds.api_url.is_empty() { "(not set)" } else { &creds.api_url }));
    utils::output::info(&format!("API Key:     {}", creds.masked_api_key()));
    utils::output::info(&format!("Auth Token:  {}", if creds.auth_token.is_some() { "(set)" } else { "(not set)" }));
    utils::output::info(&format!("Tenant ID:   {}", creds.tenant_id.as_deref().unwrap_or("(not set)")));
    utils::output::info(&format!("User ID:     {}", creds.user_id.as_deref().unwrap_or("(not set)")));
    utils::output::info(&format!("Env ID:      {}", creds.environment_id.as_deref().unwrap_or("(not set)")));
    utils::output::info(&format!("Config path: {}", config::Credentials::credentials_path().display()));
    println!();
    Ok(())
}

async fn handle_dashboard() -> anyhow::Result<()> {
    let creds = cli::auth::require_auth()?;
    tui::dashboard::run(creds).await
}
