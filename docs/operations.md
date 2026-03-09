# Operations Runbook

## 1) Service health signals

Primary checks:

- `GET /health` for process liveness
- `GET /v1/dashboard` for state overview
- `GET /v1/audit` for action and denial timeline

---

## 2) Daily/regular operational checks

1. Confirm service responds on expected bind address.
2. Validate dashboard metrics are coherent (counts align with session states).
3. Confirm audit endpoint is updating after mutations.
4. Validate unauthorized mutation attempts are denied (`401`).

---

## 3) Incident triage workflow

### Symptom: UI cannot perform actions

- Check `AIR_SENDER_API_TOKEN` consistency between desktop and core.
- Verify core endpoint URL (`AIR_SENDER_CORE_URL`).
- Confirm core process is reachable and healthy.

### Symptom: recording start fails

- Verify target session exists and is `active`.
- Check API response code:
  - `404` session missing
  - `409` session not active

### Symptom: policy update rejected

- Ensure `max_sessions` in payload is between 1 and 4.

---

## 4) Log and audit usage

- Use receiver-core logs (stdout) for runtime-level troubleshooting.
- Use `/v1/audit` to correlate user actions and denied requests.
- Persist logs externally in production environments for retention and search.

---

## 5) Security operations guidance

- Rotate API tokens on a regular schedule.
- Avoid exposing core directly to internet-facing interfaces.
- Prefer VPN/mTLS for remote control scenarios.
- Periodically test that unauthenticated mutations remain blocked.

---

## 6) Backup and persistence note

Current implementation is in-memory; restart clears sessions, trust list, recordings, and audit history.

Operational implication:

- Treat current state as ephemeral.
- Do not rely on restart durability until persistent storage is introduced.


## Added Month 1/2 operational surfaces

- Operator controls endpoint: `GET/PATCH /v1/operator/settings`.
- Pairing PIN endpoint: `POST /v1/pairing/pin`.
- Signed profile endpoints: `POST /v1/config-profiles/sign` and `POST /v1/config-profiles/verify`.
- Per-protocol enablement: `PATCH /v1/protocols/{id}`.
