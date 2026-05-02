mod config;
mod error;
mod routes;
mod state;
mod util;

use std::sync::Arc;

use tokio::net::TcpListener;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::load()?;
    let state = Arc::new(AppState::new(&config).await?);

    let app = routes::routes(state.clone()).with_state(state);
    let listener = TcpListener::bind(config.server.addr).await?;

    println!("Server running on http://{}", config.server.addr);
    axum::serve(listener, app).await?;

    Ok(())
}
