# Debugging Guide

## Enable verbose logs

```bash
cd services/receiver-core
RUST_LOG=debug AIR_SENDER_API_TOKEN=dev-token cargo run
```

## Fast smoke checks

```bash
curl -s http://127.0.0.1:9760/health
curl -s http://127.0.0.1:9760/health/detail
curl -s http://127.0.0.1:9760/v1/dashboard
curl -s http://127.0.0.1:9760/v1/debug/snapshot
```

## Debug workflow

1. Confirm process started and port is open.
2. Check `/health/detail` uptime for restarts/flapping.
3. Inspect `/v1/debug/snapshot` for:
   - stale pending sessions
   - policy limiting session creation
   - recording map divergence from session status
4. Inspect `/v1/audit` for denied auth or policy failures.

## Common issues

### `401 unauthorized`
- Cause: missing or wrong bearer token.
- Fix: set `AIR_SENDER_API_TOKEN` consistently in both core and desktop shell.

### `max sessions reached`
- Cause: pending/active sessions >= policy limit.
- Fix: stop sessions or patch policy (`max_sessions <= 4`).

### `session must be active to start recording`
- Cause: recording attempted before acceptance.
- Fix: accept session first.

## Desktop debugging

- Open Electron devtools if needed from main window.
- Status banner in UI shows action-level errors returned by core.
