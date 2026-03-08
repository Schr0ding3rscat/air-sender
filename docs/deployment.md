# Deployment Guide

## Container deployment (receiver-core)

```bash
docker compose up --build -d
```

Service listens on `:9760` and exposes health endpoint:

```bash
curl http://127.0.0.1:9760/health
```

## Production environment variables

- `AIR_SENDER_BIND` (default `127.0.0.1:9760`)
- `AIR_SENDER_API_TOKEN` (required: set a strong random token)
- `RUST_LOG` (for debug: `info`, `debug`, `trace`)

## Reverse-proxy recommendation

- Keep core bound to localhost in desktop setups.
- If remote admin is needed, terminate TLS at a reverse proxy and restrict by mTLS/VPN.

## Operational checks

- `/health` for process liveness
- `/v1/dashboard` for operational overview
- `/v1/audit` for security and lifecycle traceability
