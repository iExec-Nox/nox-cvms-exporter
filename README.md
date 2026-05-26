# nox-cvms-exporter

A lightweight Rust HTTP API (Axum) that sits in front of a [dstack-vmm](https://github.com/Dstack-TEE/dstack) instance and exposes a simplified view of active Confidential VMs (CVMs).

## Overview

`nox-cvms-exporter` queries the dstack-vmm RPC API, filters out infrastructure VMs and stopped instances, and returns a curated list of CVMs along with the routing configuration needed by clients.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Service name and current UTC timestamp |
| `GET` | `/health` | Liveness probe — returns `{"status":"ok"}` |
| `GET` | `/cvms` | Active CVMs + routing config |

### `GET /cvms`

Returns active CVMs grouped by application. Each entry lists all running instances of that app.

**Response**

```json
[
  {
    "app_id": "a1b2c3...",
    "name": "my-app",
    "instances": [
      {
        "instance_id": "i-0abc123",
        "url": "https://i-0abc123-9999.apps.my-domain.example.com",
        "machine_id": "machine-42"
      },
      {
        "instance_id": "i-0def456",
        "url": "https://i-0def456-9999.apps.my-domain.example.com",
        "machine_id": "machine-42"
      }
    ]
  }
]
```

**Filtering rules**

A CVM is excluded from the response if:
- its `status` is `stopped` or `removed`, or
- its `name` is exactly `kms` or `dstack-gateway` (infrastructure VMs)

## Configuration

All settings are loaded from environment variables prefixed with `NOX_CVMS_EXPORTER_`.  
Nested keys use `__` as separator (e.g. `NOX_CVMS_EXPORTER_SERVER__PORT=9000`).

| Environment variable | Default | Description |
|---|---|---|
| `NOX_CVMS_EXPORTER__HOST` | `0.0.0.0` | Host to bind the HTTP server to |
| `NOX_CVMS_EXPORTER_SERVER__PORT` | `8080` | Port to bind the HTTP server to |
| `NOX_CVMS_EXPORTER_DSTACK_VMM_URL` | `http://127.0.0.1:9080` | Base URL of the dstack-vmm instance |
| `NOX_CVMS_EXPORTER_SUFFIXE_URL` | _(empty)_ | URL suffix appended to CVM-facing requests |
| `NOX_CVMS_EXPORTER_QUOTE_SERVICE_PORT` | `9999` | Port of the quote service exposed by CVMs |
| `NOX_CVMS_EXPORTER_MACHINE_ID` | _(empty)_ | Identifier of the machine running this exporter |

## Running

```bash
cargo run --release
```

Override defaults as needed:

```bash
NOX_CVMS_EXPORTER_DSTACK_VMM_URL=http://127.0.0.1:9080 \
NOX_CVMS_EXPORTER_SUFFIXE_URL=apps.ovh-tdx-dev.noxprotocol.dev \
cargo run --release
```

