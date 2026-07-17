# JalanAman

**Garuda Hacks 7.0 - Track: Safety & Resilience**

> Know which route is safer before you move.

JalanAman is a mobile-first safety app and web dashboard powered by crowd-sourced community reports. It helps pedestrians and public transport users in Indonesia understand route safety, report nearby risks quickly, and trigger a one-tap SOS flow with a loud local alarm plus emergency notifications through push and email.

---

## MVP Features

| Feature | Description |
|---|---|
| Report Map | Community report pins for poor lighting, crime-prone areas, and accidents near the user. The web app uses Leaflet + OpenStreetMap. |
| Quick Report | One tap report flow: choose a category, attach current geolocation, and submit without an account. |
| Route Score | Enter a destination, calculate a route through OSRM, then display a safety level: Safe, Caution, or Avoid. |
| SOS Button | Loud local alarm, vibration, emergency contact notification, WhatsApp fallback, and SOS email delivery. |
| Dashboard | Area-based report visibility for neighborhood admins, campus security, and local safety teams. |

### Known Limitations

- Emergency push notification is best-effort and depends on the receiver's device settings, notification permissions, and network state. It is not a critical alert that bypasses Do Not Disturb.
- Emergency contacts must open the invite link and allow notifications before SOS is needed.
- The sender device always plays the local alarm without requiring an internet connection.
- Email-based alarms require an automation rule on the contact's phone, such as Tasker or MacroDroid. Use a Gmail notification trigger that contains `[JALANAMAN-SOS]`, then configure the phone to play an alarm, wake the screen, and repeat vibration. The email body also contains `JALANAMAN_SOS_TRIGGER=1` for easier filtering.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust, Axum 0.7, Tokio |
| Web Frontend | Dioxus 0.6 WASM PWA, `dx serve` |
| Mobile Frontend | Dioxus 0.6 Android/iOS |
| CSS | Tailwind CSS v3, built through npm |
| Database | PostgreSQL 16 with `earthdistance` and `cube` extensions |
| Cache | Redis 7 |
| Map and Routing | Leaflet 1.9.4, OpenStreetMap, OSRM, Nominatim |
| Push | Web Push / VAPID through the `web-push` crate |
| SOS Email | SMTP through `lettre`, tested with Gmail App Password |
| Container | Docker and Docker Compose |
| Reverse Proxy | nginx |

No paid map API key is required for the default map, routing, and geocoding stack.

---

## Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Dioxus CLI 0.6.x, matching this project
cargo install dioxus-cli --version 0.6.3 --locked

# sqlx CLI for database migrations
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# cargo-watch for backend hot reload
cargo install cargo-watch

# Node.js >= 18 for Tailwind builds
node --version
cd frontend/web && npm ci && cd ../..

# Docker and Docker Compose
docker --version
```

For Android development in WSL, see [frontend/mobile/android/README.md](frontend/mobile/android/README.md).

---

## Setup

### 1. Clone and configure environment

```bash
git clone https://github.com/YOUR_USERNAME/JalanAman
cd JalanAman
cp .env.example .env
```

Edit `.env` and fill in the VAPID keys and SMTP credentials.

### 2. Generate VAPID keys

```bash
./scripts/generate_vapid_keys.sh
```

Copy `VAPID_PUBLIC_KEY` and `VAPID_PRIVATE_KEY_PEM` into `.env`.

### 3. Start local infrastructure

```bash
make dev-infra
cd backend && sqlx migrate run
```

### 4. Run web development mode

Use two terminals:

```bash
# Terminal 1 - backend with hot reload
make dev-backend

# Terminal 2 - web frontend
make dev-web
```

The backend runs on `http://127.0.0.1:8080`. The web wrapper builds Tailwind CSS and starts Dioxus.

### 5. Run Android development mode

For the best physical-device workflow, connect the phone with USB debugging enabled, then run:

```bash
make dev-android-auto
```

This command will:

- start or reuse the backend on `http://127.0.0.1:8080`
- start PostgreSQL and Redis through Docker when needed
- apply `adb reverse tcp:8080 tcp:8080`
- build, install, and open the Android APK on the phone
- watch `frontend/mobile` and `frontend/shared`
- rebuild and reinstall the APK automatically whenever you save a watched file

For a single Android update without the watcher:

```bash
JALANAMAN_ONCE=1 make dev-android-auto
```

The older one-shot Android wrapper is still available:

```bash
make dev-android
```

### 6. Run the full Docker stack

```bash
docker compose up -d --build
```

Open `http://localhost`.

### 7. Seed demo data

```bash
make seed
```

---

## Environment Variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string |
| `REDIS_URL` | Redis connection string |
| `PORT` | Backend port, defaults to `8080` |
| `VAPID_PUBLIC_KEY` | Base64url public key for Web Push |
| `VAPID_PRIVATE_KEY_PEM` | PEM private key for Web Push |
| `GOOGLE_MAPS_API_KEY` | Optional Google Maps key for features that need it |
| `FRONTEND_URL` | Public frontend URL used by invite and notification flows |
| `SMTP_HOST` / `SMTP_PORT` | SMTP server, defaults to `smtp.gmail.com:587` |
| `SMTP_USER` / `SMTP_PASS` | SMTP credentials. For Gmail, use an App Password. |
| `SMTP_FROM` | Sender identity for SOS emails |

When values contain spaces or angle brackets, wrap them in quotes inside `.env`, for example:

```bash
SMTP_FROM="JalanAman SOS <jalanaman@example.com>"
```

---

## REST API

```text
GET  /health

# Safety reports
POST /api/reports
GET  /api/reports?lat=&lng=&radius=
POST /api/reports/:id/upvote
POST /api/reports/:id/downvote

# Directions and route scoring
GET  /api/directions
GET  /api/places
POST /api/route-score

# SOS and emergency contacts
POST   /api/sos/trigger
GET    /api/sos/contacts?device_hash=
POST   /api/sos/contacts
DELETE /api/sos/contacts/:id
GET    /api/sos/invite/:token
POST   /api/sos/subscribe

# Public frontend config
GET /api/config
```

---

## Anti-Abuse Design

- Report cooldown: one report per device in the same location every 10 minutes, backed by Redis TTL.
- Auto-hide: reports with at least three downvotes are hidden automatically.
- Report decay: report weight decreases linearly over 30 days.
- SOS rate limit: one SOS trigger per minute per device, backed by Redis TTL.
- Privacy: the backend stores a device hash rather than a real user name. SOS location is sent only to contacts registered by the user.

---

## Project Structure

```text
JalanAman/
|-- backend/                   Rust/Axum REST API
|   |-- src/
|   |   |-- main.rs
|   |   |-- config.rs
|   |   |-- routes.rs
|   |   |-- models/
|   |   `-- handlers/
|   `-- migrations/
|-- frontend/
|   |-- shared/                Shared Dioxus types and components
|   |-- web/                   Dioxus WASM PWA
|   `-- mobile/                Dioxus Android/iOS app
|       |-- src/
|       |-- assets/
|       |-- android/
|       `-- Dioxus.toml
|-- nginx/
|-- scripts/
|   |-- dev-android-auto.sh
|   |-- generate_vapid_keys.sh
|   `-- seed_demo.sql
|-- docker-compose.yml
|-- Makefile
`-- README.md
```

---

## AI Usage Disclosure

Parts of this project were assisted by AI during development:

- initial infrastructure scaffolding
- Axum handler boilerplate
- Dioxus component templates
- Service Worker and browser helper integration
- development workflow scripting and documentation cleanup

The main product behavior, route-safety logic, anti-abuse rules, emergency contact flow, and final verification are owned and reviewed by the team.

---

## Team

Add team member names here.

---

## License

MIT License.