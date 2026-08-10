use std::collections::HashMap;

use axum::Json;
use axum::extract::State;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use chrono::Utc;
use serde::Serialize;
use serde_json::{Value, json};

use crate::application::AppState;
use crate::error::AppError;

/// `GET /health` — returns `{"status":"ok"}`.
pub async fn health_check() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

/// `GET /` — returns service name and current UTC timestamp.
pub async fn root() -> Json<Value> {
    Json(json!({
        "service": "dstack-vmm-api",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

/// Fallback handler for non-existing routes.
pub async fn not_found(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": format!("Route not found {}", uri.path()) })),
    )
}

#[derive(Debug, Serialize)]
pub struct CvmInstance {
    pub instance_id: String,
    pub machine_id: String,
}

#[derive(Debug, Serialize)]
pub struct CvmSummary {
    pub app_id: String,
    pub name: String,
    pub instances: Vec<CvmInstance>,
}

/// `GET /cvms` — returns active CVMs grouped by app.
pub async fn get_active_cvms(
    State(state): State<AppState>,
) -> Result<Json<Vec<CvmSummary>>, AppError> {
    let cvms = state.vmm_client.list_active_cvms().await?;

    let mut groups: HashMap<String, CvmSummary> = HashMap::new();

    for cvm in cvms {
        let Some(instance_id) = cvm.instance_id else {
            continue;
        };
        let instance = CvmInstance {
            instance_id,
            machine_id: state.config.machine_id.clone(),
        };

        groups
            .entry(cvm.app_id.clone())
            .or_insert_with(|| CvmSummary {
                app_id: cvm.app_id,
                name: cvm.name,
                instances: Vec::new(),
            })
            .instances
            .push(instance);
    }

    Ok(Json(groups.into_values().collect()))
}
