#!/usr/bin/env bash

# Shared Android/WSL environment helpers for JalanAman mobile scripts.

JALANAMAN_ANDROID_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JALANAMAN_MOBILE_DIR="$(cd "$JALANAMAN_ANDROID_SCRIPT_DIR/.." && pwd)"

SDK_ROOT="${ANDROID_HOME:-$HOME/Android/Sdk}"
NDK_VERSION="${ANDROID_NDK_VERSION:-28.2.13676358}"
ANDROID_PLATFORM="${ANDROID_PLATFORM:-android-33}"
ANDROID_API_LEVEL="${ANDROID_API_LEVEL:-${ANDROID_PLATFORM#android-}}"
BUILD_TOOLS_VERSION="${ANDROID_BUILD_TOOLS_VERSION:-33.0.2}"
DX_VERSION="${DIOXUS_CLI_VERSION:-0.6.3}"
PROJECT_CACHE="${JALANAMAN_CACHE_DIR:-$HOME/.cache/jalanaman}"
CMDLINE_TOOLS_VERSION="${ANDROID_CMDLINE_TOOLS_VERSION:-14742923}"
TOOLS_ZIP="commandlinetools-linux-${CMDLINE_TOOLS_VERSION}_latest.zip"
TOOLS_URL="https://dl.google.com/android/repository/${TOOLS_ZIP}"
ENV_MARKER_START="# >>> jalanaman android sdk >>>"
ENV_MARKER_END="# <<< jalanaman android sdk <<<"

ANDROID_TARGETS=(
  aarch64-linux-android
  armv7-linux-androideabi
  i686-linux-android
  x86_64-linux-android
)

jalanaman_select_ndk_home() {
  local candidate

  if [ -n "${ANDROID_NDK_HOME:-}" ] \
    && [ -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
    return 0
  fi

  candidate="$ANDROID_HOME/ndk/$NDK_VERSION"
  if [ -d "$candidate/toolchains/llvm/prebuilt/linux-x86_64" ]; then
    export ANDROID_NDK_HOME="$candidate"
    return 0
  fi

  candidate="$(find "$ANDROID_HOME/ndk" -mindepth 1 -maxdepth 1 -type d -printf '%f\n' 2>/dev/null | sort -V | tail -n 1 || true)"
  if [ -n "$candidate" ] \
    && [ -d "$ANDROID_HOME/ndk/$candidate/toolchains/llvm/prebuilt/linux-x86_64" ]; then
    NDK_VERSION="$candidate"
    export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/$candidate"
    return 0
  fi

  export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
  return 1
}

jalanaman_export_android_toolchain() {
  local toolchain_bin="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
  local api_level="${ANDROID_API_LEVEL:-33}"

  if [ ! -d "$toolchain_bin" ]; then
    return 0
  fi

  export ANDROID_NDK_TOOLCHAIN_BIN="$toolchain_bin"
  export PATH="$ANDROID_NDK_TOOLCHAIN_BIN:$PATH"

  export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$toolchain_bin/aarch64-linux-android${api_level}-clang"
  export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$toolchain_bin/armv7a-linux-androideabi${api_level}-clang"
  export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="$toolchain_bin/i686-linux-android${api_level}-clang"
  export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$toolchain_bin/x86_64-linux-android${api_level}-clang"

  export CC_aarch64_linux_android="$CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER"
  export CXX_aarch64_linux_android="$toolchain_bin/aarch64-linux-android${api_level}-clang++"
  export CC_armv7_linux_androideabi="$CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER"
  export CXX_armv7_linux_androideabi="$toolchain_bin/armv7a-linux-androideabi${api_level}-clang++"
  export CC_i686_linux_android="$CARGO_TARGET_I686_LINUX_ANDROID_LINKER"
  export CXX_i686_linux_android="$toolchain_bin/i686-linux-android${api_level}-clang++"
  export CC_x86_64_linux_android="$CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER"
  export CXX_x86_64_linux_android="$toolchain_bin/x86_64-linux-android${api_level}-clang++"

  export AR_aarch64_linux_android="$toolchain_bin/llvm-ar"
  export AR_armv7_linux_androideabi="$toolchain_bin/llvm-ar"
  export AR_i686_linux_android="$toolchain_bin/llvm-ar"
  export AR_x86_64_linux_android="$toolchain_bin/llvm-ar"
  export RANLIB_aarch64_linux_android="$toolchain_bin/llvm-ranlib"
  export RANLIB_armv7_linux_androideabi="$toolchain_bin/llvm-ranlib"
  export RANLIB_i686_linux_android="$toolchain_bin/llvm-ranlib"
  export RANLIB_x86_64_linux_android="$toolchain_bin/llvm-ranlib"
}

jalanaman_apply_android_env() {
  export ANDROID_HOME="$SDK_ROOT"
  export ANDROID_SDK_ROOT="$ANDROID_HOME"
  export ANDROID_API_LEVEL="$ANDROID_API_LEVEL"
  export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
  export JALANAMAN_CACHE_DIR="$PROJECT_CACHE"
  export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PROJECT_CACHE/android-target}"
  export GRADLE_USER_HOME="${GRADLE_USER_HOME:-$PROJECT_CACHE/gradle}"
  export JALANAMAN_API_BASE_URL="${JALANAMAN_API_BASE_URL:-http://127.0.0.1:8080/api}"
  export SDKMANAGER="$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager"
  export ADB_WRAPPER_DIR="$PROJECT_CACHE/android-tools"

  mkdir -p "$PROJECT_CACHE"
  jalanaman_select_ndk_home >/dev/null 2>&1 || true
  jalanaman_export_android_toolchain
}

jalanaman_dx_bin() {
  if [ -x "$HOME/.cargo/bin/dx" ]; then
    printf '%s\n' "$HOME/.cargo/bin/dx"
    return 0
  fi

  command -v dx 2>/dev/null
}

jalanaman_has_platform_arg() {
  local arg
  for arg in "$@"; do
    case "$arg" in
      --platform|--platform=*) return 0 ;;
    esac
  done
  return 1
}

jalanaman_has_help_arg() {
  local arg
  for arg in "$@"; do
    case "$arg" in
      -h|--help) return 0 ;;
    esac
  done
  return 1
}

jalanaman_write_bashrc_env() {
  local tmp_bashrc
  local selected_ndk_version

  jalanaman_select_ndk_home >/dev/null 2>&1 || true
  selected_ndk_version="$(basename "${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/$NDK_VERSION}")"

  echo "==> Writing Android env vars to ~/.bashrc"
  tmp_bashrc="$(mktemp)"
  touch "$HOME/.bashrc"
  sed "/$ENV_MARKER_START/,/$ENV_MARKER_END/d" "$HOME/.bashrc" > "$tmp_bashrc"
  cat >> "$tmp_bashrc" <<EOF
$ENV_MARKER_START
export ANDROID_HOME="$ANDROID_HOME"
export ANDROID_SDK_ROOT="\$ANDROID_HOME"
export ANDROID_NDK_HOME="\$ANDROID_HOME/ndk/$selected_ndk_version"
export ANDROID_API_LEVEL="${ANDROID_API_LEVEL:-33}"
export ANDROID_NDK_TOOLCHAIN_BIN="\$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
export JALANAMAN_CACHE_DIR="\${JALANAMAN_CACHE_DIR:-$PROJECT_CACHE}"
export CARGO_TARGET_DIR="\${CARGO_TARGET_DIR:-\$JALANAMAN_CACHE_DIR/android-target}"
export GRADLE_USER_HOME="\${GRADLE_USER_HOME:-\$JALANAMAN_CACHE_DIR/gradle}"
export PATH="\$HOME/.cargo/bin:\$ANDROID_NDK_TOOLCHAIN_BIN:\$ANDROID_HOME/cmdline-tools/latest/bin:\$ANDROID_HOME/platform-tools:\$ANDROID_HOME/emulator:\$PATH"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="\$ANDROID_NDK_TOOLCHAIN_BIN/aarch64-linux-android\${ANDROID_API_LEVEL}-clang"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="\$ANDROID_NDK_TOOLCHAIN_BIN/armv7a-linux-androideabi\${ANDROID_API_LEVEL}-clang"
export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="\$ANDROID_NDK_TOOLCHAIN_BIN/i686-linux-android\${ANDROID_API_LEVEL}-clang"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="\$ANDROID_NDK_TOOLCHAIN_BIN/x86_64-linux-android\${ANDROID_API_LEVEL}-clang"
export CC_aarch64_linux_android="\$CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER"
export CXX_aarch64_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/aarch64-linux-android\${ANDROID_API_LEVEL}-clang++"
export CC_armv7_linux_androideabi="\$CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER"
export CXX_armv7_linux_androideabi="\$ANDROID_NDK_TOOLCHAIN_BIN/armv7a-linux-androideabi\${ANDROID_API_LEVEL}-clang++"
export CC_i686_linux_android="\$CARGO_TARGET_I686_LINUX_ANDROID_LINKER"
export CXX_i686_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/i686-linux-android\${ANDROID_API_LEVEL}-clang++"
export CC_x86_64_linux_android="\$CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER"
export CXX_x86_64_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/x86_64-linux-android\${ANDROID_API_LEVEL}-clang++"
export AR_aarch64_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ar"
export AR_armv7_linux_androideabi="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ar"
export AR_i686_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ar"
export AR_x86_64_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ar"
export RANLIB_aarch64_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ranlib"
export RANLIB_armv7_linux_androideabi="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ranlib"
export RANLIB_i686_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ranlib"
export RANLIB_x86_64_linux_android="\$ANDROID_NDK_TOOLCHAIN_BIN/llvm-ranlib"

_jalanaman_dx_has_platform_arg() {
  local arg
  for arg in "\$@"; do
    case "\$arg" in
      --platform|--platform=*) return 0 ;;
    esac
  done
  return 1
}

dx() {
  local real_dx="\$HOME/.cargo/bin/dx"
  if [ ! -x "\$real_dx" ]; then
    echo "dx binary not found at \$real_dx" >&2
    return 127
  fi

  if [ "\$#" -gt 0 ] \
    && { [ "\$1" = "serve" ] || [ "\$1" = "build" ]; } \
    && [ -f "./Dioxus.toml" ] \
    && grep -Eq 'name[[:space:]]*=[[:space:]]*"jalanaman-mobile"' "./Dioxus.toml" \
    && ! _jalanaman_dx_has_platform_arg "\$@"; then
    if [ "\$1" = "serve" ] && [ -x "./android/serve-android-wsl.sh" ]; then
      bash ./android/serve-android-wsl.sh "\${@:2}"
    elif [ "\$1" = "build" ] && [ -x "./android/build-android-wsl.sh" ]; then
      bash ./android/build-android-wsl.sh "\${@:2}"
    else
      "\$real_dx" "\$1" --platform android "\${@:2}"
    fi
  else
    "\$real_dx" "\$@"
  fi
}
$ENV_MARKER_END
EOF
  mv "$tmp_bashrc" "$HOME/.bashrc"
}

jalanaman_install_linux_packages_if_needed() {
  local missing=()

  command -v curl >/dev/null 2>&1 || missing+=("curl")
  command -v unzip >/dev/null 2>&1 || missing+=("unzip")
  command -v cmake >/dev/null 2>&1 || missing+=("cmake")
  command -v ninja >/dev/null 2>&1 || missing+=("ninja-build")
  command -v java >/dev/null 2>&1 || missing+=("openjdk-17-jdk")
  command -v perl >/dev/null 2>&1 || missing+=("perl")
  command -v pkg-config >/dev/null 2>&1 || missing+=("pkg-config")
  if command -v dpkg >/dev/null 2>&1 && ! dpkg -s build-essential >/dev/null 2>&1; then
    missing+=("build-essential")
  fi

  if [ "${#missing[@]}" -eq 0 ]; then
    echo "==> Linux packages already available."
    return 0
  fi

  if ! command -v apt-get >/dev/null 2>&1; then
    echo "Missing Linux packages: ${missing[*]}" >&2
    echo "Install them manually, then rerun this script." >&2
    return 1
  fi

  echo "==> Installing Linux packages: ${missing[*]}"
  if ! sudo -n true 2>/dev/null; then
    if [ ! -t 0 ]; then
      echo "Installing Linux packages requires an interactive sudo password prompt." >&2
      echo "Run this from your WSL terminal: bash android/install-android-sdk-wsl.sh" >&2
      return 1
    fi
    sudo -v
  fi

  sudo apt-get update
  sudo apt-get install -y build-essential cmake curl ninja-build openjdk-17-jdk perl pkg-config unzip
}

jalanaman_install_cmdline_tools_if_needed() {
  local tmp_dir

  if [ -x "$SDKMANAGER" ]; then
    echo "==> Android command-line tools already available."
    return 0
  fi

  echo "==> Installing Android command-line tools in $ANDROID_HOME"
  mkdir -p "$ANDROID_HOME/cmdline-tools"
  tmp_dir="$(mktemp -d)"

  curl -fL "$TOOLS_URL" -o "$tmp_dir/$TOOLS_ZIP"
  unzip -q "$tmp_dir/$TOOLS_ZIP" -d "$tmp_dir"
  rm -rf "$ANDROID_HOME/cmdline-tools/latest"
  mkdir -p "$ANDROID_HOME/cmdline-tools/latest"
  cp -a "$tmp_dir/cmdline-tools/." "$ANDROID_HOME/cmdline-tools/latest/"
  rm -rf "$tmp_dir"
}

jalanaman_install_android_sdk_packages() {
  if ! command -v java >/dev/null 2>&1; then
    echo "Java was not found. Run this script again after OpenJDK is installed." >&2
    return 1
  fi

  if [ ! -x "$SDKMANAGER" ]; then
    echo "sdkmanager was not found at $SDKMANAGER" >&2
    return 1
  fi

  echo "==> Accepting Android SDK licenses"
  set +o pipefail
  yes | "$SDKMANAGER" --sdk_root="$ANDROID_HOME" --licenses >/dev/null
  set -o pipefail

  echo "==> Installing SDK platform, build tools, platform-tools, and NDK"
  "$SDKMANAGER" --sdk_root="$ANDROID_HOME" \
    "platform-tools" \
    "platforms;$ANDROID_PLATFORM" \
    "build-tools;$BUILD_TOOLS_VERSION" \
    "ndk;$NDK_VERSION"

  jalanaman_select_ndk_home >/dev/null 2>&1 || true
}

jalanaman_ensure_rust_android_targets() {
  if ! command -v rustup >/dev/null 2>&1; then
    echo "rustup not found in PATH. Install Rust in WSL first." >&2
    return 1
  fi

  echo "==> Installing Rust Android targets if missing"
  rustup target add "${ANDROID_TARGETS[@]}"
}

jalanaman_ensure_dx_cli() {
  local dx_bin
  local previous_cargo_target_dir="${CARGO_TARGET_DIR:-}"

  if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found in PATH. Install Rust in WSL first." >&2
    return 1
  fi

  dx_bin="$(jalanaman_dx_bin || true)"
  if [ -n "$dx_bin" ] && "$dx_bin" --version 2>/dev/null | grep -q "$DX_VERSION"; then
    echo "==> dx $DX_VERSION already installed."
    return 0
  fi

  echo "==> Installing compatible Dioxus CLI $DX_VERSION"
  mkdir -p "$PROJECT_CACHE/tmp" "$PROJECT_CACHE/cargo-install-target"
  export TMPDIR="$PROJECT_CACHE/tmp"
  export CARGO_TARGET_DIR="$PROJECT_CACHE/cargo-install-target"
  export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
  cargo install dioxus-cli --version "$DX_VERSION" --locked --force --jobs 1
  export CARGO_TARGET_DIR="$previous_cargo_target_dir"
}

jalanaman_require_android_prereqs() {
  local missing=0
  local dx_bin
  local target

  if [ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
    echo "ANDROID_NDK_HOME is invalid: $ANDROID_NDK_HOME" >&2
    missing=1
  fi

  if ! command -v java >/dev/null 2>&1; then
    echo "Java/OpenJDK is missing." >&2
    missing=1
  fi

  if ! command -v adb >/dev/null 2>&1; then
    echo "adb is missing from PATH. Install Android platform-tools." >&2
    missing=1
  fi

  if ! command -v rustup >/dev/null 2>&1; then
    echo "rustup is missing from PATH." >&2
    missing=1
  else
    for target in "${ANDROID_TARGETS[@]}"; do
      if ! rustup target list --installed | grep -qx "$target"; then
        echo "Rust Android target is missing: $target" >&2
        missing=1
      fi
    done
  fi

  dx_bin="$(jalanaman_dx_bin || true)"
  if [ -z "$dx_bin" ]; then
    echo "dx is missing. Install dioxus-cli $DX_VERSION." >&2
    missing=1
  elif ! "$dx_bin" --version 2>/dev/null | grep -q "$DX_VERSION"; then
    echo "dx exists, but not version $DX_VERSION:" >&2
    "$dx_bin" --version >&2 || true
    missing=1
  fi

  if [ "$missing" -ne 0 ]; then
    echo >&2
    echo "Run from frontend/mobile: bash android/install-android-sdk-wsl.sh" >&2
    echo "If SDK is already installed, run: bash android/repair-android-env-wsl.sh" >&2
    return 1
  fi
}

jalanaman_detect_windows_user_from_path() {
  local path_after_users="${JALANAMAN_MOBILE_DIR#/mnt/c/Users/}"

  if [ "$path_after_users" != "$JALANAMAN_MOBILE_DIR" ]; then
    printf '%s' "${path_after_users%%/*}"
  fi
}

jalanaman_detect_windows_user_from_cmd() {
  if ! command -v cmd.exe >/dev/null 2>&1; then
    return 0
  fi

  cmd.exe /c "echo %USERNAME%" 2>/dev/null | tr -d '\r' | tail -n 1 || true
}

jalanaman_detect_windows_user() {
  local windows_user

  windows_user="$(jalanaman_detect_windows_user_from_path)"
  if [ -n "$windows_user" ]; then
    printf '%s' "$windows_user"
    return
  fi

  windows_user="$(jalanaman_detect_windows_user_from_cmd)"
  if [ -n "$windows_user" ] && [ "$windows_user" != "%USERNAME%" ]; then
    printf '%s' "$windows_user"
  fi
}

jalanaman_use_windows_adb_if_available() {
  local windows_user
  local windows_adb

  windows_user="${WINDOWS_USER:-$(jalanaman_detect_windows_user)}"
  if [ -z "$windows_user" ]; then
    return 0
  fi

  windows_adb="/mnt/c/Users/$windows_user/AppData/Local/Android/Sdk/platform-tools/adb.exe"
  if [ ! -x "$windows_adb" ]; then
    return 0
  fi

  if [ "${JALANAMAN_USE_WSL_ADB:-}" = "1" ]; then
    return 0
  fi

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
}

jalanaman_print_android_summary() {
  local dx_bin

  dx_bin="$(jalanaman_dx_bin || true)"
  echo "ANDROID_HOME=$ANDROID_HOME"
  echo "ANDROID_NDK_HOME=$ANDROID_NDK_HOME"
  echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
  echo "GRADLE_USER_HOME=$GRADLE_USER_HOME"
  if [ -n "$dx_bin" ]; then
    echo "dx=$("$dx_bin" --version 2>/dev/null || true)"
  fi
  if command -v rustc >/dev/null 2>&1; then
    echo "rustc=$(rustc -V)"
  fi
}

jalanaman_android_build_profile() {
  local arg
  for arg in "$@"; do
    case "$arg" in
      -r|--release) printf '%s\n' "release"; return 0 ;;
    esac
  done
  printf '%s\n' "debug"
}

jalanaman_clean_generated_gradle_state() {
  local profile="${1:-debug}"
  local android_app_dir="$JALANAMAN_MOBILE_DIR/target/dx/jalanaman_mobile/$profile/android/app"

  if [ -x "$android_app_dir/gradlew" ]; then
    (cd "$android_app_dir" && ./gradlew --stop >/dev/null 2>&1 || true)
  fi

  find "$android_app_dir/app/build" -depth -mindepth 1 -delete 2>/dev/null || true
  find "$android_app_dir/.gradle" -depth -mindepth 1 -delete 2>/dev/null || true
}

jalanaman_prepare_dx_cache_dir() {
  local target_dir="$JALANAMAN_MOBILE_DIR/target"
  local dx_path="$target_dir/dx"
  local cache_dx="$PROJECT_CACHE/dx-target"
  local current_target

  mkdir -p "$target_dir" "$cache_dx"

  if [ -L "$dx_path" ]; then
    current_target="$(readlink "$dx_path")"
    if [ "$current_target" = "$cache_dx" ]; then
      return 0
    fi
    unlink "$dx_path"
  fi

  if [ -e "$dx_path" ]; then
    echo "Moving generated Dioxus Android target off /mnt/c to avoid Gradle file locks."
    find "$dx_path" -depth -mindepth 1 -delete 2>/dev/null || true
    rmdir "$dx_path" 2>/dev/null || true
  fi

  if [ -e "$dx_path" ]; then
    echo "Could not replace $dx_path with a cache symlink." >&2
    echo "Close any running dx/Gradle process and retry." >&2
    return 1
  fi

  ln -s "$cache_dx" "$dx_path"
}

jalanaman_android_app_dir() {
  local profile="${1:-debug}"

  printf '%s\n' "$JALANAMAN_MOBILE_DIR/target/dx/jalanaman_mobile/$profile/android/app"
}

jalanaman_find_android_apk() {
  local profile="${1:-debug}"
  local android_app_dir
  local preferred_apk
  local apk_root

  android_app_dir="$(jalanaman_android_app_dir "$profile")"
  preferred_apk="$android_app_dir/app/build/outputs/apk/$profile/app-$profile.apk"
  apk_root="$android_app_dir/app/build/outputs/apk"

  if [ -f "$preferred_apk" ]; then
    printf '%s\n' "$preferred_apk"
    return 0
  fi

  find "$apk_root" -type f -name '*.apk' 2>/dev/null | sort | tail -n 1
}

jalanaman_android_application_id() {
  local profile="${1:-debug}"
  local android_app_dir
  local gradle_file
  local manifest
  local app_id

  android_app_dir="$(jalanaman_android_app_dir "$profile")"
  gradle_file="$android_app_dir/app/build.gradle.kts"

  if [ -f "$gradle_file" ]; then
    app_id="$(awk -F '"' '/applicationId[[:space:]]*=/ { print $2; exit }' "$gradle_file")"
    if [ -n "$app_id" ]; then
      printf '%s\n' "$app_id"
      return 0
    fi
  fi

  for manifest in \
    "$android_app_dir/app/build/intermediates/merged_manifest/$profile/process${profile^}MainManifest/AndroidManifest.xml" \
    "$android_app_dir/app/build/intermediates/merged_manifests/$profile/process${profile^}Manifest/AndroidManifest.xml" \
    "$android_app_dir/app/build/intermediates/packaged_manifests/$profile/process${profile^}ManifestForPackage/AndroidManifest.xml"
  do
    if [ -f "$manifest" ]; then
      awk -F '"' '/package=/ { print $2; exit }' "$manifest"
      return 0
    fi
  done
}

jalanaman_adb_reverse_backend() {
  if command -v adb >/dev/null 2>&1; then
    adb reverse tcp:8080 tcp:8080 >/dev/null 2>&1 || true
  fi
}

jalanaman_install_and_launch_android_apk() {
  local profile="${1:-debug}"
  local apk
  local app_id

  apk="$(jalanaman_find_android_apk "$profile")"
  if [ -z "$apk" ]; then
    echo "Android APK was not found for profile: $profile" >&2
    echo "Expected it below: $(jalanaman_android_app_dir "$profile")/app/build/outputs/apk" >&2
    return 1
  fi

  app_id="$(jalanaman_android_application_id "$profile")"
  if [ -z "$app_id" ]; then
    echo "Could not detect Android applicationId from generated Gradle project." >&2
    return 1
  fi

  jalanaman_adb_reverse_backend
  echo "Installing Android APK to connected device: $apk"
  adb install -r "$apk"
  echo "Launching Android app: $app_id"
  adb shell monkey -p "$app_id" -c android.intent.category.LAUNCHER 1 >/dev/null
}

jalanaman_apply_android_env
