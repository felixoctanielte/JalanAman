# Android Platform Files

Place Android-specific platform files here:

- `AndroidManifest.xml` — permissions (ACCESS_FINE_LOCATION, VIBRATE, POST_NOTIFICATIONS)
- `res/` — drawables, icons, strings
- `build.gradle` — Android build config

## Build

```bash
# Install Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# Build with Dioxus CLI
dx build --platform android --release
```

## Push Notifications (Android)

Android push uses FCM. The web VAPID approach doesn't apply natively.
For native Android, integrate `firebase-android-sdk` or use `fcm-push` in Rust.
