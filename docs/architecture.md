# Architecture

## Runtime split

1. **UI Process (Electron)**
   - Presents protocol/session state
   - Sends authenticated control requests to receiver core
2. **Receiver Core (Rust service)**
   - Owns policy, pairing, session lifecycle, recording orchestration, audit
   - Hosts versioned local IPC API
3. **Privilege Helper (future native daemon/service)**
   - Reserved for OS-level networking/display capabilities requiring elevated permissions

## Core interfaces

- `ProtocolAdapter`
- `SessionController`
- `CompositorEngine`
- `Recorder`
- `PairingStore`
- `PolicyEngine`

## Media pipeline contract (v1 target)

`ingest -> demux/parse -> decode -> a/v sync -> compositor -> display -> recorder muxer`

## Security model

- Bind default API listener to localhost
- Require `Authorization: Bearer <AIR_SENDER_API_TOKEN>` for state-mutating endpoints
- Deny non-local mutations and audit denied attempts
- Pairing trust list and acceptance policy (`auto`, `ask`, `trusted-only`)

## Planned protocol adapters

- `AirPlayAdapter`
- `CastAdapter`
- `MiracastAdapter`

Adapters report explicit capability flags so UI only renders supported actions.
