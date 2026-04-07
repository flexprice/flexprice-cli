use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Default FlexPrice API base URL when none is configured.
pub const DEFAULT_API_URL: &str = "https://api.cloud.flexprice.io";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    #[serde(default)]
    pub api_url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub environment_id: Option<String>,
}

impl Default for Credentials {
    fn default() -> Self {
        Self {
            api_url: DEFAULT_API_URL.to_string(),
            api_key: None,
            environment_id: None,
        }
    }
}

impl Credentials {
    /// Returns the path to ~/.flexprice/credentials.json
    pub fn credentials_path() -> PathBuf {
        let home = dirs::home_dir().expect("Could not determine home directory");
        home.join(".flexprice").join("credentials.json")
    }

    /// Load credentials with priority: CLI flags > .env in cwd > ~/.flexprice/credentials.json
    pub fn load(
        cli_api_url: Option<&str>,
        cli_api_key: Option<&str>,
    ) -> anyhow::Result<Self> {
        // 1. Start with stored credentials
        let mut creds = Self::load_from_file().unwrap_or_default();

        // 2. Override with .env in cwd
        if let Ok(val) = std::env::var("FLEXPRICE_API_URL") {
            if !val.is_empty() {
                creds.api_url = val;
            }
        }
        if let Ok(val) = std::env::var("FLEXPRICE_API_KEY") {
            if !val.is_empty() {
                creds.api_key = Some(val);
            }
        }
        if let Ok(val) = std::env::var("FLEXPRICE_ENVIRONMENT_ID") {
            if !val.is_empty() {
                creds.environment_id = Some(val);
            }
        }

        // 3. Override with CLI flags
        if let Some(url) = cli_api_url {
            creds.api_url = url.to_string();
        }
        if let Some(key) = cli_api_key {
            creds.api_key = Some(key.to_string());
        }

        Ok(creds)
    }

    /// Load from ~/.flexprice/credentials.json
    pub fn load_from_file() -> anyhow::Result<Self> {
        let path = Self::credentials_path();
        if !path.exists() {
            anyhow::bail!("No credentials file found");
        }
        let content = fs::read_to_string(&path)?;
        let creds: Credentials = serde_json::from_str(&content)?;
        Ok(creds)
    }

    /// Save to ~/.flexprice/credentials.json
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::credentials_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Delete credentials file
    pub fn delete() -> anyhow::Result<()> {
        let path = Self::credentials_path();
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    /// Check if the user has an API key configured
    pub fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    /// Returns the auth header name and value
    pub fn get_auth_header(&self) -> Option<(&'static str, String)> {
        self.api_key
            .as_ref()
            .map(|key| ("x-api-key", key.clone()))
    }

    /// Mask the API key for display
    pub fn masked_api_key(&self) -> String {
        match &self.api_key {
            Some(key) if key.len() > 8 => {
                format!("{}...{}", &key[..4], &key[key.len() - 4..])
            }
            Some(key) => "*".repeat(key.len()),
            None => "(not set)".to_string(),
        }
    }
}
