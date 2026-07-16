# iOS Platform Files

Place iOS-specific platform files here:

- `Info.plist` — app metadata, permissions (NSLocationWhenInUseUsageDescription, etc.)
- `*.entitlements` — capabilities (push notifications require `aps-environment`)
- `Assets.xcassets/` — app icons, launch screen

## Build

```bash
# Install iOS target
rustup target add aarch64-apple-ios

# Build with Dioxus CLI
dx build --platform ios --release
```

## Critical Alerts (SOS roadmap)

iOS Critical Alerts (bypass DND) require explicit Apple entitlement approval.
Apply via: https://developer.apple.com/contact/request/notifications-critical-alerts-entitlement/
