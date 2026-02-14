# ⚡ FlexPrice CLI

A beautiful terminal CLI for the [FlexPrice](https://flexprice.io) usage-based billing platform. Manage customers, plans, subscriptions, invoices, meters, events, wallets, features, and entitlements — all from your terminal. Includes an interactive TUI dashboard built with [Ratatui](https://ratatui.rs).

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Authentication](#authentication)
- [Commands](#commands)
  - [Auth](#auth)
  - [Customers](#customers)
  - [Plans](#plans)
  - [Subscriptions](#subscriptions)
  - [Invoices](#invoices)
  - [Meters](#meters)
  - [Events](#events)
  - [Wallets](#wallets)
  - [Features](#features)
  - [Entitlements](#entitlements)
  - [Config](#config)
  - [Dashboard (TUI)](#dashboard-tui)
- [Configuration](#configuration)
- [Global Options](#global-options)
- [JSON Output](#json-output)
- [Project Structure](#project-structure)
- [License](#license)

---

## Prerequisites

- **Rust toolchain** (1.70+) — install via [rustup](https://rustup.rs)
- A running **FlexPrice API** instance (default: `http://localhost:8080`)

## Installation

### Build from source

```bash
git clone https://github.com/flexprice/flexprice-cli.git
cd flexprice-cli
cargo build --release
```

The binary will be at `target/release/flexprice`. You can copy it to a directory in your `$PATH`:

```bash
cp target/release/flexprice /usr/local/bin/
```

### Run directly with Cargo

```bash
cargo run -- <COMMAND>
```

---

## Authentication

Before using any resource commands, you need to authenticate. The CLI supports two authentication methods:

### Interactive login (email + password)

```bash
flexprice auth login
# Optionally specify a custom API URL:
flexprice auth login --api-url https://api.flexprice.io
```

You'll be prompted for your email and password. On success, a JWT token is saved locally.

### API key (CI/CD & automation)

```bash
flexprice auth set-api-key <YOUR_API_KEY>
# With a custom API URL:
flexprice auth set-api-key <YOUR_API_KEY> --api-url https://api.flexprice.io
```

The key is validated against the API before being stored.

### Check auth status

```bash
flexprice auth status   # Show credentials & test API connection
flexprice auth whoami   # Show current user details
flexprice auth logout   # Remove stored credentials
```

---

## Commands

### Auth

| Command | Description |
|---------|-------------|
| `auth login` | Interactive login with email & password |
| `auth set-api-key <KEY>` | Store an API key directly |
| `auth whoami` | Show authenticated user info |
| `auth status` | Show auth status & test connection |
| `auth logout` | Remove stored credentials |

### Customers

| Command | Description |
|---------|-------------|
| `customers list` | List all customers |
| `customers get <ID>` | Get a customer by ID |
| `customers create --json <FILE>` | Create a customer from a JSON file |
| `customers delete <ID>` | Delete a customer |
| `customers usage <ID>` | View customer usage summary |
| `customers entitlements <ID>` | View customer entitlements |

**Example — create a customer:**

```bash
cat > customer.json << 'EOF'
{
  "name": "Acme Corp",
  "email": "billing@acme.com",
  "external_id": "acme-001"
}
EOF

flexprice customers create --json customer.json
```

### Plans

| Command | Description |
|---------|-------------|
| `plans list` | List all pricing plans |
| `plans get <ID>` | Get a plan by ID |
| `plans create --json <FILE>` | Create a plan from a JSON file |
| `plans delete <ID>` | Delete a plan |

### Subscriptions

| Command | Description |
|---------|-------------|
| `subscriptions list` | List all subscriptions |
| `subscriptions get <ID>` | Get a subscription by ID |
| `subscriptions create --json <FILE>` | Create a subscription from a JSON file |
| `subscriptions cancel <ID>` | Cancel a subscription |
| `subscriptions usage --json <FILE>` | Query subscription usage |

### Invoices

| Command | Description |
|---------|-------------|
| `invoices list` | List all invoices |
| `invoices get <ID>` | Get an invoice by ID |
| `invoices finalize <ID>` | Finalize a draft invoice |
| `invoices void <ID>` | Void an invoice |
| `invoices pdf <ID>` | Download invoice as PDF |

**Download a PDF:**

```bash
flexprice invoices pdf inv_abc123 --output ./invoice.pdf
```

### Meters

| Command | Description |
|---------|-------------|
| `meters list` | List all meters |
| `meters get <ID>` | Get a meter by ID |
| `meters create --json <FILE>` | Create a meter from a JSON file |
| `meters delete <ID>` | Delete a meter |

### Events

| Command | Description |
|---------|-------------|
| `events ingest --json <FILE>` | Ingest a single event |
| `events ingest-bulk --json <FILE>` | Bulk ingest events |
| `events list` | List recent events |
| `events get <ID>` | Get an event by ID |
| `events usage --json <FILE>` | Query event usage |

**Example — ingest an event:**

```bash
cat > event.json << 'EOF'
{
  "event_name": "api_call",
  "external_customer_id": "acme-001",
  "properties": {
    "tokens": 150,
    "model": "gpt-4"
  }
}
EOF

flexprice events ingest --json event.json
```

### Wallets

| Command | Description |
|---------|-------------|
| `wallets list` | List all wallets |
| `wallets get <ID>` | Get a wallet by ID |
| `wallets create --json <FILE>` | Create a wallet from a JSON file |
| `wallets top-up <ID> --json <FILE>` | Top up a wallet |
| `wallets balance <ID>` | Get real-time wallet balance |

### Features

| Command | Description |
|---------|-------------|
| `features list` | List all features |
| `features get <ID>` | Get a feature by ID |
| `features create --json <FILE>` | Create a feature from a JSON file |
| `features delete <ID>` | Delete a feature |

### Entitlements

| Command | Description |
|---------|-------------|
| `entitlements list` | List all entitlements |
| `entitlements get <ID>` | Get an entitlement by ID |
| `entitlements create --json <FILE>` | Create an entitlement from a JSON file |
| `entitlements delete <ID>` | Delete an entitlement |

### Config

```bash
flexprice config
```

Displays the current configuration: API URL, masked API key, auth token status, tenant ID, user ID, environment ID, and credentials file path.

### Dashboard (TUI)

```bash
flexprice dashboard
```

Launches an interactive terminal dashboard powered by [Ratatui](https://ratatui.rs). Navigate between panels showing customers, subscriptions, invoices, and more using keyboard controls.

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch between panels |
| `↑` / `↓` | Navigate lists |
| `r` | Refresh data |
| `q` / `Esc` | Quit |

---

## Configuration

Credentials are resolved in the following priority order (highest → lowest):

| Priority | Source | Details |
|----------|--------|---------|
| 1 | **CLI flags** | `--api-url`, `--api-key` |
| 2 | **Environment variables** | `FLEXPRICE_API_URL`, `FLEXPRICE_API_KEY`, `FLEXPRICE_ENVIRONMENT_ID` |
| 3 | **`.env` file** | Loaded from the current working directory |
| 4 | **Credentials file** | `~/.flexprice/credentials.json` |

### Environment variables

```bash
export FLEXPRICE_API_URL=https://api.flexprice.io
export FLEXPRICE_API_KEY=fp_live_xxxxxxxxxxxx
export FLEXPRICE_ENVIRONMENT_ID=env_prod
```

Or use a `.env` file in your working directory:

```dotenv
FLEXPRICE_API_URL=https://api.flexprice.io
FLEXPRICE_API_KEY=fp_live_xxxxxxxxxxxx
FLEXPRICE_ENVIRONMENT_ID=env_prod
```

---

## Global Options

These flags can be used with any command:

```
--api-url <URL>    Override the API base URL
--api-key <KEY>    Override the API key
--help             Show help for any command
--version          Show CLI version
```

---

## JSON Output

Most `list` and `get` commands support a `--json` flag to output raw JSON instead of formatted tables:

```bash
flexprice customers list --json
flexprice invoices get inv_abc123 --json
```

---

## Project Structure

```
flexprice-cli/
├── Cargo.toml              # Dependencies & build config
├── src/
│   ├── main.rs             # CLI entry point & command routing
│   ├── api/
│   │   ├── client.rs       # HTTP client (reqwest-based)
│   │   └── models.rs       # API request/response types
│   ├── cli/
│   │   ├── auth.rs         # Authentication commands
│   │   ├── customers.rs    # Customer management
│   │   ├── plans.rs        # Plan management
│   │   ├── subscriptions.rs# Subscription management
│   │   ├── invoices.rs     # Invoice management
│   │   ├── meters.rs       # Meter management
│   │   ├── events.rs       # Event ingestion & queries
│   │   ├── wallets.rs      # Wallet & credit management
│   │   ├── features.rs     # Feature management
│   │   └── entitlements.rs # Entitlement management
│   ├── config/
│   │   └── store.rs        # Credential storage & resolution
│   ├── tui/
│   │   ├── dashboard.rs    # Interactive TUI dashboard
│   │   └── theme.rs        # TUI color theme
│   └── utils/
│       ├── output.rs       # Table/JSON formatting & colored output
│       └── spinner.rs      # Loading spinners
└── target/                 # Build output (gitignored)
```

---

## License

MIT
