# Compatibility Matrix (Initial)

| Sender family | OS / version band | Protocols validated | Smoke status |
|---|---|---|---|
| iPhone / iPad | iOS 16-17 | AirPlay | Pass |
| macOS laptop | macOS 13-14 | AirPlay, Cast (Chrome) | Pass |
| Android phone | Android 13-14 | Cast, Miracast | Pass |
| Chromebook | ChromeOS 118+ | Cast | Pass |
| Windows laptop | Windows 11 | Miracast, Cast (Chrome) | Pass |

## Validation method

- Nightly smoke run executes `scripts/test-all-features-local.sh`.
- Interactive desktop spot-check validates operator controls, protocol toggles, and session lifecycle visibility.
- Matrix will expand in Month 3 with per-device vendor breakdown and regression tags.
