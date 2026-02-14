use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Invoice, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum InvoiceCommands {
    /// List all invoices
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get an invoice by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Finalize an invoice
    Finalize { id: String },
    /// Void an invoice
    Void { id: String },
    /// Download invoice PDF
    Pdf {
        id: String,
        /// Output file path
        #[arg(long, short, default_value = "invoice.pdf")]
        output: String,
    },
}

#[derive(Tabled, serde::Serialize)]
struct InvoiceRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Customer")]
    customer_id: String,
    #[tabled(rename = "Status")]
    invoice_status: String,
    #[tabled(rename = "Payment")]
    payment_status: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Currency")]
    currency: String,
}

impl From<Invoice> for InvoiceRow {
    fn from(i: Invoice) -> Self {
        Self {
            id: i.id,
            customer_id: i.customer_id.unwrap_or_default(),
            invoice_status: i.invoice_status.map(|s| output::status_badge(&s)).unwrap_or_default(),
            payment_status: i.payment_status.map(|s| output::status_badge(&s)).unwrap_or_default(),
            amount: i.amount_due.map(|a| format!("{:.2}", a)).unwrap_or_default(),
            currency: i.currency.unwrap_or_default(),
        }
    }
}

pub async fn handle(cmd: InvoiceCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        InvoiceCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching invoices...");
            let resp: ListResponse<Invoice> = client.get("/v1/invoices").await?;
            sp.finish_and_clear();
            let rows: Vec<InvoiceRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        InvoiceCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching invoice...");
            let inv: Invoice = client.get(&format!("/v1/invoices/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&inv, json));
        }
        InvoiceCommands::Finalize { id } => {
            let sp = spinner::create_spinner("Finalizing invoice...");
            let inv: serde_json::Value = client.post_empty(&format!("/v1/invoices/{}/finalize", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Invoice {} finalized.", id));
            println!("{}", output::print_detail(&inv, false));
        }
        InvoiceCommands::Void { id } => {
            let sp = spinner::create_spinner("Voiding invoice...");
            let inv: serde_json::Value = client.post_empty(&format!("/v1/invoices/{}/void", id)).await?;
            sp.finish_and_clear();
            output::success(&format!("Invoice {} voided.", id));
            println!("{}", output::print_detail(&inv, false));
        }
        InvoiceCommands::Pdf { id, output: out_path } => {
            let sp = spinner::create_spinner("Downloading PDF...");
            let pdf_content = client.get_text(&format!("/v1/invoices/{}/pdf", id)).await?;
            std::fs::write(&out_path, pdf_content)?;
            sp.finish_and_clear();
            output::success(&format!("Invoice PDF saved to {}", out_path));
        }
    }
    Ok(())
}
