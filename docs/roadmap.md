# 12-Month Product Roadmap (Post-GA)

The initial 3-month delivery plan is complete. This roadmap focuses on the next 12 months of **product features and customer-facing improvements** for classrooms, meeting rooms, and labs.

## Completed baseline (now standard product capabilities)

These items are complete and no longer part of upcoming roadmap delivery:

- Multi-protocol control plane for AirPlay, Cast, and Miracast session lifecycle.
- Desktop operator dashboard for sessions, trust, policies, and audit visibility.
- Session policy controls (approval modes, queue/priority behavior, reconnect support).
- Recording/export, diagnostics bundle, and audit export foundations.
- Deployment/operations baseline and GA readiness pass.

---

## Product goals for the next 12 months

1. Deliver real media streaming quality that matches customer expectations in live classrooms and meetings.
2. Improve day-to-day operator experience so common tasks are faster and require fewer manual steps.
3. Expand enterprise-grade management features for larger deployments.
4. Increase compatibility confidence across sender devices and OS versions.

---

## Quarter 1 (Months 1-3): Real media MVP + operator visibility

### Feature themes

- Real media playback for first protocol target.
- Better session health visibility in product UI.
- More reliable behavior after restart.

### Planned feature delivery

1. **Real AirPlay media path (MVP)**
   - Move AirPlay from simulated session flow to real ingest/decode/render playback.
   - Keep existing control APIs compatible so current integrations continue to work.
   - Add automatic fallback behavior when media path fails unexpectedly.

2. **Live session quality indicators in dashboard**
   - Add visible stream health badges (latency health, reconnect state, dropped frames).
   - Show first-frame timing and current quality mode (standard / best effort).
   - Surface actionable error messages instead of generic failures.

3. **Persistent receiver settings and trust state**
   - Preserve trusted devices, core policy settings, and key operator preferences across restart.
   - Add safe startup recovery when persisted state is partially invalid.

4. **Improved diagnostics for frontline support**
   - Enrich diagnostics bundles with media startup and failure context.
   - Add one-click collection path in desktop for failed-session incidents.

### Customer impact by end of Q1

- Customers can run real AirPlay sessions instead of simulation-only behavior.
- Operators can quickly identify why a session is unhealthy.
- Common restart events no longer reset trust/policy setup.

---

## Quarter 2 (Months 4-6): Protocol depth + smoother classroom workflows

### Feature themes

- Expand real media support to additional protocols.
- Improve queue/handoff behavior for shared-room scenarios.
- Reduce friction in recording and moderation workflows.

### Planned feature delivery

1. **Real Cast media support**
   - Add real Cast playback path with parity to AirPlay baseline controls.
   - Improve Cast reconnect behavior for transient network disruptions.

2. **Miracast stability improvements**
   - Improve connect reliability and reduce failed handshakes.
   - Add clearer session-state transitions for Miracast troubleshooting.

3. **Queue and handoff UX improvements**
   - Add explicit queue reason labels (why a presenter is waiting).
   - Add scheduled handoff windows for planned presenter transitions.
   - Add clearer admin override actions in dashboard.

4. **Recording management v2**
   - Add retention duration and storage quota controls.
   - Add improved export reliability with retry + integrity validation.
   - Show recording status and failure reasons directly in UI.

5. **Approval workflow enhancements**
   - Show richer pending-request context (device trust, protocol, policy reason).
   - Add faster operator actions for approve/reject with audit trace continuity.

### Customer impact by end of Q2

- Two primary protocols (AirPlay + Cast) support real media playback.
- Presenter transitions in shared rooms become more predictable.
- Recording/export becomes reliable for routine classroom and meeting use.

---

## Quarter 3 (Months 7-9): Enterprise management and access control

### Feature themes

- Make multi-device deployments easier to manage.
- Add enterprise-friendly authentication and permissions.
- Improve observability for operations teams.

### Planned feature delivery

1. **Policy bundles and deployment templates**
   - Provide reusable templates for classroom, conference-room, and lab profiles.
   - Add staged rollout controls for policy changes across receiver groups.
   - Add rollback to prior known-good policy profile.

2. **Enterprise sign-in options**
   - Add OIDC-based operator authentication.
   - Preserve local token mode for simple/offline environments.
   - Add login/provider status visibility in admin settings.

3. **Role-based permissions**
   - Introduce viewer/operator/admin roles in desktop and API.
   - Restrict sensitive actions (policy mutation, trust override, stop-all sessions) to authorized roles.
   - Add role-aware UI affordances (disabled controls + permission guidance).

4. **Operational observability features**
   - Export structured metrics for monitoring platforms.
   - Add request/session correlation IDs for faster incident tracing.
   - Add dashboard panels for incident trend visibility.

5. **Backup and restore product workflows**
   - Add operator-facing backup/restore actions for key settings and trust state.
   - Validate restore results and present recovery summary in UI.

### Customer impact by end of Q3

- IT teams can manage many receivers using shared policy templates.
- Organizations can apply least-privilege access for daily operations.
- Incident investigations become significantly faster.

---

## Quarter 4 (Months 10-12): Scale quality, compliance features, and v2 readiness

### Feature themes

- Improve performance at higher usage levels.
- Add enterprise-requested governance/compliance capabilities.
- Prepare product for broad v2 rollout.

### Planned feature delivery

1. **Performance and concurrency improvements**
   - Optimize render/decode path for heavier room usage patterns.
   - Improve behavior for high-motion content and mixed audio/video loads.
   - Add quality guardrails for standard and best-effort high-resolution modes.

2. **Compatibility expansion program**
   - Expand verified sender coverage across OS/device matrix.
   - Add compatibility notes visible to operators when known limitations apply.
   - Publish compatibility updates with each release channel cycle.

3. **Compliance and audit enhancements**
   - Add configurable audit retention policies.
   - Add signed/tamper-evident export options for audit artifacts.
   - Add simplified compliance evidence export package for enterprise reviews.

4. **Release channel productization**
   - Provide stable/candidate update channels for controlled rollout.
   - Add staged deployment controls with health-based rollback triggers.
   - Improve release notes quality with explicit feature and risk callouts.

5. **v2 launch-readiness package**
   - Finalize support playbooks and operator runbooks for new capabilities.
   - Refresh onboarding guidance for enterprise deployment patterns.
   - Complete broad-release validation gates.

### Customer impact by end of Q4

- Product performs consistently in larger and more demanding deployments.
- Compliance-sensitive customers can satisfy audit/governance needs more easily.
- v2 is ready for broad adoption with safer rollout controls.

---

## Continuous roadmap tracks (all four quarters)

- **User experience polish:** ongoing dashboard clarity improvements, fewer clicks for common tasks, and better action feedback.
- **Reliability improvements:** regular hardening of reconnect, queue, and long-running session behavior.
- **Documentation improvements:** keep admin/operator docs aligned to every major feature release.
- **Compatibility maintenance:** continuously update tested sender matrix and known-issue guidance.

---

## Success measures (product-facing)

By the end of the 12-month roadmap, success means:

- Real media support is default for core protocol workflows.
- Operators can diagnose and resolve common session issues directly from the dashboard.
- Enterprise customers can deploy, secure, and govern receivers with minimal custom process.
- Compatibility and rollout confidence are high enough for broad production expansion.
