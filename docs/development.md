# Development Guide

## 1) Prerequisites

- Rust stable toolchain
- Node.js 18+
- npm
- Git

Optional:

- Docker + Docker Compose for containerized core runs

---

## 2) Local setup

### Start receiver-core

```bash
cd services/receiver-core
AIR_SENDER_API_TOKEN=dev-token cargo run
```

### Start desktop app

```bash
cd apps/desktop
npm install
AIR_SENDER_API_TOKEN=dev-token npm run dev
```

Use the same token in both processes.

---

## 3) Useful development commands

### Rust service

```bash
cd services/receiver-core
cargo fmt
cargo test
cargo run
```

### Desktop app

```bash
cd apps/desktop
npm install
npm run dev
```

---

## 4) Manual verification flow

1. Launch core and desktop.
2. In desktop UI, create a session.
3. Accept session and verify status becomes `active`.
4. Start/stop recording for active session.
5. Trust and revoke a device ID.
6. Update policy and confirm values persist in API response.
7. Inspect audit pane for recorded actions.

---

## 5) Curl-based API sanity checks

```bash
TOKEN=dev-token
BASE=http://127.0.0.1:9760

curl -s "$BASE/v1/dashboard"
curl -s -X POST "$BASE/v1/trust/demo" -H "Authorization: Bearer $TOKEN"
curl -s -X PATCH "$BASE/v1/policy" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"acceptance":"ask","max_sessions":3}'
```

---

## 6) Codebase orientation

- `services/receiver-core/src/lib.rs`
  - API routes, state model, auth checks, handlers, tests
- `services/receiver-core/src/main.rs`
  - process entrypoint and env configuration
- `apps/desktop/src/main.js`
  - Electron process and HTTP bridge implementation
- `apps/desktop/static/index.html`
  - dashboard layout and client interaction logic

---


Additional validation checks worth testing manually:

- `PATCH /v1/policy` rejects blank `audio_output_device` or `target_display` values (`400`).
- `POST /v1/sessions/{id}/accept` returns `404` for unknown IDs without modifying active sessions.
- `POST /v1/sessions/{id}/accept` rejects stopped sessions (`409`).

## 7) Common pitfalls

- Token mismatch between desktop and core causes `401` on actions.
- Non-running core causes initial desktop fetch failures.
- Recording start fails unless session status is `active`.
- Policy update fails if `max_sessions` is outside `1..4`.
