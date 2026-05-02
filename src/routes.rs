use std::sync::Arc;

use crate::state::AppState;

pub mod github;
pub mod google_calendar;
pub mod healthz;

pub fn routes(state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/healthz", axum::routing::get(healthz::healthz))
        .nest("/github", github::routes())
        .nest("/google-calendar", google_calendar::routes(state))
}
