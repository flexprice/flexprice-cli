use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Feature, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum FeatureCommands {
    /// List all features
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get a feature by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a new feature from a JSON file
    Create {
        #[arg(long)]
        json: String,
    },
    /// Delete a feature by ID
    Delete { id: String },
}

#[derive(Tabled, serde::Serialize)]
struct FeatureRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Lookup Key")]
    lookup_key: String,
    #[tabled(rename = "Type")]
    feature_type: String,
    #[tabled(rename = "Status")]
    status: String,
}

impl From<Feature> for FeatureRow {
    fn from(f: Feature) -> Self {
        Self {
            id: f.id,
            name: f.name.unwrap_or_default(),
            lookup_key: f.lookup_key.unwrap_or_default(),
            feature_type: f.feature_type.unwrap_or_default(),
            status: f.status.map(|s| output::status_badge(&s)).unwrap_or_default(),
        }
    }
}

pub async fn handle(cmd: FeatureCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        FeatureCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching features...");
            let resp: ListResponse<Feature> = client.get("/v1/features").await?;
            sp.finish_and_clear();
            let rows: Vec<FeatureRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        FeatureCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching feature...");
            let feature: Feature = client.get(&format!("/v1/features/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&feature, json));
        }
        FeatureCommands::Create { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Creating feature...");
            let feature: Feature = client.post("/v1/features", &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Feature created: {}", feature.id));
            println!("{}", output::print_detail(&feature, false));
        }
        FeatureCommands::Delete { id } => {
            let sp = spinner::create_spinner("Deleting feature...");
            client.delete_empty(&format!("/v1/features/{}", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Feature {} deleted.", id));
        }
    }
    Ok(())
}
