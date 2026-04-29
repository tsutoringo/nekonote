use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::{
        HeaderMap, HeaderName, StatusCode,
        header::{
            ACCEPT, AUTHORIZATION, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, HOST,
            TRANSFER_ENCODING, UPGRADE, USER_AGENT,
        },
    },
    response::{IntoResponse, Response},
    routing::any,
};
use secrecy::ExposeSecret;

use crate::AppState;

pub fn routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/mcp", any(handle_mcp_proxy))
        .route("/repos/{*path}", any(handle_github_repos_proxy))
}

async fn handle_mcp_proxy(
    State(state): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, ProxyError> {
    let github = state
        .providers
        .github
        .as_ref()
        .ok_or(ProxyError::GitHubProviderNotConfigured)?;

    let token = github
        .octocrab
        .installation_token()
        .await
        .map_err(|_| ProxyError::InstallationToken)?;

    let (parts, body) = req.into_parts();
    let mut upstream_url =
        reqwest::Url::parse(&github.mcp_endpoint).map_err(|_| ProxyError::InvalidEndpoint)?;
    upstream_url.set_query(parts.uri.query());

    let body = reqwest::Body::wrap_stream(body.into_data_stream());

    let mut upstream = state
        .client
        .request(parts.method, upstream_url)
        .bearer_auth(token.expose_secret())
        .body(body);

    upstream = copy_request_headers(upstream, &parts.headers);

    let upstream_response = upstream.send().await.map_err(|_| ProxyError::Upstream)?;
    proxy_response(upstream_response)
}

async fn handle_github_repos_proxy(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    req: Request,
) -> Result<Response, ProxyError> {
    let github = state
        .providers
        .github
        .as_ref()
        .ok_or(ProxyError::GitHubProviderNotConfigured)?;

    let token = github
        .octocrab
        .installation_token()
        .await
        .map_err(|_| ProxyError::InstallationToken)?;

    let (parts, body) = req.into_parts();
    let mut upstream_url =
        reqwest::Url::parse("https://github.com/").map_err(|_| ProxyError::InvalidEndpoint)?;
    upstream_url.set_path(&path);
    upstream_url.set_query(parts.uri.query());

    let body = reqwest::Body::wrap_stream(body.into_data_stream());

    let mut upstream = state
        .client
        .request(parts.method, upstream_url)
        .basic_auth("x-access-token", Some(token.expose_secret()))
        .body(body);

    upstream = copy_request_headers(upstream, &parts.headers);

    let upstream_response = upstream.send().await.map_err(|_| ProxyError::Upstream)?;
    proxy_response(upstream_response)
}

fn copy_request_headers(
    mut upstream: reqwest::RequestBuilder,
    headers: &HeaderMap,
) -> reqwest::RequestBuilder {
    for (name, value) in headers.iter() {
        if should_forward_request_header(name) {
            upstream = upstream.header(name, value);
        }
    }

    upstream
}

fn should_forward_request_header(name: &HeaderName) -> bool {
    matches!(name, &ACCEPT | &CONTENT_TYPE | &USER_AGENT)
        || is_mcp_header(name)
        || is_git_header(name)
}

fn proxy_response(upstream_response: reqwest::Response) -> Result<Response, ProxyError> {
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();

    let mut response = Response::builder().status(status);
    for (name, value) in headers.iter() {
        if should_forward_response_header(name) {
            response = response.header(name, value);
        }
    }

    response
        .body(Body::from_stream(upstream_response.bytes_stream()))
        .map_err(|_| ProxyError::BuildResponse)
}

fn should_forward_response_header(name: &HeaderName) -> bool {
    !is_hop_by_hop_header(name)
}

fn is_mcp_header(name: &HeaderName) -> bool {
    name.as_str().eq_ignore_ascii_case("mcp-session-id")
        || name.as_str().eq_ignore_ascii_case("mcp-protocol-version")
        || name.as_str().eq_ignore_ascii_case("last-event-id")
}

fn is_git_header(name: &HeaderName) -> bool {
    name.as_str().eq_ignore_ascii_case("git-protocol")
}

fn is_hop_by_hop_header(name: &HeaderName) -> bool {
    matches!(
        name,
        &AUTHORIZATION | &CONNECTION | &CONTENT_LENGTH | &HOST | &TRANSFER_ENCODING | &UPGRADE
    ) || name.as_str().eq_ignore_ascii_case("keep-alive")
}

#[derive(Debug)]
enum ProxyError {
    GitHubProviderNotConfigured,
    InvalidEndpoint,
    InstallationToken,
    Upstream,
    BuildResponse,
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ProxyError::GitHubProviderNotConfigured => (
                StatusCode::SERVICE_UNAVAILABLE,
                "GitHub provider is not configured",
            ),
            ProxyError::InvalidEndpoint => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "GitHub upstream endpoint is invalid",
            ),
            ProxyError::InstallationToken => (
                StatusCode::BAD_GATEWAY,
                "Failed to acquire GitHub installation token",
            ),
            ProxyError::Upstream => (StatusCode::BAD_GATEWAY, "GitHub MCP upstream failed"),
            ProxyError::BuildResponse => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build proxy response",
            ),
        };

        (status, message).into_response()
    }
}
