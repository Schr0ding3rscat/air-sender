# 3-Month Delivery Plan

This plan accelerates delivery from the current control-plane baseline to an AirServer-competitive receiver focused on classroom, meeting room, and lab deployments.

## Month 1 (Weeks 1-4): Core parity foundation

- Finalize control-plane API and orchestration runtime for protocol adapters.
- Ship AirPlay receiver v1 with:
  - Discovery, pairing, and trust controls.
  - Mirroring + audio streaming baseline.
  - On-screen PIN and optional persistent trust.
- Ship Google Cast receiver v1 with:
  - Discovery and connect/disconnect lifecycle.
  - Basic tab/app casting paths and receiver status reporting.
- Add operator UX baseline for:
  - Device name, PIN policy, and network visibility.
  - Session start/stop, active sender visibility, and event log.
- Build first compatibility matrix and nightly smoke tests for target OS/device combinations.

**Exit criteria**

- AirPlay and Cast sessions can be started/stopped reliably from representative devices.
- Admin can enforce pairing/PIN policy and observe session state in real time.

---

## Month 2 (Weeks 5-8): Feature-depth to match AirServer expectations

- Add Miracast receiver v1 with connect controls and stability baseline.
- Multi-protocol session manager:
  - Queue/priority policy (first-in, teacher-priority, admin-override).
  - Graceful handoff between active senders.
- Audio capabilities:
  - System-audio routing selection (local output device picker).
  - Audio-only mode for AirPlay/Cast.
- Display capabilities:
  - Multi-display target selection.
  - Fit/fill/actual-size scaling controls.
  - Rotation and aspect-ratio-safe rendering.
- Recording and capture:
  - Single-session recording with timestamped metadata.
  - Quick export flow for common formats.
- Security/management:
  - Per-protocol enable/disable toggles.
  - Session approval mode (auto-accept vs moderator-approve).
  - Signed configuration profiles for managed rollout.

**Exit criteria**

- AirPlay, Cast, and Miracast available behind one policy and session model.
- Core classroom/conference workflows (PIN + approval + handoff + audio routing) validated end to end.

---

## Month 3 (Weeks 9-12): Hardening and release readiness

- Reliability and performance hardening:
  - Reconnect resilience for network jitter and transient drops.
  - Latency/throughput tuning for 1080p60 baseline and 4K best-effort path.
  - Soak tests under repeated connect/disconnect cycles.
- Compatibility and quality:
  - Expanded matrix across major iOS/macOS, Android/ChromeOS, and Windows senders.
  - Regression suite for discovery, pairing, routing, and rendering.
- Enterprise-readiness:
  - Installer/signing pipeline and update rollback flow.
  - Audit/event export and structured diagnostics bundle.
  - Admin CLI parity for key UI actions.
- Release operations:
  - Security review and threat-model sign-off.
  - Deployment runbooks and support playbooks.
  - GA readiness review with objective pass/fail gates.

**Exit criteria**

- Release candidate demonstrates stable multi-protocol operation in target environments.
- Operational, security, and support checklists complete for production launch.
