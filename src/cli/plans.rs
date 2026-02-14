use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Plan, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum PlanCommands {
    /// List all plans
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get a plan by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a new plan from a JSON file
    Create {
        #[arg(long)]
        json: String,
    },
    /// Delete a plan by ID
    Delete { id: String },
}

#[derive(Tabled, serde::Serialize)]
struct PlanRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Status")]
    status: String,
}

impl From<Plan> for PlanRow {
    fn from(p: Plan) -> Self {
        Self {
            id: p.id,
            name: p.name.unwrap_or_default(),
            description: p.description.unwrap_or_default(),
            status: p.status.map(|s| output::status_badge(&s)).unwrap_or_default(),
        }
    }
}

pub async fn handle(cmd: PlanCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        PlanCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching plans...");
            let resp: ListResponse<Plan> = client.get("/v1/plans").await?;
            sp.finish_and_clear();
            let rows: Vec<PlanRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        PlanCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching plan...");
            let plan: Plan = client.get(&format!("/v1/plans/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&plan, json));
        }
        PlanCommands::Create { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Creating plan...");
            let plan: Plan = client.post("/v1/plans", &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Plan created: {}", plan.id));
            println!("{}", output::print_detail(&plan, false));
        }
        PlanCommands::Delete { id } => {
            let sp = spinner::create_spinner("Deleting plan...");
            client.delete_empty(&format!("/v1/plans/{}", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Plan {} deleted.", id));
        }
    }
    Ok(())
}
