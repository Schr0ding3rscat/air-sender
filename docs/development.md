# Development Guide

## Repo layout

- `services/receiver-core`: Rust API service
- `apps/desktop`: Electron control plane
- `docs`: architecture, API, debugging, runbooks

## Local development

```bash
# terminal 1
cd services/receiver-core
AIR_SENDER_API_TOKEN=dev-token cargo run

# terminal 2
cd apps/desktop
npm install
AIR_SENDER_API_TOKEN=dev-token npm run dev
```

## Testing

```bash
cd services/receiver-core
cargo fmt
cargo test
```

```bash
node --check apps/desktop/src/main.js
node --check apps/desktop/src/preload.js
```

## Contribution expectations

- Keep API additions versioned under `/v1`.
- Add or update tests for mutating endpoint logic.
- Update `docs/api-reference.md` for new routes or payload changes.
