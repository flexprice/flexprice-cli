use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Meter, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum MeterCommands {
    /// List all meters
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get a meter by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a new meter from a JSON file
    Create {
        #[arg(long)]
        json: String,
    },
    /// Delete a meter by ID
    Delete { id: String },
}

#[derive(Tabled, serde::Serialize)]
struct MeterRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Event Name")]
    event_name: String,
    #[tabled(rename = "Aggregation")]
    aggregation: String,
    #[tabled(rename = "Status")]
    status: String,
}

impl From<Meter> for MeterRow {
    fn from(m: Meter) -> Self {
        Self {
            id: m.id,
            name: m.name.unwrap_or_default(),
            event_name: m.event_name.unwrap_or_default(),
            aggregation: m.aggregation.unwrap_or_default(),
            status: m.status.map(|s| output::status_badge(&s)).unwrap_or_default(),
        }
    }
}

pub async fn handle(cmd: MeterCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        MeterCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching meters...");
            let resp: ListResponse<Meter> = client.get("/v1/meters").await?;
            sp.finish_and_clear();
            let rows: Vec<MeterRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        MeterCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching meter...");
            let meter: Meter = client.get(&format!("/v1/meters/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&meter, json));
        }
        MeterCommands::Create { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Creating meter...");
            let meter: Meter = client.post("/v1/meters", &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Meter created: {}", meter.id));
            println!("{}", output::print_detail(&meter, false));
        }
        MeterCommands::Delete { id } => {
            let sp = spinner::create_spinner("Deleting meter...");
            client.delete_empty(&format!("/v1/meters/{}", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Meter {} deleted.", id));
        }
    }
    Ok(())
}
