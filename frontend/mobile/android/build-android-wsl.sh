#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MOBILE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
SDK_ROOT="${ANDROID_HOME:-$HOME/Android/Sdk}"
NDK_VERSION="${ANDROID_NDK_VERSION:-28.2.13676358}"
PROJECT_CACHE="${JALANAMAN_CACHE_DIR:-$HOME/.cache/jalanaman}"

export ANDROID_HOME="$SDK_ROOT"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/$NDK_VERSION}"
export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PROJECT_CACHE/android-target}"
export GRADLE_USER_HOME="${GRADLE_USER_HOME:-$PROJECT_CACHE/gradle}"
export JALANAMAN_API_BASE_URL="${JALANAMAN_API_BASE_URL:-http://127.0.0.1:8080/api}"

if [ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
  echo "ANDROID_NDK_HOME is invalid: $ANDROID_NDK_HOME" >&2
  echo "Run: bash android/repair-android-env-wsl.sh" >&2
  exit 1
fi

cd "$MOBILE_DIR"
dx build --platform android "$@"
bash "$SCRIPT_DIR/patch-generated-android.sh"

ANDROID_APP_DIR="$MOBILE_DIR/target/dx/jalanaman_mobile/debug/android/app"
if [ -x "$ANDROID_APP_DIR/gradlew" ]; then
  (cd "$ANDROID_APP_DIR" && ./gradlew assembleDebug)
fi
