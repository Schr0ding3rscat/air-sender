# Air Sender Receiver

Open-source cross-platform desktop receiver for **AirPlay + Google Cast + Miracast** with an Electron control plane and a native Rust receiver core.

## What is now implemented

- Native `receiver-core` service with versioned local API (`/v1/...`)
- Session lifecycle controls (create, accept, stop)
- Trust-list management (trust + revoke)
- Recording lifecycle controls (start, stop, list)
- Receiver policy controls (`acceptance`, `max_sessions`)
- Dashboard summary endpoint for operational UX
- Audit trail for security/session/recording/policy events
- Polished desktop dashboard with live controls and KPI cards
- Deployment assets (`Dockerfile`, `docker-compose.yml`, deployment guide)

## Monorepo layout

- `apps/desktop`: Electron app (control plane)
- `services/receiver-core`: Native Rust service
- `docs`: architecture, release plan, deployment guide

## Quick start

### 1) Start receiver core

```bash
cd services/receiver-core
AIR_SENDER_API_TOKEN=dev-token cargo run
```

### 2) Start desktop app

```bash
cd apps/desktop
npm install
AIR_SENDER_API_TOKEN=dev-token npm run dev
```

## API highlights

- `GET /health`
- `GET /v1/dashboard`
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

## Deployment

See `docs/deployment.md` and `docker-compose.yml`.
