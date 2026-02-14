use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Customer, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum CustomerCommands {
    /// List all customers
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Get a customer by ID
    Get {
        /// Customer ID
        id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Create a new customer from a JSON file
    Create {
        /// Path to JSON file with customer data
        #[arg(long)]
        json: String,
    },
    /// Delete a customer by ID
    Delete {
        /// Customer ID
        id: String,
    },
    /// View customer usage summary
    Usage {
        /// Customer ID
        id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// View customer entitlements
    Entitlements {
        /// Customer ID
        id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Tabled, serde::Serialize)]
struct CustomerRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Email")]
    email: String,
    #[tabled(rename = "External ID")]
    external_id: String,
    #[tabled(rename = "Status")]
    status: String,
}

impl From<Customer> for CustomerRow {
    fn from(c: Customer) -> Self {
        Self {
            id: c.id,
            name: c.name.unwrap_or_default(),
            email: c.email.unwrap_or_default(),
            external_id: c.external_id.unwrap_or_default(),
            status: c.status.map(|s| output::status_badge(&s)).unwrap_or_default(),
        }
    }
}

pub async fn handle(cmd: CustomerCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        CustomerCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching customers...");
            let resp: ListResponse<Customer> = client.get("/v1/customers").await?;
            sp.finish_and_clear();

            let rows: Vec<CustomerRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        CustomerCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching customer...");
            let customer: Customer = client.get(&format!("/v1/customers/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&customer, json));
        }
        CustomerCommands::Create { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Creating customer...");
            let customer: Customer = client.post("/v1/customers", &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Customer created: {}", customer.id));
            println!("{}", output::print_detail(&customer, false));
        }
        CustomerCommands::Delete { id } => {
            let sp = spinner::create_spinner("Deleting customer...");
            client.delete_empty(&format!("/v1/customers/{}", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Customer {} deleted.", id));
        }
        CustomerCommands::Usage { id, json } => {
            let sp = spinner::create_spinner("Fetching usage...");
            let usage: serde_json::Value = client.get(&format!("/v1/customers/{}/usage", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&usage, json));
        }
        CustomerCommands::Entitlements { id, json } => {
            let sp = spinner::create_spinner("Fetching entitlements...");
            let ents: serde_json::Value = client.get(&format!("/v1/customers/{}/entitlements", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&ents, json));
        }
    }
    Ok(())
}
