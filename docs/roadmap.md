# 3-Month Delivery Plan

This plan accelerates delivery from the current control-plane baseline to an AirServer-competitive receiver focused on classroom, meeting room, and lab deployments.

## Month 1 (Weeks 1-4): Core parity foundation ✅ Complete

Delivered in this sprint:

- Control-plane API and orchestration runtime for protocol adapters (AirPlay, Cast, Miracast descriptors under one control model).
- AirPlay receiver v1 parity simulation:
  - Discovery via protocol descriptors.
  - Pairing + trust controls (`/v1/trust/*`, `/v1/pairing/pin`).
  - Mirroring/audio baseline and audio-only policy path.
  - On-screen PIN generation and persistent trust list support.
- Google Cast receiver v1 parity simulation:
  - Discovery and connect/disconnect lifecycle via session create/accept/stop.
  - Basic tab/app casting behavior represented through session state transitions.
  - Receiver status reporting in dashboard + sessions listing.
- Operator UX baseline delivered:
  - Device name, PIN policy, network visibility (`/v1/operator/settings`).
  - Session start/stop controls and active sender visibility in desktop panel.
  - Event log backed by `/v1/audit`.
- Initial compatibility matrix + nightly smoke tests:
  - Compatibility matrix documented in `docs/compatibility-matrix.md`.
  - Automated smoke harness in `scripts/test-all-features-local.sh` for nightly CI/local runs.

**Exit criteria status**

- ✅ AirPlay and Cast sessions can be started/stopped reliably from representative simulated devices.
- ✅ Admin can enforce pairing/PIN policy and observe session state in real time.

---

## Month 2 (Weeks 5-8): Feature-depth to match AirServer expectations ✅ Complete

Delivered in this sprint:

- Miracast receiver v1 parity simulation with connect controls and stability baseline.
- Multi-protocol session manager:
  - Queue/priority policy (`first-in`, `teacher-priority`, `admin-override`).
  - Graceful handoff between active senders when limits are reached.
- Audio capabilities:
  - System-audio routing selection (`audio_output_device`).
  - Audio-only mode for AirPlay/Cast session creation.
- Display capabilities:
  - Multi-display target selection (`target_display`).
  - Fit/fill/actual-size scaling controls.
  - Rotation + aspect-ratio-safe rendering flags.
- Recording and capture:
  - Single-session recording with timestamped metadata.
  - Quick export profile fields (codec/container/preset/path) via recording profile.
- Security/management:
  - Per-protocol enable/disable toggles.
  - Session approval mode (`auto`, `ask`, `trusted-only`).
  - Signed configuration profiles (`/v1/config-profiles/sign`, `/v1/config-profiles/verify`).

**Exit criteria status**

- ✅ AirPlay, Cast, and Miracast available behind one policy and session model.
- ✅ Core classroom/conference workflows (PIN + approval + handoff + audio routing) validated end-to-end through API + desktop flow and script harness.

---

## Month 3 (Weeks 9-12): Hardening and release readiness ✅ Complete

Delivered in this sprint:

- Reliability and performance hardening:
  - Reconnect resilience API with bounded retry behavior for transient drops (`POST /v1/sessions/{id}/reconnect`).
  - Performance tuning controls in policy (`target_latency_ms`, `max_bitrate_mbps`, `baseline_profile`, `allow_4k_best_effort`) and a synthesized throughput/latency report (`GET /v1/performance/report`).
  - Soak coverage for repeated connect/disconnect cycles via `scripts/soak-connect-disconnect.sh`.
- Compatibility and quality:
  - Expanded sender compatibility matrix across major iOS/macOS, Android/ChromeOS, and Windows classes in `docs/compatibility-matrix.md`.
  - Regression suite expanded to cover reconnect behavior, performance policy validation, diagnostics, and export endpoints.
- Enterprise-readiness:
  - Installer/signing + rollback flow documented and scripted (`scripts/release-installer-sign-and-rollback.sh`).
  - Audit/event export (`GET /v1/audit/export`) and structured diagnostics bundle (`GET /v1/diagnostics/bundle`).
  - Admin CLI parity script for key UI actions (`scripts/admin-cli.sh`).
- Release operations:
  - Security review + threat model sign-off doc (`docs/security-review-month3.md`).
  - Deployment runbook + support playbook updates in `docs/deployment.md` and `docs/operations.md`.
  - GA readiness gates and objective pass/fail rubric in `docs/ga-readiness.md`.

**Exit criteria**

- Release candidate demonstrates stable multi-protocol operation in target environments.
- Operational, security, and support checklists complete for production launch.
