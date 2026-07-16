.PHONY: dev-infra dev-backend dev-web dev-android dev-ios dev-frontend up down logs db-migrate db-reset seed vapid-keys fmt check build-backend build-frontend build-android

MOBILE_GRADLE_HOME := $(CURDIR)/frontend/mobile/target/gradle-home
ANDROID_API ?= 23

# ── Dev mode (infra only, run backend locally) ────────────────────────────────
dev-infra:
	docker compose up postgres redis -d

dev-backend:
	cd backend && cargo watch -x run

# Build Tailwind lalu jalankan dx serve (web)
dev-web:
	cd frontend/web && npx tailwindcss -i assets/tailwind.css -o assets/tw.css && dx serve

# Mobile: pastikan Android SDK + NDK sudah terpasang
dev-android:
	cd frontend/mobile && GRADLE_USER_HOME=$(MOBILE_GRADLE_HOME) dx serve --platform android

dev-ios:
	cd frontend/mobile && dx serve --platform ios

# Alias lama
dev-frontend: dev-web

# ── Full Docker stack ─────────────────────────────────────────────────────────
up:
	docker compose up -d --build

down:
	docker compose down

logs:
	docker compose logs -f backend

# ── Database ──────────────────────────────────────────────────────────────────
db-migrate:
	cd backend && sqlx migrate run

db-reset:
	docker compose exec postgres psql -U jalanaman -c "DROP DATABASE IF EXISTS jalanaman; CREATE DATABASE jalanaman;"
	$(MAKE) db-migrate

seed:
	docker compose exec postgres psql -U jalanaman -d jalanaman -f /dev/stdin < scripts/seed_demo.sql

# ── Security keys ─────────────────────────────────────────────────────────────
vapid-keys:
	./scripts/generate_vapid_keys.sh

# ── Code quality ─────────────────────────────────────────────────────────────
fmt:
	cargo fmt --all
	cd frontend/mobile && cargo fmt

check:
	cargo clippy --workspace -- -D warnings
	cd frontend/mobile && CC_aarch64_linux_android="$${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android$(ANDROID_API)-clang" AR_aarch64_linux_android="$${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar" cargo check --target aarch64-linux-android

build-backend:
	cd backend && cargo build --release

build-frontend:
	cd frontend/web && npx tailwindcss -i assets/tailwind.css -o assets/tw.css --minify && dx build --release

build-android:
	cd frontend/mobile && GRADLE_USER_HOME=$(MOBILE_GRADLE_HOME) dx build --platform android
