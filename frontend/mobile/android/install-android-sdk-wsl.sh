#!/usr/bin/env bash
set -euo pipefail

SDK_ROOT="${ANDROID_HOME:-$HOME/Android/Sdk}"
CMDLINE_TOOLS_VERSION="${ANDROID_CMDLINE_TOOLS_VERSION:-14742923}"
NDK_VERSION="${ANDROID_NDK_VERSION:-28.2.13676358}"
ANDROID_PLATFORM="${ANDROID_PLATFORM:-android-33}"
BUILD_TOOLS_VERSION="${ANDROID_BUILD_TOOLS_VERSION:-33.0.2}"
DX_VERSION="${DIOXUS_CLI_VERSION:-0.6.3}"
PROJECT_CACHE="${JALANAMAN_CACHE_DIR:-$HOME/.cache/jalanaman}"
TOOLS_ZIP="commandlinetools-linux-${CMDLINE_TOOLS_VERSION}_latest.zip"
TOOLS_URL="https://dl.google.com/android/repository/${TOOLS_ZIP}"
SDKMANAGER="$SDK_ROOT/cmdline-tools/latest/bin/sdkmanager"
ENV_MARKER_START="# >>> jalanaman android sdk >>>"
ENV_MARKER_END="# <<< jalanaman android sdk <<<"

write_bashrc_env() {
  local tmp_bashrc

  echo "==> Writing Android env vars to ~/.bashrc"
  tmp_bashrc="$(mktemp)"
  touch "$HOME/.bashrc"
  sed "/$ENV_MARKER_START/,/$ENV_MARKER_END/d" "$HOME/.bashrc" > "$tmp_bashrc"
  cat >> "$tmp_bashrc" <<EOF
$ENV_MARKER_START
export ANDROID_HOME="$SDK_ROOT"
export ANDROID_SDK_ROOT="\$ANDROID_HOME"
export ANDROID_NDK_HOME="\$ANDROID_HOME/ndk/$NDK_VERSION"
export PATH="\$HOME/.cargo/bin:\$ANDROID_HOME/cmdline-tools/latest/bin:\$ANDROID_HOME/platform-tools:\$ANDROID_HOME/emulator:\$PATH"

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

SDK_READY=false
if [ -x "$SDKMANAGER" ] \
  && [ -x "$SDK_ROOT/platform-tools/adb" ] \
  && [ -d "$SDK_ROOT/ndk/$NDK_VERSION/toolchains/llvm/prebuilt/linux-x86_64" ]; then
  SDK_READY=true
fi

if [ "$SDK_READY" = true ]; then
  echo "==> Android SDK/NDK already exists, skipping apt-get and sdkmanager install."
else
  echo "==> Installing Linux packages"
  if command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get install -y build-essential cmake curl ninja-build openjdk-17-jdk perl pkg-config unzip
  else
    echo "apt-get not found. Install manually: build-essential, cmake, curl, ninja-build, OpenJDK 17, perl, pkg-config, unzip." >&2
  fi

  echo "==> Installing Android command-line tools in $SDK_ROOT"
  mkdir -p "$SDK_ROOT/cmdline-tools"

  if [ ! -x "$SDKMANAGER" ]; then
    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "$tmp_dir"' EXIT

    curl -fL "$TOOLS_URL" -o "$tmp_dir/$TOOLS_ZIP"
    unzip -q "$tmp_dir/$TOOLS_ZIP" -d "$tmp_dir"
    rm -rf "$SDK_ROOT/cmdline-tools/latest"
    mkdir -p "$SDK_ROOT/cmdline-tools/latest"
    cp -a "$tmp_dir/cmdline-tools/." "$SDK_ROOT/cmdline-tools/latest/"
  fi

  echo "==> Accepting Android SDK licenses"
  set +o pipefail
  yes | "$SDKMANAGER" --sdk_root="$SDK_ROOT" --licenses >/dev/null
  set -o pipefail

  echo "==> Installing SDK platform, build tools, platform-tools, and NDK"
  "$SDKMANAGER" --sdk_root="$SDK_ROOT" \
    "platform-tools" \
    "platforms;$ANDROID_PLATFORM" \
    "build-tools;$BUILD_TOOLS_VERSION" \
    "ndk;$NDK_VERSION"
fi

ANDROID_NDK_HOME="$SDK_ROOT/ndk/$NDK_VERSION"
if [ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
  echo "NDK is installed, but linux-x86_64 toolchain was not found in $ANDROID_NDK_HOME" >&2
  exit 1
fi

write_bashrc_env

export ANDROID_HOME="$SDK_ROOT"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"

if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup not found in PATH. Install Rust in WSL first." >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found in PATH. Install Rust in WSL first." >&2
  exit 1
fi

echo "==> Installing Rust Android targets"
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  i686-linux-android \
  x86_64-linux-android

echo "==> Installing compatible Dioxus CLI"
if dx --version 2>/dev/null | grep -q "$DX_VERSION"; then
  echo "dx $DX_VERSION already installed, skipping cargo install."
else
  mkdir -p "$PROJECT_CACHE/tmp" "$PROJECT_CACHE/cargo-install-target"
  export TMPDIR="$PROJECT_CACHE/tmp"
  export CARGO_TARGET_DIR="$PROJECT_CACHE/cargo-install-target"
  export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
  cargo install dioxus-cli --version "$DX_VERSION" --locked --force --jobs 1
fi

echo "==> Done"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "ANDROID_NDK_HOME=$ANDROID_NDK_HOME"
echo "dx version: $(dx --version)"
echo
echo "Restart terminal or run: source ~/.bashrc"
echo "Then from frontend/mobile run: dx serve"
