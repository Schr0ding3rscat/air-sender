# Deployment Guide

## 1) Deployment modes

## Local developer mode

Run directly with Rust + Node tooling. Best for feature development and endpoint validation.

## Container mode

Use Docker Compose to run `receiver-core` as a service process.

---

## 2) Docker Compose deployment

From repository root:

```bash
docker compose up --build -d
```

Validate:

```bash
curl http://127.0.0.1:9760/health
```

Stop:

```bash
docker compose down
```

---

## 3) Required and optional environment variables

### receiver-core

- `AIR_SENDER_BIND`
  - Format: `host:port`
  - Default in code: `127.0.0.1:9760`
- `AIR_SENDER_API_TOKEN`
  - Required for production usage (must be strong and secret)
  - Used to authorize mutating API endpoints
- `RUST_LOG`
  - Optional log filter (e.g., `info`, `debug`, `trace`)

### desktop

- `AIR_SENDER_CORE_URL`
  - URL to receiver-core API (default `http://127.0.0.1:9760`)
- `AIR_SENDER_API_TOKEN`
  - Must match receiver-core token

---

## 4) Network and reverse proxy recommendations

- Prefer localhost bind for desktop-hosted control plane.
- If remote access is required:
  - terminate TLS at reverse proxy,
  - restrict source access with VPN/mTLS,
  - do not expose development tokens,
  - rotate tokens periodically.

---

## 5) Baseline production hardening

- Use non-default `AIR_SENDER_API_TOKEN` with high entropy.
- Keep mutating API inaccessible from untrusted networks.
- Centralize logs (stdout capture or sidecar collector).
- Configure restart policy (`unless-stopped` or platform equivalent).
- Perform periodic endpoint health and auth checks.

---

## 6) Post-deploy verification checklist

1. `GET /health` returns `ok`.
2. `GET /v1/dashboard` returns structured JSON.
3. Unauthenticated `POST /v1/trust/...` returns `401`.
4. Authenticated trust mutation succeeds (`200`).
5. Audit stream shows both denied and successful actions.

---

## 7) Upgrade strategy

- Rebuild/redeploy receiver-core container with new version.
- Run smoke checks from the verification checklist.
- Confirm desktop app remains API-compatible.
- Roll back if health/auth invariants fail.

## Month 3 release operations additions

- Use `scripts/release-installer-sign-and-rollback.sh` to generate release artifacts, signatures, and rollback markers.
- Before rollout, archive `/v1/audit/export` output for baseline operational audit.
- In incident response, capture `/v1/diagnostics/bundle` and attach to support ticket.
- Rollback trigger: any failed GA gate or sustained reconnect failures above SLO.
