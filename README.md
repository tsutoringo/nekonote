# nekonote

`nekonote` is a small provider-aware HTTP proxy for services that need server-side credentials. It currently supports GitHub App installation authentication and is structured so additional providers can be added under the same routing and configuration model.

## Current Providers

### GitHub

GitHub support is mounted under `/github`.

| Route | Upstream | Auth |
| --- | --- | --- |
| `ANY /github/mcp` | GitHub Copilot MCP endpoint | `Bearer <installation token>` |
| `ANY /github/repos/{*path}` | `https://github.com/{path}` | Basic auth as `x-access-token:<installation token>` |

The repos proxy supports Git smart HTTP operations, so commands such as `git ls-remote` and `git clone` can use the local URL:

```bash
git ls-remote http://127.0.0.1:19502/github/repos/OWNER/REPO.git
git clone http://127.0.0.1:19502/github/repos/OWNER/REPO.git
```

Repository access is intentionally delegated to the GitHub App installation permissions.

## Configuration

Configuration is loaded from `config.toml` and can be overridden with environment variables using the `NEKONOTE__` prefix and `__` separators.

Minimal example:

```toml
[server]
addr = "0.0.0.0:19502"

[provider.github]
app_id = 123456
installation_id = 12345678
app_key = "BASE64_ENCODED_RSA_PRIVATE_KEY_PEM"
# Optional. Defaults to https://api.githubcopilot.com/mcp/
mcp_endpoint = "https://api.githubcopilot.com/mcp/"
```

Environment variable equivalents:

```bash
export NEKONOTE__SERVER__ADDR=0.0.0.0:19502
export NEKONOTE__PROVIDER__GITHUB__APP_ID=123456
export NEKONOTE__PROVIDER__GITHUB__INSTALLATION_ID=12345678
export NEKONOTE__PROVIDER__GITHUB__APP_KEY=BASE64_ENCODED_RSA_PRIVATE_KEY_PEM
```

`provider.github.app_key` is expected to be the base64 encoding of the GitHub App RSA private key PEM.

## Running

```bash
cargo run
```

Health check:

```bash
curl http://127.0.0.1:19502/healthz
```

GitHub MCP smoke test:

```bash
curl -i -N \
  -X POST http://127.0.0.1:19502/github/mcp \
  -H 'content-type: application/json' \
  -H 'accept: application/json, text/event-stream' \
  -H 'mcp-protocol-version: 2025-06-18' \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"nekonote-smoke","version":"0.1.0"}}}'
```

## Provider Model

Providers are represented in three places:

1. `src/config.rs`
   Add provider-specific config under `ProviderConfig`.

2. `src/main.rs`
   Add provider-specific runtime state under `ProviderState`.

3. `src/routes/<provider>.rs`
   Add routes and proxy handlers for that provider.

The current GitHub implementation uses this layout:

```text
provider.github config
  -> GitHubState
  -> /github routes
```

Future providers should follow the same pattern:

```text
provider.<name> config
  -> <Name>State
  -> /<name> routes
```

This keeps provider credentials, token acquisition, and route behavior isolated while sharing the application-level HTTP client and error handling style.

## Proxy Rules

Proxy handlers should preserve streaming in both directions:

- Request bodies should be forwarded as streams.
- Upstream response bodies should be returned as streams.
- Hop-by-hop headers must not be forwarded.
- Client-supplied `Authorization` should not be forwarded when the provider injects server-side credentials.

Current forwarded request headers are intentionally allowlisted:

- `accept`
- `content-type`
- `user-agent`
- MCP headers: `mcp-session-id`, `mcp-protocol-version`, `last-event-id`
- Git header: `git-protocol`

## Verification

```bash
cargo fmt --check
cargo check
```

GitHub repos proxy:

```bash
git ls-remote http://127.0.0.1:19502/github/repos/OWNER/REPO.git
git clone --depth 1 http://127.0.0.1:19502/github/repos/OWNER/REPO.git /tmp/nekonote-clone-test
```
