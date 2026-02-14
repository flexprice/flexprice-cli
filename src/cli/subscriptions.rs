use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Subscription, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum SubscriptionCommands {
    /// List all subscriptions
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get a subscription by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a new subscription from a JSON file
    Create {
        #[arg(long)]
        json: String,
    },
    /// Cancel a subscription
    Cancel { id: String },
    /// Get usage for a subscription
    Usage {
        /// JSON body for usage query
        #[arg(long)]
        json: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct SubscriptionRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Customer")]
    customer_id: String,
    #[tabled(rename = "Plan")]
    plan_id: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Period Start")]
    period_start: String,
    #[tabled(rename = "Period End")]
    period_end: String,
}

impl From<Subscription> for SubscriptionRow {
    fn from(s: Subscription) -> Self {
        Self {
            id: s.id,
            customer_id: s.customer_id.unwrap_or_default(),
            plan_id: s.plan_id.unwrap_or_default(),
            status: s.subscription_status.map(|st| output::status_badge(&st)).unwrap_or_default(),
            period_start: s.current_period_start.unwrap_or_default(),
            period_end: s.current_period_end.unwrap_or_default(),
        }
    }
}

pub async fn handle(cmd: SubscriptionCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        SubscriptionCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching subscriptions...");
            let resp: ListResponse<Subscription> = client.get("/v1/subscriptions").await?;
            sp.finish_and_clear();
            let rows: Vec<SubscriptionRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        SubscriptionCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching subscription...");
            let sub: Subscription = client.get(&format!("/v1/subscriptions/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&sub, json));
        }
        SubscriptionCommands::Create { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Creating subscription...");
            let sub: Subscription = client.post("/v1/subscriptions", &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Subscription created: {}", sub.id));
            println!("{}", output::print_detail(&sub, false));
        }
        SubscriptionCommands::Cancel { id } => {
            let sp = spinner::create_spinner("Cancelling subscription...");
            let sub: serde_json::Value = client.post_empty(&format!("/v1/subscriptions/{}/cancel", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Subscription {} cancelled.", id));
            println!("{}", output::print_detail(&sub, false));
        }
        SubscriptionCommands::Usage { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Fetching usage...");
            let usage: serde_json::Value = client.post("/v1/subscriptions/usage", &body).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&usage, false));
        }
    }
    Ok(())
}
