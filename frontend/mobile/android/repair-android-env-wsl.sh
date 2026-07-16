#!/usr/bin/env bash
set -euo pipefail

SDK_ROOT="${ANDROID_HOME:-$HOME/Android/Sdk}"
DEFAULT_NDK_VERSION="${ANDROID_NDK_VERSION:-28.2.13676358}"
DX_VERSION="${DIOXUS_CLI_VERSION:-0.6.3}"
ENV_MARKER_START="# >>> jalanaman android sdk >>>"
ENV_MARKER_END="# <<< jalanaman android sdk <<<"

if [ ! -d "$SDK_ROOT" ]; then
  echo "Android SDK not found in $SDK_ROOT" >&2
  echo "Run: bash android/install-android-sdk-wsl.sh" >&2
  exit 1
fi

if [ -d "$SDK_ROOT/ndk/$DEFAULT_NDK_VERSION" ]; then
  NDK_VERSION="$DEFAULT_NDK_VERSION"
else
  NDK_VERSION="$(find "$SDK_ROOT/ndk" -mindepth 1 -maxdepth 1 -type d -printf '%f\n' 2>/dev/null | sort -V | tail -n 1 || true)"
fi

if [ -z "${NDK_VERSION:-}" ]; then
  echo "NDK not found in $SDK_ROOT/ndk" >&2
  exit 1
fi

ANDROID_NDK_HOME="$SDK_ROOT/ndk/$NDK_VERSION"
if [ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" ]; then
  echo "NDK exists, but Linux toolchain was not found:" >&2
  echo "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64" >&2
  exit 1
fi

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

export ANDROID_HOME="$SDK_ROOT"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"

if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup not found in PATH. Check Rust installation in WSL." >&2
  exit 1
fi

echo "==> Installing Android Rust targets if missing"
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  i686-linux-android \
  x86_64-linux-android

if ! command -v dx >/dev/null 2>&1; then
  echo "dx not found. Install with:" >&2
  echo "cargo install dioxus-cli --version $DX_VERSION --locked --force --jobs 1" >&2
  exit 1
fi

if ! dx --version | grep -q "$DX_VERSION"; then
  echo "dx exists, but not version $DX_VERSION:" >&2
  dx --version >&2
  echo "Install again with:" >&2
  echo "cargo install dioxus-cli --version $DX_VERSION --locked --force --jobs 1" >&2
  exit 1
fi

echo "==> Android WSL environment ready"
echo "ANDROID_HOME=$ANDROID_HOME"
echo "ANDROID_NDK_HOME=$ANDROID_NDK_HOME"
echo "dx=$(dx --version)"
echo "rustc=$(rustc -V)"
echo
echo "Run: source ~/.bashrc"
echo "Then: cd frontend/mobile"
echo "And: dx serve"
