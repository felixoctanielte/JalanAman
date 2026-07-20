#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MOBILE_DIR="$ROOT_DIR/frontend/mobile"
ANDROID_DIR="$MOBILE_DIR/android"
PROJECT_CACHE="${JALANAMAN_CACHE_DIR:-$HOME/.cache/jalanaman}"
ADB_WRAPPER_DIR="$PROJECT_CACHE/android-tools"
BACKEND_PORT="${PORT:-8080}"
BACKEND_LOG="${JALANAMAN_BACKEND_LOG:-/tmp/jalanaman-backend.log}"
PACKAGE_NAME="${JALANAMAN_ANDROID_PACKAGE:-com.jalanaman.JalanamanMobile}"
ACTIVITY_NAME="${JALANAMAN_ANDROID_ACTIVITY:-dev.dioxus.main.MainActivity}"
POLL_INTERVAL="${JALANAMAN_WATCH_POLL_INTERVAL:-2}"
BACKEND_PID=""

WATCH_PATHS=(
  "$MOBILE_DIR/src"
  "$MOBILE_DIR/assets"
  "$MOBILE_DIR/Cargo.toml"
  "$MOBILE_DIR/Dioxus.toml"
  "$ANDROID_DIR/build-android-wsl.sh"
  "$ANDROID_DIR/patch-generated-android.sh"
  "$ROOT_DIR/frontend/shared/src"
  "$ROOT_DIR/frontend/shared/Cargo.toml"
)

log() {
  printf '[%s] %s\n' "$1" "$2"
}

warn() {
  printf '[warn] %s\n' "$1" >&2
}

die() {
  printf '[error] %s\n' "$1" >&2
  exit 1
}

cleanup() {
  if [ -n "$BACKEND_PID" ] && kill -0 "$BACKEND_PID" >/dev/null 2>&1; then
    log backend "Stopping backend PID $BACKEND_PID"
    kill "$BACKEND_PID" >/dev/null 2>&1 || true
    wait "$BACKEND_PID" >/dev/null 2>&1 || true
  fi
}

trap cleanup EXIT
trap 'exit 130' INT TERM

setup_android_env() {
  local sdk_root="${ANDROID_HOME:-$HOME/Android/Sdk}"
  local ndk_version="${ANDROID_NDK_VERSION:-28.2.13676358}"

  export ANDROID_HOME="$sdk_root"
  export ANDROID_SDK_ROOT="$ANDROID_HOME"
  export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/$ndk_version}"
  export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
  export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PROJECT_CACHE/android-target}"
  export GRADLE_USER_HOME="${GRADLE_USER_HOME:-$PROJECT_CACHE/gradle}"
  export JALANAMAN_API_BASE_URL="${JALANAMAN_API_BASE_URL:-http://127.0.0.1:$BACKEND_PORT/api}"
}

assert_android_env() {
  if [ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
    die "ANDROID_NDK_HOME invalid: $ANDROID_NDK_HOME. Run: bash frontend/mobile/android/repair-android-env-wsl.sh"
  fi

  command -v dx >/dev/null 2>&1 || die "dx tidak ditemukan. Install dioxus-cli 0.6.3 dulu."
  command -v adb >/dev/null 2>&1 || die "adb tidak ditemukan di PATH Android SDK."
}

detect_windows_user_from_path() {
  local path_after_users="${ROOT_DIR#/mnt/c/Users/}"

  if [ "$path_after_users" != "$ROOT_DIR" ]; then
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

adb_has_device() {
  adb devices 2>/dev/null \
    | tr -d '\r' \
    | awk 'NR > 1 && $2 == "device" { found = 1 } END { exit found ? 0 : 1 }'
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

  if ! "$windows_adb" devices 2>/dev/null | tr -d '\r' | grep -qE '^[^[:space:]]+[[:space:]]+device$'; then
    return
  fi

  mkdir -p "$ADB_WRAPPER_DIR"
  {
    printf '%s\n' '#!/usr/bin/env bash'
    printf '%s\n' 'set -euo pipefail'
    printf 'WINDOWS_ADB=%q\n' "$windows_adb"
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
  log android "Using Windows adb: $windows_adb"
}

select_android_device() {
  local serials
  local first_serial
  local count

  if [ -n "${ANDROID_SERIAL:-}" ]; then
    return
  fi

  serials="$(adb devices 2>/dev/null | tr -d '\r' | awk 'NR > 1 && $2 == "device" { print $1 }')"
  first_serial="$(printf '%s\n' "$serials" | sed -n '1p')"
  count="$(printf '%s\n' "$serials" | sed '/^$/d' | wc -l | tr -d ' ')"

  if [ -n "$first_serial" ]; then
    export ANDROID_SERIAL="$first_serial"
    if [ "$count" != "1" ]; then
      log android "Multiple devices detected; using $ANDROID_SERIAL"
    fi
  fi
}

ensure_android_device() {
  if ! adb_has_device; then
    use_windows_adb_if_available
  fi

  if ! adb_has_device; then
    adb devices -l 2>/dev/null || true
    die "HP Android belum terdeteksi. Aktifkan USB debugging, terima prompt RSA di HP, lalu jalankan ulang."
  fi

  select_android_device
}

backend_healthy() {
  curl -fsS "http://127.0.0.1:$BACKEND_PORT/health" 2>/dev/null \
    | grep -q '"status":"ok"'
}

port_is_listening() {
  ss -ltn "( sport = :$BACKEND_PORT )" 2>/dev/null | grep -q LISTEN
}

start_infra() {
  if [ "${JALANAMAN_SKIP_INFRA:-0}" = "1" ]; then
    log infra "Skipping docker infra"
    return
  fi

  if ! command -v docker >/dev/null 2>&1; then
    warn "docker tidak ditemukan; backend akan memakai database/redis yang sudah ada."
    return
  fi

  log infra "Starting postgres + redis"
  if ! (cd "$ROOT_DIR" && docker compose up postgres redis -d); then
    warn "docker compose gagal; lanjut mencoba backend lokal."
  fi
}

wait_for_backend() {
  local seconds="${JALANAMAN_BACKEND_WAIT_SECONDS:-90}"
  local i

  for i in $(seq 1 "$seconds"); do
    if backend_healthy; then
      log backend "Ready at http://127.0.0.1:$BACKEND_PORT"
      return
    fi

    if [ -n "$BACKEND_PID" ] && ! kill -0 "$BACKEND_PID" >/dev/null 2>&1; then
      warn "Backend berhenti sebelum siap. Log terakhir:"
      tail -n 80 "$BACKEND_LOG" >&2 || true
      exit 1
    fi

    sleep 1
  done

  warn "Backend belum siap setelah ${seconds}s. Log terakhir:"
  tail -n 80 "$BACKEND_LOG" >&2 || true
  exit 1
}

start_backend() {
  if [ "${JALANAMAN_SKIP_BACKEND:-0}" = "1" ]; then
    log backend "Skipping backend"
    return
  fi

  if backend_healthy; then
    log backend "Reusing backend at http://127.0.0.1:$BACKEND_PORT"
    return
  fi

  if port_is_listening; then
    die "Port $BACKEND_PORT sudah dipakai, tapi /health bukan backend JalanAman. Stop proses itu atau set PORT lain."
  fi

  start_infra
  log backend "Starting backend (log: $BACKEND_LOG)"
  (cd "$ROOT_DIR/backend" && cargo run) > "$BACKEND_LOG" 2>&1 &
  BACKEND_PID="$!"
  wait_for_backend
}

reverse_backend_port() {
  if adb reverse "tcp:$BACKEND_PORT" "tcp:$BACKEND_PORT" >/dev/null 2>&1; then
    log android "adb reverse tcp:$BACKEND_PORT -> tcp:$BACKEND_PORT"
  else
    warn "adb reverse gagal. Jika app tidak bisa akses backend, cek koneksi adb."
  fi
}

grant_dev_permissions() {
  local permission
  local op

  for permission in \
    android.permission.ACCESS_FINE_LOCATION \
    android.permission.ACCESS_COARSE_LOCATION \
    android.permission.POST_NOTIFICATIONS
  do
    adb shell pm clear-permission-flags "$PACKAGE_NAME" "$permission" user-set user-fixed >/dev/null 2>&1 || true
    adb shell pm grant "$PACKAGE_NAME" "$permission" >/dev/null 2>&1 || true
  done

  for op in FINE_LOCATION COARSE_LOCATION POST_NOTIFICATION; do
    adb shell appops set "$PACKAGE_NAME" "$op" allow >/dev/null 2>&1 || true
  done
}

install_and_launch_apk() {
  local apk_path="$MOBILE_DIR/target/dx/jalanaman_mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk"

  if [ ! -f "$apk_path" ]; then
    warn "APK debug tidak ditemukan: $apk_path"
    return 1
  fi

  log android "Installing APK to HP"
  adb install -r "$apk_path"
  grant_dev_permissions
  adb shell am force-stop "$PACKAGE_NAME" >/dev/null 2>&1 || true
  adb shell am start -n "$PACKAGE_NAME/$ACTIVITY_NAME" >/dev/null
  log android "JalanAman mobile updated and opened"
}

build_install_launch() {
  ensure_android_device
  reverse_backend_port

  log android "Building APK"
  if ! (cd "$MOBILE_DIR" && bash "$ANDROID_DIR/build-android-wsl.sh"); then
    warn "Build Android gagal. Watcher tetap hidup; save lagi setelah diperbaiki."
    return 1
  fi

  if ! install_and_launch_apk; then
    warn "Install/launch gagal. Watcher tetap hidup; cek koneksi HP lalu save lagi."
    return 1
  fi
}

watch_fingerprint() {
  local path

  for path in "${WATCH_PATHS[@]}"; do
    if [ -d "$path" ]; then
      find "$path" -type f \
        ! -path '*/target/*' \
        ! -path '*/.git/*' \
        -printf '%T@ %s %p\n'
    elif [ -f "$path" ]; then
      stat -c '%Y %s %n' "$path"
    fi
  done | sort | sha1sum | awk '{ print $1 }'
}

watch_with_inotify() {
  log watch "Watching mobile/shared files with inotifywait"
  while true; do
    inotifywait -q -r -e close_write,create,delete,move "${WATCH_PATHS[@]}" >/dev/null || true
    log watch "Change detected"
    build_install_launch || true
  done
}

watch_with_polling() {
  local previous
  local current

  previous="$(watch_fingerprint)"
  log watch "inotifywait not found; polling every ${POLL_INTERVAL}s"
  while true; do
    sleep "$POLL_INTERVAL"
    current="$(watch_fingerprint)"
    if [ "$current" != "$previous" ]; then
      log watch "Change detected"
      previous="$current"
      build_install_launch || true
      previous="$(watch_fingerprint)"
    fi
  done
}

main() {
  setup_android_env
  assert_android_env
  start_backend

  if [ "${JALANAMAN_ONCE:-0}" = "1" ]; then
    build_install_launch
    log watch "One-shot mode selesai"
    return
  fi

  build_install_launch || true
  log watch "Save file di frontend/mobile atau frontend/shared untuk auto update ke HP. Ctrl+C untuk stop."
  if command -v inotifywait >/dev/null 2>&1; then
    watch_with_inotify
  else
    watch_with_polling
  fi
}

main "$@"
