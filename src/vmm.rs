use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cvm {
    pub app_id: String,
    pub instance_id: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
struct ListActiveCvmsResponse {
    vms: Vec<Cvm>,
}

#[derive(Debug, Clone)]
pub struct VmmClient {
    client: Client,
    base_url: String,
}

impl VmmClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_owned(),
        }
    }

    pub async fn list_active_cvms(&self) -> Result<Vec<Cvm>, AppError> {
        let url = format!("{}/prpc/Status?json", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("failed to reach vmm: {e}")))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "vmm returned status {}",
                response.status()
            )));
        }

        let status: ListActiveCvmsResponse = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("failed to parse vmm response: {e}")))?;

        const EXCLUDED_CVM_NAMES: &[&str] = &["kms", "dstack-gateway"];

        Ok(status
            .vms
            .into_iter()
            .filter(|vm| {
                !matches!(vm.status.as_str(), "stopped" | "removed")
                    && !EXCLUDED_CVM_NAMES.contains(&vm.name.as_str())
            })
            .collect())
    }
}
