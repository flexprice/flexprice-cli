use serde::{Deserialize, Serialize};

// ─── Auth ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub tenant_id: String,
}

// ─── Generic List Wrapper ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ListResponse<T> {
    #[serde(default)]
    pub items: Vec<T>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

// ─── Customer ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Customer {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ─── Plan ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ─── Subscription ───────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub customer_id: Option<String>,
    #[serde(default)]
    pub plan_id: Option<String>,
    #[serde(default)]
    pub subscription_status: Option<String>,
    #[serde(default)]
    pub current_period_start: Option<String>,
    #[serde(default)]
    pub current_period_end: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ─── Invoice ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Invoice {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub customer_id: Option<String>,
    #[serde(default)]
    pub subscription_id: Option<String>,
    #[serde(default)]
    pub invoice_status: Option<String>,
    #[serde(default)]
    pub payment_status: Option<String>,
    #[serde(default)]
    pub amount_due: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ─── Meter ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub event_name: Option<String>,
    #[serde(default)]
    pub aggregation: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ─── Event ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Event {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub event_name: Option<String>,
    #[serde(default)]
    pub external_customer_id: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub properties: Option<serde_json::Value>,
}

// ─── Wallet ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub customer_id: Option<String>,
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub wallet_status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub real_time_balance: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
}

// ─── Feature ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Feature {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub lookup_key: Option<String>,
    #[serde(default, rename = "type")]
    pub feature_type: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

// ─── Entitlement ────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Entitlement {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub plan_id: Option<String>,
    #[serde(default)]
    pub feature_id: Option<String>,
    #[serde(default)]
    pub feature_type: Option<String>,
    #[serde(default)]
    pub is_enabled: Option<bool>,
    #[serde(default)]
    pub usage_limit: Option<f64>,
    #[serde(default)]
    pub created_at: Option<String>,
}
