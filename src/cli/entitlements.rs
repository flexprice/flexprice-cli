use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Entitlement, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum EntitlementCommands {
    /// List all entitlements
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get an entitlement by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a new entitlement from a JSON file
    Create {
        #[arg(long)]
        json: String,
    },
    /// Delete an entitlement by ID
    Delete { id: String },
}

#[derive(Tabled, serde::Serialize)]
struct EntitlementRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Plan")]
    plan_id: String,
    #[tabled(rename = "Feature")]
    feature_id: String,
    #[tabled(rename = "Type")]
    feature_type: String,
    #[tabled(rename = "Enabled")]
    enabled: String,
    #[tabled(rename = "Usage Limit")]
    usage_limit: String,
}

impl From<Entitlement> for EntitlementRow {
    fn from(e: Entitlement) -> Self {
        Self {
            id: e.id,
            plan_id: e.plan_id.unwrap_or_default(),
            feature_id: e.feature_id.unwrap_or_default(),
            feature_type: e.feature_type.unwrap_or_default(),
            enabled: e.is_enabled.map(|b| if b { "✓".to_string() } else { "✗".to_string() }).unwrap_or_default(),
            usage_limit: e.usage_limit.map(|l| format!("{:.0}", l)).unwrap_or("∞".to_string()),
        }
    }
}

pub async fn handle(cmd: EntitlementCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        EntitlementCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching entitlements...");
            let resp: ListResponse<Entitlement> = client.get("/v1/entitlements").await?;
            sp.finish_and_clear();
            let rows: Vec<EntitlementRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        EntitlementCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching entitlement...");
            let ent: Entitlement = client.get(&format!("/v1/entitlements/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&ent, json));
        }
        EntitlementCommands::Create { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Creating entitlement...");
            let ent: Entitlement = client.post("/v1/entitlements", &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Entitlement created: {}", ent.id));
            println!("{}", output::print_detail(&ent, false));
        }
        EntitlementCommands::Delete { id } => {
            let sp = spinner::create_spinner("Deleting entitlement...");
            client.delete_empty(&format!("/v1/entitlements/{}", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Entitlement {} deleted.", id));
        }
    }
    Ok(())
}
