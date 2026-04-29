use std::sync::Arc;

use axum::extract::State;

use crate::AppState;

pub async fn healthz(State(_): State<Arc<AppState>>) -> &'static str {
    "OK"
}
