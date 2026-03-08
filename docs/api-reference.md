# API Reference

Base URL: `http://127.0.0.1:9760`

## Authentication

Mutating endpoints require:

```http
Authorization: Bearer <AIR_SENDER_API_TOKEN>
```

## Health

- `GET /health` -> basic liveness string
- `GET /health/detail` -> liveness + server timestamp + uptime seconds

## Dashboard + Debug

- `GET /v1/dashboard`
  - Counts protocols, sessions by state, trusted devices, active recordings.
- `GET /v1/debug/snapshot`
  - Full current in-memory state snapshot and trailing audit events.
  - Intended for local debugging and support diagnostics.

## Protocols

- `GET /v1/protocols`
  - Lists `airplay`, `cast`, `miracast` descriptors + capability flags.

## Sessions

- `GET /v1/sessions`
  - Returns all known sessions.
- `POST /v1/sessions` (auth)
  - Creates a pending mock session.
  - Enforces policy max sessions (pending+active).

Request:

```json
{
  "protocol": "cast",
  "device_name": "Pixel 8",
  "device_platform": "Android"
}
```

- `POST /v1/sessions/{id}/accept` (auth)
- `POST /v1/sessions/{id}/stop` (auth)

## Recording

- `GET /v1/recordings`
- `POST /v1/recordings/start` (auth)
  - Session must exist and be `active`.
- `POST /v1/recordings/stop` (auth)

Start request:

```json
{
  "session_id": "uuid",
  "profile": {
    "destination_path": "./recordings",
    "quality_preset": "balanced",
    "codec": "h264",
    "container": "mp4"
  }
}
```

## Trust list

- `GET /v1/trust`
- `POST /v1/trust/{deviceId}` (auth)
- `DELETE /v1/trust/{deviceId}` (auth)

## Policy

- `GET /v1/policy`
- `PATCH /v1/policy` (auth)

Request:

```json
{
  "acceptance": "ask",
  "max_sessions": 4
}
```

`max_sessions` must be `1..=4`.

## Audit

- `GET /v1/audit`
  - Returns chronological event log.
