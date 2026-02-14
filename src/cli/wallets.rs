use anyhow::Result;
use clap::Subcommand;
use tabled::Tabled;

use crate::api::client::ApiClient;
use crate::api::models::{Wallet, WalletBalance, ListResponse};
use crate::cli::auth::require_auth;
use crate::utils::{output, spinner};

#[derive(Subcommand)]
pub enum WalletCommands {
    /// List all wallets
    List {
        #[arg(long)]
        json: bool,
    },
    /// Get a wallet by ID
    Get {
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a new wallet from a JSON file
    Create {
        #[arg(long)]
        json: String,
    },
    /// Top up a wallet
    TopUp {
        /// Wallet ID
        id: String,
        /// JSON body with top-up details
        #[arg(long)]
        json: String,
    },
    /// Get real-time wallet balance
    Balance {
        id: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Tabled, serde::Serialize)]
struct WalletRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Customer")]
    customer_id: String,
    #[tabled(rename = "Balance")]
    balance: String,
    #[tabled(rename = "Currency")]
    currency: String,
    #[tabled(rename = "Status")]
    status: String,
}

impl From<Wallet> for WalletRow {
    fn from(w: Wallet) -> Self {
        Self {
            id: w.id,
            customer_id: w.customer_id.unwrap_or_default(),
            balance: w.balance.map(|b| format!("{:.2}", b)).unwrap_or_default(),
            currency: w.currency.unwrap_or_default(),
            status: w.wallet_status.map(|s| output::status_badge(&s)).unwrap_or_default(),
        }
    }
}

pub async fn handle(cmd: WalletCommands) -> Result<()> {
    let creds = require_auth()?;
    let client = ApiClient::new(creds)?;

    match cmd {
        WalletCommands::List { json } => {
            let sp = spinner::create_spinner("Fetching wallets...");
            let resp: ListResponse<Wallet> = client.get("/v1/wallets").await?;
            sp.finish_and_clear();
            let rows: Vec<WalletRow> = resp.items.into_iter().map(Into::into).collect();
            println!("{}", output::print_table(&rows, json));
        }
        WalletCommands::Get { id, json } => {
            let sp = spinner::create_spinner("Fetching wallet...");
            let wallet: Wallet = client.get(&format!("/v1/wallets/{}", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&wallet, json));
        }
        WalletCommands::Create { json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Creating wallet...");
            let wallet: Wallet = client.post("/v1/wallets", &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Wallet created: {}", wallet.id));
            println!("{}", output::print_detail(&wallet, false));
        }
        WalletCommands::TopUp { id, json: file } => {
            let data = std::fs::read_to_string(&file)?;
            let body: serde_json::Value = serde_json::from_str(&data)?;
            let sp = spinner::create_spinner("Topping up wallet...");
            let resp: serde_json::Value = client.post(&format!("/v1/wallets/{}/top-up", id), &body).await?;
            sp.finish_and_clear();
            output::success(&format!("Wallet {} topped up.", id));
            println!("{}", output::print_detail(&resp, false));
        }
        WalletCommands::Balance { id, json } => {
            let sp = spinner::create_spinner("Fetching balance...");
            let balance: WalletBalance = client.get(&format!("/v1/wallets/{}/balance/real-time", id)).await?;
            sp.finish_and_clear();
            println!("{}", output::print_detail(&balance, json));
        }
    }
    Ok(())
}
