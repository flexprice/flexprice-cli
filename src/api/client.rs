use anyhow::{Context, Result};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::Credentials;

/// FlexPrice API client with automatic auth and error handling
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    credentials: Credentials,
}

#[derive(Debug, serde::Deserialize)]
struct ApiError {
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    hint: Option<String>,
}

impl ApiClient {
    pub fn new(credentials: Credentials) -> Result<Self> {
        let base_url = if credentials.api_url.is_empty() {
            "http://localhost:8080".to_string()
        } else {
            credentials.api_url.trim_end_matches('/').to_string()
        };

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url,
            credentials,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn apply_auth(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some((header, value)) = self.credentials.get_auth_header() {
            req = req.header(header, value);
        }
        if let Some(ref env_id) = self.credentials.environment_id {
            req = req.header("x-environment-id", env_id);
        }
        req
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T> {
        let status = response.status();
        if status.is_success() {
            let body = response.json::<T>().await
                .context("Failed to parse response body")?;
            Ok(body)
        } else {
            let body_text = response.text().await.unwrap_or_default();
            let err_msg = if let Ok(api_err) = serde_json::from_str::<ApiError>(&body_text) {
                let msg = api_err.error
                    .or(api_err.message)
                    .unwrap_or_else(|| "Unknown error".to_string());
                if let Some(hint) = api_err.hint {
                    format!("{} ({}): {} — {}", status.as_u16(), status.canonical_reason().unwrap_or(""), msg, hint)
                } else {
                    format!("{} ({}): {}", status.as_u16(), status.canonical_reason().unwrap_or(""), msg)
                }
            } else {
                match status {
                    StatusCode::UNAUTHORIZED => "Authentication failed. Run `flexprice auth login` or check your API key.".to_string(),
                    StatusCode::FORBIDDEN => "Permission denied. Your credentials may not have access to this resource.".to_string(),
                    StatusCode::NOT_FOUND => "Resource not found. Verify the ID is correct.".to_string(),
                    _ => format!("{}: {}", status, body_text),
                }
            };
            anyhow::bail!("{}", err_msg)
        }
    }

    async fn handle_response_text(response: Response) -> Result<String> {
        let status = response.status();
        if status.is_success() {
            Ok(response.text().await.unwrap_or_default())
        } else {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("{}: {}", status, body)
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let req = self.client.get(self.url(path));
        let req = self.apply_auth(req);
        let resp = req.send().await.context("Request failed")?;
        Self::handle_response(resp).await
    }

    pub async fn get_text(&self, path: &str) -> Result<String> {
        let req = self.client.get(self.url(path));
        let req = self.apply_auth(req);
        let resp = req.send().await.context("Request failed")?;
        Self::handle_response_text(resp).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let req = self.client.post(self.url(path)).json(body);
        let req = self.apply_auth(req);
        let resp = req.send().await.context("Request failed")?;
        Self::handle_response(resp).await
    }

    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let req = self.client.post(self.url(path));
        let req = self.apply_auth(req);
        let resp = req.send().await.context("Request failed")?;
        Self::handle_response(resp).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let req = self.client.put(self.url(path)).json(body);
        let req = self.apply_auth(req);
        let resp = req.send().await.context("Request failed")?;
        Self::handle_response(resp).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let req = self.client.delete(self.url(path));
        let req = self.apply_auth(req);
        let resp = req.send().await.context("Request failed")?;
        Self::handle_response(resp).await
    }

    pub async fn delete_empty(&self, path: &str) -> Result<()> {
        let req = self.client.delete(self.url(path));
        let req = self.apply_auth(req);
        let resp = req.send().await.context("Request failed")?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("{}: {}", status, body)
        }
    }

    /// Health check — used for validating connection + credentials
    pub async fn health_check(&self) -> Result<()> {
        let req = self.client.get(self.url("/health"));
        let resp = req.send().await.context("Cannot reach FlexPrice API")?;
        if resp.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!("API returned status {}", resp.status())
        }
    }
}
