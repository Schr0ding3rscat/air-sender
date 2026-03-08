# 12-Month Delivery Plan

This plan outlines a staged path from the current control-plane baseline to a production-grade multi-protocol receiver.

## M1-M2: Foundation and platform baseline

- Monorepo bootstrapping and core API scaffolding.
- Session lifecycle model (`pending/active/stopped`).
- Policy/trust/recording state primitives and audit framework.
- Initial desktop operator dashboard.
- Compliance and licensing investigation for protocol implementations.

**Exit criteria**

- Stable local control API with authenticated mutations.
- Basic operational UI for session and policy controls.

---

## M3-M5: First protocol integration

- AirPlay adapter MVP with discovery and connection flow.
- Audio/video sync baseline for primary target platforms.
- Pairing UX refinement and trust semantics hardening.
- Single-session recording stabilization.

**Exit criteria**

- Demonstrated end-to-end mirror session on representative devices.
- Controlled recording flow for active sessions.

---

## M6-M8: Expanded protocol coverage

- Cast adapter MVP.
- Miracast adapter MVP.
- Dynamic protocol routing in runtime orchestration.
- Multi-session handling up to policy limits.
- Initial admin CLI for scripted control.

**Exit criteria**

- Core protocol matrix available in non-GA quality.
- Policy enforcement under mixed-protocol workloads.

---

## M9-M10: Reliability and hardening

- Compatibility matrix expansion (OS + device variants).
- Reconnect and impairment resilience scenarios.
- Installer/signing pipeline hardening.
- Update-channel and rollback procedures.

**Exit criteria**

- Defined SLO targets with load/soak validation evidence.
- Reproducible installer and update artifacts.

---

## M11-M12: Release readiness

- Security validation and threat-model review.
- Performance acceptance test sign-off.
- Documentation and support runbooks completion.
- DRM feasibility and legal risk summary.
- GA gate review.

**Exit criteria**

- Release candidate with production checklists complete.
- Executive go/no-go decision package assembled.
