# Operations Runbook

## Start core service

```bash
cd services/receiver-core
AIR_SENDER_API_TOKEN=<strong-token> cargo run
```

## Start desktop app

```bash
cd apps/desktop
npm install
AIR_SENDER_API_TOKEN=<same-token> npm run dev
```

## Runtime checks

- Health: `GET /health`
- Detailed health: `GET /health/detail`
- Operational summary: `GET /v1/dashboard`
- Full debug snapshot: `GET /v1/debug/snapshot`

## Incident triage checklist

1. Verify API token alignment across processes.
2. Fetch audit tail and inspect denied/security events.
3. Check policy for restrictive `trusted-only` or low `max_sessions`.
4. Validate sessions are transitioning pending -> active -> stopped.
5. Validate recordings are only attached to active sessions.

## Restart strategy

- Local dev: stop/restart process.
- Container: `docker compose restart receiver-core`.

## Data persistence note

Current implementation uses in-memory state and does not persist sessions/trust/policy across restarts.
