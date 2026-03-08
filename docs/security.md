# Security Notes

## Current controls

- Core binds to localhost by default.
- Mutating endpoints require bearer token.
- Audit logging records denied operations and key lifecycle actions.
- Policy supports `auto`, `ask`, `trusted-only` acceptance modes.

## Deployment recommendations

- Use a strong random `AIR_SENDER_API_TOKEN` in production.
- Keep API bound to localhost unless remote control is required.
- If remote access is required, place behind TLS + network controls (VPN/mTLS).
- Rotate token and restart service on suspected compromise.

## Non-goals (current implementation)

- No persistent encrypted secret store yet.
- No per-user authz model yet.
- No request signing/nonce replay protection yet.
