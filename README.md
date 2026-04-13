# ⚡ FlexPrice CLI

A beautiful terminal CLI for the [FlexPrice](https://flexprice.io) usage-based billing platform. Manage customers, plans, subscriptions, invoices, meters, events, wallets, features, and entitlements — all from your terminal. Includes an interactive TUI dashboard built with [Ink](https://github.com/vadimdemedes/ink).

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

- **Node.js 20+** — [nodejs.org](https://nodejs.org) or your version manager
- Access to the **FlexPrice API** (default base URL: `https://api.cloud.flexprice.io`)

## Installation

### From this repository

```bash
git clone https://github.com/flexprice/flexprice-cli.git
cd flexprice-cli/npm
npm install
npm run build
```

Run the CLI:

```bash
node dist/cli-entry.js --help
```

Link globally from the `npm/` directory:

```bash
cd flexprice-cli/npm
npm link
flexprice --help
```

Or install the folder as a global package:

```bash
npm install -g ./npm
```

### After publishing to npm

```bash
npm install -g flexprice-cli
```

(See [npm/README.md](npm/README.md) for publishing notes.)

---

## Authentication

Before using most commands, configure an **API key**. The default API URL is `https://api.cloud.flexprice.io`.

### API key

```bash
flexprice auth set-api-key <YOUR_API_KEY>
# Override the API URL (optional):
flexprice auth set-api-key <YOUR_API_KEY> --api-url https://api.cloud.flexprice.io
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
| `auth set-api-key <KEY>` | Store an API key (default URL: cloud API) |
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

Displays the current configuration: API URL, masked API key, environment ID, and credentials file path.

### Dashboard (TUI)

```bash
flexprice dashboard
```

Launches an interactive terminal dashboard powered by [Ink](https://github.com/vadimdemedes/ink). Navigate between panels showing customers, plans, subscriptions, invoices, and more using keyboard controls.

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab`, `l` / `h` | Switch between resource tabs |
| `↑` / `↓`, `j` / `k` | Navigate lists |
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
export FLEXPRICE_API_URL=https://api.cloud.flexprice.io
export FLEXPRICE_API_KEY=fp_live_xxxxxxxxxxxx
export FLEXPRICE_ENVIRONMENT_ID=env_prod
```

Or use a `.env` file in your working directory:

```dotenv
FLEXPRICE_API_URL=https://api.cloud.flexprice.io
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
├── README.md
└── npm/                    # Node.js package (TypeScript)
    ├── package.json
    ├── tsconfig.json
    ├── tsup.config.ts
    ├── src/
    │   ├── cli-entry.ts    # Process entry (dotenv + run)
    │   ├── main.ts         # Commander program & routing
    │   ├── api/
    │   │   ├── client.ts   # HTTP client (fetch)
    │   │   └── models.ts   # API types
    │   ├── cli/            # Command handlers (auth, customers, …)
    │   ├── config/
    │   │   └── store.ts    # Credentials file & merge order
    │   ├── tui/
    │   │   ├── dashboard.tsx
    │   │   └── theme.ts
    │   └── utils/
    │       ├── output.ts
    │       └── spinner.ts
    └── dist/               # Build output (after `npm run build`)
```

---

## License

MIT
