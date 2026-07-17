#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MOBILE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
SDK_ROOT="${ANDROID_HOME:-$HOME/Android/Sdk}"
NDK_VERSION="${ANDROID_NDK_VERSION:-28.2.13676358}"
PROJECT_CACHE="${JALANAMAN_CACHE_DIR:-$HOME/.cache/jalanaman}"
ADB_WRAPPER_DIR="$PROJECT_CACHE/android-tools"

export ANDROID_HOME="$SDK_ROOT"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/$NDK_VERSION}"
export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PROJECT_CACHE/android-target}"
export GRADLE_USER_HOME="${GRADLE_USER_HOME:-$PROJECT_CACHE/gradle}"

detect_windows_user_from_path() {
  local path_after_users="${MOBILE_DIR#/mnt/c/Users/}"

  if [ "$path_after_users" != "$MOBILE_DIR" ]; then
    printf '%s' "${path_after_users%%/*}"
  fi
}

detect_windows_user_from_cmd() {
  if ! command -v cmd.exe >/dev/null 2>&1; then
    return 0
  fi

  cmd.exe /c "echo %USERNAME%" 2>/dev/null | tr -d '\r' | tail -n 1 || true
}

detect_windows_user() {
  local windows_user

  windows_user="$(detect_windows_user_from_path)"
  if [ -n "$windows_user" ]; then
    printf '%s' "$windows_user"
    return
  fi

  windows_user="$(detect_windows_user_from_cmd)"
  if [ -n "$windows_user" ] && [ "$windows_user" != "%USERNAME%" ]; then
    printf '%s' "$windows_user"
  fi
}

use_windows_adb_if_available() {
  local windows_user
  local windows_adb

  windows_user="${WINDOWS_USER:-$(detect_windows_user)}"
  if [ -z "$windows_user" ]; then
    return
  fi

  windows_adb="/mnt/c/Users/$windows_user/AppData/Local/Android/Sdk/platform-tools/adb.exe"
  if [ ! -x "$windows_adb" ]; then
    return
  fi

  if "$windows_adb" devices | tr -d '\r' | grep -qE '^[^[:space:]]+[[:space:]]+device$'; then
    mkdir -p "$ADB_WRAPPER_DIR"
    {
      printf '%s\n' '#!/usr/bin/env bash'
      printf '%s\n' 'set -euo pipefail'
      printf 'WINDOWS_ADB=%q\n' "$windows_adb"
      printf '%s\n' 'case "${1:-}" in'
      printf '%s\n' '  install|install-multiple|install-multi-package|push|pull) ;;'
      printf '%s\n' '  *) exec "$WINDOWS_ADB" "$@" ;;'
      printf '%s\n' 'esac'
      printf '%s\n' 'converted=()'
      printf '%s\n' 'for arg in "$@"; do'
      printf '%s\n' '  if [ -e "$arg" ] && command -v wslpath >/dev/null 2>&1; then'
      printf '%s\n' '    converted+=("$(wslpath -w "$arg")")'
      printf '%s\n' '  else'
      printf '%s\n' '    converted+=("$arg")'
      printf '%s\n' '  fi'
      printf '%s\n' 'done'
      printf '%s\n' 'exec "$WINDOWS_ADB" "${converted[@]}"'
    } > "$ADB_WRAPPER_DIR/adb"
    chmod +x "$ADB_WRAPPER_DIR/adb"
    export PATH="$ADB_WRAPPER_DIR:$PATH"
    echo "Using Windows adb for Android device/emulator: $windows_adb"
  fi
}

grant_dev_permissions() {
  local package_name="com.jalanaman.JalanamanMobile"
  local permission
  local op

  for permission in \
    android.permission.ACCESS_FINE_LOCATION \
    android.permission.ACCESS_COARSE_LOCATION \
    android.permission.POST_NOTIFICATIONS
  do
    adb shell pm clear-permission-flags "$package_name" "$permission" user-set user-fixed >/dev/null 2>&1 || true
    adb shell pm grant "$package_name" "$permission" >/dev/null 2>&1 || true
  done

  for op in FINE_LOCATION COARSE_LOCATION POST_NOTIFICATION; do
    adb shell appops set "$package_name" "$op" allow >/dev/null 2>&1 || true
  done
}

if [ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
  echo "ANDROID_NDK_HOME is invalid: $ANDROID_NDK_HOME" >&2
  echo "Run: bash android/repair-android-env-wsl.sh" >&2
  exit 1
fi

use_windows_adb_if_available

if command -v adb >/dev/null 2>&1; then
  adb reverse tcp:8080 tcp:8080 >/dev/null 2>&1 || true
fi

export JALANAMAN_API_BASE_URL="${JALANAMAN_API_BASE_URL:-http://127.0.0.1:8080/api}"

cd "$MOBILE_DIR"
bash "$SCRIPT_DIR/build-android-wsl.sh" "$@"

APK_PATH="$MOBILE_DIR/target/dx/jalanaman_mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk"
if ! adb get-state 2>/dev/null | tr -d '\r' | grep -qx "device"; then
  echo "HP Android belum terdeteksi oleh adb. Aktifkan USB debugging lalu jalankan ulang." >&2
  exit 1
fi

adb install --no-streaming -r "$APK_PATH"
grant_dev_permissions
adb shell am force-stop com.jalanaman.JalanamanMobile
adb shell am start -n com.jalanaman.JalanamanMobile/dev.dioxus.main.MainActivity >/dev/null
echo "JalanAman sudah dipasang dan dibuka di HP."
