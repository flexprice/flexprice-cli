use anyhow::Result;
use clap::Subcommand;

use crate::api::client::ApiClient;
use crate::config::{Credentials, DEFAULT_API_URL};
use crate::utils::output;
use crate::utils::spinner;

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Set an API key (validated against the API before saving)
    SetApiKey {
        /// The API key to store
        key: String,
        /// API endpoint URL
        #[arg(long, default_value = DEFAULT_API_URL)]
        api_url: String,
    },
    /// Show current authenticated user and tenant
    Whoami,
    /// Show authentication status
    Status,
    /// Remove stored credentials
    Logout,
}

pub async fn handle(cmd: AuthCommands) -> Result<()> {
    match cmd {
        AuthCommands::SetApiKey { key, api_url } => set_api_key(key, api_url).await,
        AuthCommands::Whoami => whoami().await,
        AuthCommands::Status => status().await,
        AuthCommands::Logout => logout(),
    }
}

async fn set_api_key(key: String, api_url: String) -> Result<()> {
    let sp = spinner::create_spinner("Validating API key...");

    let creds = Credentials {
        api_url: api_url.clone(),
        api_key: Some(key.clone()),
        ..Default::default()
    };

    let client = ApiClient::new(creds.clone())?;
    client.health_check().await?;

    sp.finish_and_clear();

    creds.save()?;

    output::success("API key validated and saved!");
    output::success(&format!("API URL: {}", api_url));
    output::success(&format!(
        "Credentials saved to {}",
        Credentials::credentials_path().display()
    ));

    Ok(())
}

async fn whoami() -> Result<()> {
    let creds = require_auth()?;

    let sp = spinner::create_spinner("Fetching user info...");
    let client = ApiClient::new(creds.clone())?;

    let user_info: serde_json::Value = client.get("/v1/users/me").await?;
    sp.finish_and_clear();

    println!();
    output::info(&format!("API URL:    {}", creds.api_url));
    if let Some(ref eid) = creds.environment_id {
        output::info(&format!("Env ID:     {}", eid));
    }
    output::info("Auth:       API Key");
    println!();

    println!("{}", output::print_detail(&user_info, false));

    Ok(())
}

async fn status() -> Result<()> {
    match Credentials::load_from_file() {
        Ok(creds) => {
            output::success("Credentials found");
            output::info(&format!("API URL:    {}", creds.api_url));
            output::info(&format!("API Key:    {}", creds.masked_api_key()));
            if let Some(ref eid) = creds.environment_id {
                output::info(&format!("Env ID:     {}", eid));
            }

            // Try health check
            let sp = spinner::create_spinner("Testing connection...");
            let client = ApiClient::new(creds)?;
            match client.health_check().await {
                Ok(_) => {
                    sp.finish_and_clear();
                    output::success("API connection OK");
                }
                Err(e) => {
                    sp.finish_and_clear();
                    output::warning(&format!("API unreachable: {}", e));
                }
            }
        }
        Err(_) => {
            output::warning("Not authenticated.");
            output::info(
                "Run `flexprice auth set-api-key <KEY>` or set FLEXPRICE_API_KEY (see `--help`).",
            );
        }
    }
    Ok(())
}

fn logout() -> Result<()> {
    Credentials::delete()?;
    output::success("Credentials removed. You are now logged out.");
    Ok(())
}

/// Require authentication before proceeding. Returns credentials or exits.
pub fn require_auth() -> Result<Credentials> {
    let creds = Credentials::load(None, None)?;
    if !creds.is_authenticated() {
        output::warning(
            "Not authenticated. Run `flexprice auth set-api-key <KEY>` or set FLEXPRICE_API_KEY.",
        );
        std::process::exit(1);
    }
    Ok(creds)
}
