# Air Sender Receiver

Open-source cross-platform desktop receiver for **AirPlay + Google Cast + Miracast** with an Electron control plane and a native Rust receiver core.

## Features

- Versioned local control API in Rust (`/v1/...`)
- Session lifecycle: create, accept, stop
- Trust lifecycle: list, trust, revoke
- Recording lifecycle: list, start, stop
- Policy controls (`acceptance`, `max_sessions`)
- Audit trail and debug snapshot endpoint
- Detailed health endpoint with uptime
- Modern desktop dashboard UX with live actions
- Container deployment assets (`Dockerfile`, `docker-compose.yml`)

## Quick start

### Run core service

```bash
cd services/receiver-core
AIR_SENDER_API_TOKEN=dev-token cargo run
```

### Run desktop app

```bash
cd apps/desktop
npm install
AIR_SENDER_API_TOKEN=dev-token npm run dev
```

## Core endpoints

- `GET /health`
- `GET /health/detail`
- `GET /v1/dashboard`
- `GET /v1/debug/snapshot`
- `GET /v1/protocols`
- `GET /v1/sessions`
- `POST /v1/sessions`
- `POST /v1/sessions/{id}/accept`
- `POST /v1/sessions/{id}/stop`
- `GET /v1/recordings`
- `POST /v1/recordings/start`
- `POST /v1/recordings/stop`
- `GET /v1/trust`
- `POST /v1/trust/{deviceId}`
- `DELETE /v1/trust/{deviceId}`
- `GET /v1/policy`
- `PATCH /v1/policy`
- `GET /v1/audit`

## Documentation

- Architecture: `docs/architecture.md`
- Release plan: `docs/release-plan.md`
- Deployment: `docs/deployment.md`
- API reference: `docs/api-reference.md`
- Debugging: `docs/debugging.md`
- Operations runbook: `docs/operations-runbook.md`
- Security notes: `docs/security.md`
- Development guide: `docs/development.md`
