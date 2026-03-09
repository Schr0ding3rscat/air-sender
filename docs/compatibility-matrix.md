# Compatibility Matrix (Expanded Month 3)

| Sender family | OS / version band | Protocols validated | Discovery | Pairing | Routing/rendering | Reconnect resilience | Status |
|---|---|---|---|---|---|---|---|
| iPhone / iPad | iOS 16-17 | AirPlay | Pass | Pass | Pass (1080p60 baseline) | Pass | Pass |
| macOS laptop | macOS 13-14 | AirPlay, Cast (Chrome) | Pass | Pass | Pass (multi-display fit/fill) | Pass | Pass |
| Android phone | Android 13-15 | Cast, Miracast | Pass | N/A | Pass (1080p60 baseline) | Pass | Pass |
| Chromebook | ChromeOS 118+ | Cast | Pass | N/A | Pass | Pass | Pass |
| Windows laptop | Windows 10/11 | Miracast, Cast (Chrome) | Pass | N/A | Pass (best-effort 4K path) | Pass | Pass |

## Validation method

- Nightly smoke and regression run via `scripts/test-all-features-local.sh`.
- Soak reliability run via `scripts/soak-connect-disconnect.sh`.
- API-level regression includes discovery, pairing, routing, rendering-adjacent policy paths, reconnect, and diagnostics export.
