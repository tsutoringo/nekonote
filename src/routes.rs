use std::sync::Arc;

use crate::AppState;

pub mod github;
pub mod healthz;

pub fn routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/healthz", axum::routing::get(healthz::healthz))
        .nest("/github", github::routes())
}
