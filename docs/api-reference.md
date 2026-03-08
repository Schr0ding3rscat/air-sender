# API Reference (`receiver-core`)

Base URL (default local): `http://127.0.0.1:9760`

## Authentication

Mutating endpoints require:

```http
Authorization: Bearer <AIR_SENDER_API_TOKEN>
```

If missing/invalid, server returns:

- `401 Unauthorized`
- body: `{ "error": "unauthorized" }`

---

## Health

### `GET /health`

Returns process liveness.

**Response 200**

```text
ok
```

---

## Dashboard and discovery

### `GET /v1/dashboard`

Aggregated KPIs:

- protocol_count
- pending_sessions (includes queued)
- active_sessions
- stopped_sessions
- trusted_device_count
- active_recordings

### `GET /v1/protocols`

Returns protocol descriptors and capability flags.

### `PATCH /v1/protocols/{id}` (auth required)

Enables/disables a protocol by ID (`airplay`, `cast`, `miracast`).

**Request**

```json
{
  "enabled": false
}
```

**Errors**

- `404` protocol not found

---

## Sessions

### `GET /v1/sessions`

Returns all sessions.

### `POST /v1/sessions` (auth required)

Creates a mock/simulated session.

**Request**

```json
{
  "protocol": "airplay",
  "device_name": "QA iPhone",
  "device_platform": "iOS",
  "priority": "teacher",
  "audio_mode": "audio-only"
}
```

Notes:

- `priority` options: `normal`, `teacher`, `admin-override`.
- `audio_mode` options: `full`, `audio-only`.
- `audio-only` is only valid for AirPlay/Cast.
- If current sessions hit policy limit, new sessions are created as `queued`.
- Disabled protocols cannot create sessions.

**Response 201**

A `SessionDescriptor` object.

### `POST /v1/sessions/{id}/accept` (auth required)

Transitions a session to `active` and may perform policy-based active session handoff when at session limit.

- `404` if session ID does not exist.

### `POST /v1/sessions/{id}/stop` (auth required)

Transitions a session to `stopped` and removes active recording for that session if present.

- `404` if session ID does not exist.

---

## Recording

### `GET /v1/recordings`

Lists active recordings.

### `POST /v1/recordings/start` (auth required)

Starts recording for an active session.

**Request**

```json
{
  "session_id": "<uuid>",
  "profile": {
    "destination_path": "./recordings",
    "quality_preset": "balanced",
    "codec": "h264",
    "container": "mp4"
  }
}
```

**Errors**

- `404` session not found
- `409` session not active

### `POST /v1/recordings/stop` (auth required)

Stops recording for a session.

**Request**

```json
{
  "session_id": "<uuid>"
}
```

**Errors**

- `404` if recording is not running for given session

---

## Trust management

### `GET /v1/trust`

Returns array of trusted device IDs.

### `POST /v1/trust/{deviceId}` (auth required)

Marks device ID as trusted.

### `DELETE /v1/trust/{deviceId}` (auth required)

Revokes trust.

- `404` if the device is not currently trusted.

---

## Policy

### `GET /v1/policy`

Returns receiver policy with acceptance, queue behavior, audio routing, and display options.

### `PATCH /v1/policy` (auth required)

Partial update.

**Request (example)**

```json
{
  "acceptance": "trusted-only",
  "max_sessions": 2,
  "queue_policy": "teacher-priority",
  "audio_output_device": "hdmi-1",
  "target_display": "display-2",
  "scaling_mode": "fill",
  "rotation_degrees": 90,
  "preserve_aspect_ratio": true
}
```

Validation:

- `max_sessions` must be between 1 and 4.
- `rotation_degrees` must be one of `0`, `90`, `180`, `270`.
- `audio_output_device` cannot be empty.

---

## Audit

### `GET /v1/audit`

Returns chronological list of audit events with fields:

- `id`
- `ts`
- `kind`
- `message`

Audit includes successful operations (policy, protocol, session, recording, trust) and denied security attempts.
