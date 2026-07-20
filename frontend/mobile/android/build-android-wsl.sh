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

DX_PROFILE="debug"
for arg in "$@"; do
  if [ "$arg" = "--release" ] || [ "$arg" = "-r" ]; then
    DX_PROFILE="release"
  fi
done

ANDROID_APP_DIR="$MOBILE_DIR/target/dx/jalanaman_mobile/$DX_PROFILE/android/app"
APK_PATH="$ANDROID_APP_DIR/app/build/outputs/apk/debug/app-debug.apk"

if [ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
  echo "ANDROID_NDK_HOME is invalid: $ANDROID_NDK_HOME" >&2
  echo "Run: bash android/repair-android-env-wsl.sh" >&2
  exit 1
fi

cd "$MOBILE_DIR"
BUILD_LOG="$(mktemp)"
if ! dx build --platform android "$@" 2>&1 | tee "$BUILD_LOG"; then
  if grep -q "Build completed successfully" "$BUILD_LOG" && [ -x "$ANDROID_APP_DIR/gradlew" ]; then
    echo "dx completed bundling, but its internal Gradle assemble failed. Retrying with the patched Android project."
  else
    rm -f "$BUILD_LOG"
    exit 1
  fi
fi
rm -f "$BUILD_LOG"

JALANAMAN_DX_PROFILE="$DX_PROFILE" bash "$SCRIPT_DIR/patch-generated-android.sh"

if [ -x "$ANDROID_APP_DIR/gradlew" ]; then
  (cd "$ANDROID_APP_DIR" && ./gradlew assembleDebug)
fi

if [ ! -f "$APK_PATH" ]; then
  echo "APK debug tidak ditemukan setelah build: $APK_PATH" >&2
  exit 1
fi

AAPT_PATH="$(find "$ANDROID_HOME/build-tools" -maxdepth 2 -type f -name aapt 2>/dev/null | sort -V | tail -n 1)"
if [ -n "$AAPT_PATH" ] && ! "$AAPT_PATH" dump permissions "$APK_PATH" | grep -q "android.permission.ACCESS_FINE_LOCATION"; then
  echo "APK dibuat tanpa izin lokasi. Patch Android belum ikut terpaketkan." >&2
  exit 1
fi

if command -v unzip >/dev/null 2>&1 \
  && ! unzip -p "$APK_PATH" 'classes*.dex' 2>/dev/null | grep -a -q "JalanAmanNative"; then
  echo "APK dibuat tanpa bridge lokasi native. Menjalankan clean rebuild Android." >&2
  (cd "$ANDROID_APP_DIR" && ./gradlew clean assembleDebug)
fi

echo "APK siap dipasang: $APK_PATH"
