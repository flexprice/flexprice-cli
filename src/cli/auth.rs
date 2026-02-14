use anyhow::Result;
use clap::Subcommand;
use dialoguer::{Input, Password};

use crate::api::client::ApiClient;
use crate::api::models::AuthResponse;
use crate::api::models::LoginRequest;
use crate::config::Credentials;
use crate::utils::output;
use crate::utils::spinner;

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Interactive login with email and password
    Login {
        /// API endpoint URL
        #[arg(long)]
        api_url: Option<String>,
    },
    /// Set an API key directly (for CI/CD or pre-provisioned keys)
    SetApiKey {
        /// The API key to store
        key: String,
        /// API endpoint URL
        #[arg(long, default_value = "http://localhost:8080")]
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
        AuthCommands::Login { api_url } => login(api_url).await,
        AuthCommands::SetApiKey { key, api_url } => set_api_key(key, api_url).await,
        AuthCommands::Whoami => whoami().await,
        AuthCommands::Status => status().await,
        AuthCommands::Logout => logout(),
    }
}

async fn login(override_url: Option<String>) -> Result<()> {
    output::print_banner();

    let api_url: String = if let Some(url) = override_url {
        url
    } else {
        Input::new()
            .with_prompt("  API Endpoint")
            .default("http://localhost:8080".to_string())
            .interact_text()?
    };

    let email: String = Input::new()
        .with_prompt("  Email")
        .interact_text()?;

    let password = Password::new()
        .with_prompt("  Password")
        .interact()?;

    let sp = spinner::create_spinner("Authenticating...");

    let creds = Credentials {
        api_url: api_url.clone(),
        ..Default::default()
    };
    let client = ApiClient::new(creds)?;

    let login_req = LoginRequest { email: email.clone(), password };
    let auth_resp: AuthResponse = client.post("/v1/auth/login", &login_req).await?;

    sp.finish_and_clear();

    // Store credentials
    let creds = Credentials {
        api_url,
        auth_token: Some(auth_resp.token),
        tenant_id: Some(auth_resp.tenant_id.clone()),
        user_id: Some(auth_resp.user_id.clone()),
        api_key: None,
        environment_id: None,
    };
    creds.save()?;

    println!();
    output::success("Authenticated successfully!");
    output::success(&format!("Tenant: {}", auth_resp.tenant_id));
    output::success(&format!("User: {} ({})", email, auth_resp.user_id));
    output::success(&format!(
        "Credentials saved to {}",
        Credentials::credentials_path().display()
    ));
    println!();

    Ok(())
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
    if let Some(ref tid) = creds.tenant_id {
        output::info(&format!("Tenant ID:  {}", tid));
    }
    if let Some(ref uid) = creds.user_id {
        output::info(&format!("User ID:    {}", uid));
    }
    if let Some(ref eid) = creds.environment_id {
        output::info(&format!("Env ID:     {}", eid));
    }
    output::info(&format!("Auth:       {}", if creds.api_key.is_some() { "API Key" } else { "JWT Token" }));
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
            output::info(&format!("Auth:       {}", if creds.api_key.is_some() { "API Key" } else if creds.auth_token.is_some() { "JWT Token" } else { "(none)" }));
            if let Some(ref tid) = creds.tenant_id {
                output::info(&format!("Tenant ID:  {}", tid));
            }
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
            output::info("Run `flexprice auth login` or `flexprice auth set-api-key <KEY>` to get started.");
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
        output::warning("Not authenticated. Run `flexprice auth login` or `flexprice auth set-api-key <KEY>` first.");
        std::process::exit(1);
    }
    Ok(creds)
}
