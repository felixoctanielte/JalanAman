#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${PORT:-8080}"
ADDR="${ADDR:-127.0.0.1}"
URL="${JALANAMAN_WEB_URL:-http://localhost:$PORT}"
OPEN_DELAY_SECONDS="${OPEN_DELAY_SECONDS:-3}"

open_chrome_after_delay() {
  (
    sleep "$OPEN_DELAY_SECONDS"

    if command -v powershell.exe >/dev/null 2>&1; then
      powershell.exe -NoProfile -Command "Start-Process chrome '$URL'" >/dev/null 2>&1 \
        || powershell.exe -NoProfile -Command "Start-Process '$URL'" >/dev/null 2>&1 \
        || true
      exit 0
    fi

    if command -v cmd.exe >/dev/null 2>&1; then
      cmd.exe /C start chrome "$URL" >/dev/null 2>&1 \
        || cmd.exe /C start "" "$URL" >/dev/null 2>&1 \
        || true
      exit 0
    fi

    if command -v google-chrome >/dev/null 2>&1; then
      google-chrome "$URL" >/dev/null 2>&1 || true
      exit 0
    fi

    if command -v xdg-open >/dev/null 2>&1; then
      xdg-open "$URL" >/dev/null 2>&1 || true
    fi
  ) &
}

cd "$SCRIPT_DIR"
npm run tailwind:build

echo "Opening Chrome at $URL"
open_chrome_after_delay

exec dx serve --addr "$ADDR" --port "$PORT" --open false "$@"
