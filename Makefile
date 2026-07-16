.PHONY: dev up down logs db-reset seed vapid-keys fmt check

# ── Dev mode (infra only, run backend locally) ────────────────────────────────
dev-infra:
	docker compose up postgres redis -d

dev-backend:
	cd backend && cargo watch -x run

dev-frontend:
	cd frontend && trunk serve --proxy-backend http://localhost:8080/api

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
	cd backend && cargo fmt
	cd frontend && cargo fmt

check:
	cd backend && cargo clippy -- -D warnings
	cd frontend && cargo clippy -- -D warnings

build-backend:
	cd backend && cargo build --release

build-frontend:
	cd frontend && trunk build --release
