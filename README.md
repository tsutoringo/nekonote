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

### Google Calendar

Google Calendar support is mounted under `/google-calendar/mcp` and uses Service Account key
authentication. It only accesses calendars that have been explicitly shared with the service
account email.

Implemented MCP tools:

| Tool | Purpose |
| --- | --- |
| `calendar_get` | Get metadata for a shared calendar |
| `event_list` | List events in a shared calendar |
| `event_get` | Get one event |
| `event_create` | Create an event |
| `event_update` | Patch an event |
| `event_delete` | Delete an event |
| `freebusy_query` | Query busy blocks |

## Configuration

Configuration is loaded from `config.toml` and can be overridden with environment variables using the `NEKONOTE__` prefix and `__` separators.

Minimal example:

```toml
[server]
addr = "0.0.0.0:19502"
# Optional. Hostnames or host:port authorities accepted by Streamable HTTP MCP services.
# rmcp defaults to loopback-only Host validation, so include the externally used service name.
allowed_hosts = ["localhost", "127.0.0.1", "nekonote.moltis.svc.cluster.local"]

[provider.github]
app_id = 123456
installation_id = 12345678
app_key = "BASE64_ENCODED_RSA_PRIVATE_KEY_PEM"
# Optional. Defaults to https://api.githubcopilot.com/mcp/
mcp_endpoint = "https://api.githubcopilot.com/mcp/"

[provider.google_calendar.auth]
type = "service_account"
key_path = "/path/to/service-account-key.json"
```

Environment variable equivalents:

```bash
export NEKONOTE__SERVER__ADDR=0.0.0.0:19502
export NEKONOTE__SERVER__ALLOWED_HOSTS=localhost,127.0.0.1,nekonote.moltis.svc.cluster.local
export NEKONOTE__PROVIDER__GITHUB__APP_ID=123456
export NEKONOTE__PROVIDER__GITHUB__INSTALLATION_ID=12345678
export NEKONOTE__PROVIDER__GITHUB__APP_KEY=BASE64_ENCODED_RSA_PRIVATE_KEY_PEM
export NEKONOTE__PROVIDER__GOOGLE_CALENDAR__AUTH__TYPE=service_account
export NEKONOTE__PROVIDER__GOOGLE_CALENDAR__AUTH__KEY_PATH=/path/to/service-account-key.json
```

`provider.github.app_key` is expected to be the base64 encoding of the GitHub App RSA private key PEM.

## Running

```bash
cargo run
```

Docker:

```bash
docker build -f Dockerfile -t nekonote:local .
docker run --rm -p 19502:19502 \
  -e NEKONOTE__SERVER__ADDR=0.0.0.0:19502 \
  -e NEKONOTE__SERVER__ALLOWED_HOSTS=localhost,127.0.0.1,nekonote.moltis.svc.cluster.local \
  -e NEKONOTE__PROVIDER__GOOGLE_CALENDAR__AUTH__TYPE=service_account \
  -e NEKONOTE__PROVIDER__GOOGLE_CALENDAR__AUTH__KEY_PATH=/run/secrets/google-calendar.json \
  -v /path/to/service-account-key.json:/run/secrets/google-calendar.json:ro \
  nekonote:local
```

Health check:

```bash
curl http://127.0.0.1:19502/healthz
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
