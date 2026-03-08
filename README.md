# Air Sender Receiver

Air Sender Receiver is a monorepo for a cross-platform desktop receiver experience:

- **`receiver-core`**: a Rust HTTP service that models receiver protocols, sessions, trust policy, recording state, and audit history.
- **`apps/desktop`**: an Electron desktop control plane that calls `receiver-core` and renders a live operator dashboard.

> Current scope: this repository provides a robust management/control surface and a simulated runtime model (seeded sessions, protocol metadata, policy handling), designed to support real adapter/media integrations in future milestones.

## Documentation map

- [Quick start](#quick-start)
- [Repository layout](#repository-layout)
- [Configuration](#configuration)
- [Running with Docker Compose](#running-with-docker-compose)
- [API overview](#api-overview)
- [Security model](#security-model)
- [Development workflow](#development-workflow)
- [Troubleshooting](#troubleshooting)
- [Roadmap docs](#roadmap-docs)

Additional detailed docs:

- `docs/architecture.md`
- `docs/api-reference.md`
- `docs/development.md`
- `docs/deployment.md`
- `docs/operations.md`
- `docs/release-plan.md`

---

## Quick start

### Prerequisites

- **Rust** toolchain (stable; `cargo` available)
- **Node.js** 18+ and npm
- Optional: Docker + Docker Compose

### 1) Run receiver-core locally

```bash
cd services/receiver-core
AIR_SENDER_API_TOKEN=dev-token cargo run
```

By default the service binds to `127.0.0.1:9760`.

### 2) Run the desktop app

In a second terminal:

```bash
cd apps/desktop
npm install
AIR_SENDER_API_TOKEN=dev-token npm run dev
```

The Electron app opens a dashboard and communicates with `http://127.0.0.1:9760`.

### 3) Verify service health

```bash
curl http://127.0.0.1:9760/health
```

Expected response:

```text
ok
```

---

## Repository layout

```text
air-sender/
├─ apps/
│  └─ desktop/             # Electron UI/control plane
├─ services/
│  └─ receiver-core/       # Rust API service
├─ docs/                   # Architecture, deployment, API, ops, roadmap
├─ docker-compose.yml
└─ receiver-config.v1.json
```

---

## Configuration

`receiver-core` reads these environment variables:

- `AIR_SENDER_BIND` (default: `127.0.0.1:9760`)
- `AIR_SENDER_API_TOKEN` (default in code: `dev-token`; set explicitly for non-dev usage)
- `RUST_LOG` (optional log verbosity; e.g., `info`, `debug`, `trace`)

`apps/desktop` reads:

- `AIR_SENDER_API_BASE` (primary) or `AIR_SENDER_CORE_URL` (compat fallback), default: `http://127.0.0.1:9760`
- `AIR_SENDER_API_TOKEN` (used for authenticated mutating calls)

---

## Running with Docker Compose

From repository root:

```bash
docker compose up --build -d
```

The provided compose file exposes `receiver-core` on `9760` and injects development defaults.

---

## API overview

Public/readonly endpoints:

- `GET /health`
- `GET /v1/dashboard`
- `GET /v1/protocols`
- `GET /v1/policy`
- `GET /v1/sessions`
- `GET /v1/recordings`
- `GET /v1/trust`
- `GET /v1/audit`

Mutating endpoints (require `Authorization: Bearer <AIR_SENDER_API_TOKEN>`):

- `POST /v1/sessions`
- `POST /v1/sessions/{id}/accept`
- `POST /v1/sessions/{id}/stop`
- `PATCH /v1/protocols/{id}`
- `POST /v1/recordings/start`
- `POST /v1/recordings/stop`
- `POST /v1/trust/{deviceId}`
- `DELETE /v1/trust/{deviceId}`
- `PATCH /v1/policy`

For request/response payloads and examples, see `docs/api-reference.md`.

---

## Security model

- `receiver-core` is intended to run on localhost by default.
- State-changing operations require a bearer token.
- Unauthorized mutating attempts are denied and written to audit.
- Policy supports acceptance strategies: `auto`, `ask`, `trusted-only`.

For production hardening practices, see `docs/deployment.md` and `docs/operations.md`.

---

## Development workflow

Recommended sequence:

1. Run `receiver-core` with a known token.
2. Run Electron app with the same token.
3. Use dashboard actions to create, accept, stop sessions, and manage trust/policy.
4. Validate behavior with integration-like calls from `curl`.
5. Run `cargo test` in `services/receiver-core`.

See `docs/development.md` for full setup, verification checklist, and change strategy.

---

## Troubleshooting

### Desktop shows auth errors

Ensure both processes use the same `AIR_SENDER_API_TOKEN`.

### Desktop cannot connect

Confirm `receiver-core` is running and reachable at `AIR_SENDER_CORE_URL`.

### Port conflict on 9760

Set a custom bind:

```bash
AIR_SENDER_BIND=127.0.0.1:9876 AIR_SENDER_API_TOKEN=dev-token cargo run
```

And point desktop to:

```bash
AIR_SENDER_CORE_URL=http://127.0.0.1:9876 AIR_SENDER_API_TOKEN=dev-token npm run dev
```

---

## Roadmap docs

- Architecture and subsystem boundaries: `docs/architecture.md`
- Deployment and hardening guidance: `docs/deployment.md`
- Operational runbook and monitoring checks: `docs/operations.md`
- 12-month phased release plan: `docs/release-plan.md`
