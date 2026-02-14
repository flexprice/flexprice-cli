use anyhow::Result;
use clap::Subcommand;

use crate::api::client::ApiClient;
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum EventCommands {
    /// Ingest a single event from a JSON file
    Ingest {
        #[arg(long)]
        json: String,
    },
    /// Ingest events in bulk from a JSON file
    IngestBulk {
        #[arg(long)]
        json: String,
    },
    /// List recent events
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get an event by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Query event usage
    Usage {
        /// JSON body for usage query
        #[arg(long)]
        json: String,
    },
}

pub async fn handle(cmd: EventCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        EventCommands::Ingest { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Ingesting event...");
            let resp: serde_json::Value = client.post("/v1/events", &body).await?;
            sp.finish_and_clear();
            output::success("Event ingested successfully!");
            println!("{}", output::print_detail(&resp, false));
        }
        EventCommands::IngestBulk { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Ingesting events in bulk...");
            let resp: serde_json::Value = client.post("/v1/events/bulk", &body).await?;
            sp.finish_and_clear();
            output::success("Bulk events ingested successfully!");
            println!("{}", output::print_detail(&resp, false));
        }
        EventCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching events...");
            let resp: serde_json::Value = client.get("/v1/events").await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&resp, json));
        }
        EventCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching event...");
            let event: serde_json::Value = client.get(&format!("/v1/events/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&event, json));
        }
        EventCommands::Usage { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Fetching usage...");
            let usage: serde_json::Value = client.post("/v1/events/usage", &body).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&usage, false));
        }
    }
    Ok(())
}
