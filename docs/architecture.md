# Architecture

## 1) System overview

Air Sender Receiver is split into a **desktop control plane** and a **native receiver service**.

1. **Electron UI (`apps/desktop`)**
   - Presents operational KPIs and session/trust/policy controls.
   - Sends authenticated HTTP requests to `receiver-core`.
2. **Receiver core (`services/receiver-core`)**
   - Maintains in-memory receiver state.
   - Enforces authentication for mutating actions.
   - Emits audit events for security and lifecycle operations.
3. **Future privilege helper (planned)**
   - Reserved for OS-level integrations that may require elevated capabilities.

---

## 2) Runtime responsibilities

### Desktop app

- Bootstraps Electron `BrowserWindow` and serves static dashboard UI.
- Uses preload bridge APIs to isolate renderer from direct Node access.
- Proxies operator actions to receiver-core endpoint set.

### Receiver core

- Hosts HTTP API (`/health`, `/v1/...`) using Axum.
- Models:
  - Protocol descriptors and capability flags
  - Session lifecycle state
  - Trusted devices
  - Recording state
  - Receiver policy
  - Audit timeline
- Maintains data in process memory (no persistent DB yet).

---

## 3) Data model summary

### Protocol model

- Protocol kinds: `airplay`, `cast`, `miracast`
- Per protocol capabilities include max resolution, max FPS, audio and recording support.

### Session model

- Session status lifecycle:
  - `pending` → `active` → `stopped`
- Sessions include protocol and device metadata.

### Policy model

- Acceptance modes:
  - `auto`
  - `ask`
  - `trusted-only`
- `max_sessions` constrained to `1..=4` by API validation.

### Trust model

- Trusted device IDs represented as a set.
- Trust/revoke actions mutate the set and are audited.

### Audit model

- Append-only event list in memory.
- Captures denied security attempts and normal operations.

---

## 4) API boundary

- Read endpoints are unauthenticated.
- Mutating endpoints require `Authorization: Bearer <token>`.
- Errors return JSON payloads (`{ "error": "..." }`) with appropriate HTTP status.

See full endpoint contracts in `docs/api-reference.md`.

---

## 5) Media pipeline target (future integration)

Current implementation is control-plane stateful scaffolding.
Target pipeline contract remains:

`ingest -> demux/parse -> decode -> a/v sync -> compositor -> display -> recorder muxer`

This future path enables real adapter integrations while preserving current API/UI contracts.

---

## 6) Design constraints and implications

- **Local-first operation:** defaults favor localhost and desktop usage.
- **Token-based mutations:** simple but effective baseline for single-host deployments.
- **In-memory state:** fast prototyping; non-persistent after restart.
- **Protocol abstraction:** protocol descriptors and capability reporting allow UI behavior to remain adapter-aware.

---

## 7) Extension points

- Persistent backing store for sessions/trust/audit.
- Pluggable authentication provider(s).
- Real protocol adapter implementations.
- Background worker model for recording and transcoding jobs.
- Structured metrics and distributed tracing export.
