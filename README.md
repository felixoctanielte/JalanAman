# JalanAman

**Garuda Hacks 7.0 — Track: Safety & Resilience**

<<<<<<< HEAD
> "Google Maps, but it knows which route is safe."

A mobile PWA + web dashboard built on community-sourced safety reports, helping pedestrians and public-transit users in Indonesia know how safe a given route is — with a one-tap SOS button that sounds a loud local alarm and notifies emergency contacts by WhatsApp and email.

No login required. Identity is a local `device_hash`, generated and stored on-device — there is no account system and no profile screen.
=======
> "Tahu mana yang aman sebelum melangkah"

Aplikasi mobile PWA + dashboard web berbasis laporan komunitas (*crowd-sourced*) yang membantu pejalan kaki dan pengguna transportasi umum di Indonesia mengetahui tingkat keamanan suatu rute, lengkap dengan tombol SOS satu-tap yang membunyikan alarm keras di device dan mengirim Web Push + email ke kontak darurat.
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3

---

## MVP Features

| Feature | Description |
|---|---|
<<<<<<< HEAD
| 🗺 **Safety map** | Community-submitted pins (poor lighting / crime-prone / accident-prone / other) shown within a radius around the user |
| ⚠️ **Quick report** | One tap → pick a category → submit with automatic geolocation, no account needed |
| 📊 **Route score** | Enter a destination → overlays an **Aman (safe) / Waspada (caution) / Hindari (avoid)** score on top of Google Directions results; picks the best-scoring route among the alternatives Directions already returns |
| 🆘 **SOS button** | Tap → 3-second cancellable countdown → loud local alarm + vibration (instant, works offline) + automatic email to emergency contacts + one-tap pre-filled WhatsApp message |
| 🏘 **Web dashboard** | Read-only heatmap of reports per area, for RT/RW (neighborhood watch), campus security, or local police |
=======
| 🗺 **Peta Laporan** | Pin report komunitas (pencahayaan buruk / rawan begal / kecelakaan) dalam radius 500m — Leaflet + OpenStreetMap, gratis |
| ⚠️ **Lapor Cepat** | 1 tap → pilih kategori → kirim dengan geolokasi otomatis, tanpa akun |
| 📊 **Skor Rute** | Input tujuan → routing via OSRM (gratis) → overlay skor **Aman / Waspada / Hindari** |
| 🆘 **Tombol SOS** | Alarm suara keras + vibrate lokal (instan) + Web Push + email ke kontak darurat |
| 🏘 **Dashboard** | Heatmap laporan per wilayah untuk RT/RW, satpam kampus, kepolisian |
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3

### Known limitations (disclosed by design)

<<<<<<< HEAD
- **Local alarm is the only truly real-time, guaranteed channel** — it doesn't depend on network access or on anyone else's device/settings.
- **Email** to emergency contacts is sent automatically from the backend (no tap required), but whether the recipient notices it immediately depends on their phone's mail-app notification settings. It is not a guaranteed real-time channel.
- **WhatsApp** messages are pre-filled with the location link, but WhatsApp does not allow fully silent, unattended sending — the sender still has to tap "Send" once per contact. This is a WhatsApp platform restriction, not a bug.
- We do **not** attempt to bypass a recipient's silent/Do-Not-Disturb mode. That kind of OS-level "critical alert" requires a special entitlement from Apple (restricted, approval-only) and is not something a third-party app can obtain for a hackathon project. Framing this honestly is intentional.
- Emergency contacts do **not** need to install anything or grant any permission ahead of time — they only need a valid WhatsApp number or email address.
=======
- Push notification ke kontak darurat adalah **best-effort**: tunduk pada setting notifikasi perangkat penerima dan status koneksi. Bukan *critical alert* yang bypass DND.
- Kontak darurat harus membuka link undangan dan mengizinkan notifikasi **sebelum** SOS diperlukan.
- Alarm suara **lokal** di device pengirim selalu bunyi — tidak butuh koneksi internet.
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3

---

## Tech Stack

| Layer | Technology |
|---|---|
<<<<<<< HEAD
| Backend | Rust (Axum 0.7 + Tokio) |
| Frontend | Dioxus 0.5 (WASM PWA) + Trunk |
| Database | PostgreSQL 16 + `earthdistance` extension |
| Cache | Redis 7 (optional — used for report cooldown / SOS rate-limit TTLs) |
| Maps | Google Maps JavaScript API (map + heatmap) + Directions API (route + polyline) |
| SOS – Email | Resend / SendGrid (free-tier REST API), called from the backend |
| SOS – WhatsApp | `wa.me` deep link, built client-side — no library or paid API needed |
=======
| Backend | Rust · Axum 0.7 · Tokio |
| Frontend Web | Dioxus 0.6 (WASM PWA) · `dx serve` |
| Frontend Mobile | Dioxus 0.6 · `dx serve --platform android/ios` |
| CSS | Tailwind CSS v3 (build via npm) |
| Database | PostgreSQL 16 · `earthdistance` + `cube` extensions |
| Cache | Redis 7 |
| Peta & Routing | Leaflet 1.9.4 · OpenStreetMap · OSRM · Nominatim — **gratis, tanpa API key** |
| Push | Web Push / VAPID (`web-push` crate) |
| Email SOS | SMTP via `lettre` (Gmail App Password) |
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3
| Container | Docker & Docker Compose |
| Reverse proxy | nginx |

**Scope note:** Dioxus targets **Web (WASM) as an installable PWA**, not native iOS/Android builds — this avoids the immature native mobile toolchain in Dioxus and keeps the 30-hour build loop fast. "Add to Home Screen" covers the "feels like a mobile app" requirement without that risk. REST is prioritized over gRPC/tonic for the same reason: keep the critical path simple.

---

## Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Dioxus CLI (pengganti Trunk)
cargo install dioxus-cli

<<<<<<< HEAD
# Docker & Docker Compose (already installed)
=======
# sqlx CLI (migrasi database)
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# cargo-watch (hot reload backend)
cargo install cargo-watch

# Node.js >= 18 (untuk build Tailwind CSS)
node --version

# Docker & Docker Compose
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3
docker --version
```

---

## Setup & Running

### 1. Clone & environment

```bash
git clone https://github.com/YOUR_USERNAME/JalanAman
cd JalanAman
cp .env.example .env
<<<<<<< HEAD
# Edit .env: fill in GOOGLE_MAPS_API_KEY and EMAIL_API_KEY (Resend or SendGrid)
```

`.env.example`:
```
DATABASE_URL=postgres://jalanaman:jalanaman@localhost:5432/jalanaman
REDIS_URL=redis://localhost:6379
GOOGLE_MAPS_API_KEY=
EMAIL_API_KEY=
EMAIL_FROM=alerts@jalanaman.app
=======
# Edit .env: isi VAPID keys, SMTP credentials
```

### 2. Generate VAPID keys

```bash
./scripts/generate_vapid_keys.sh
# Salin VAPID_PUBLIC_KEY dan VAPID_PRIVATE_KEY_PEM ke .env
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3
```

### 2. Start infrastructure (DB + Redis)

```bash
<<<<<<< HEAD
docker compose up postgres redis -d
# Wait until healthy, then:
cd backend && sqlx migrate run
```

### 3. Dev mode
=======
make dev-infra         # docker compose up postgres redis -d
cd backend && sqlx migrate run
```

### 4. Dev mode (3 terminal)
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3

```bash
# Terminal 1 – backend (hot reload)
make dev-backend       # cargo watch -x run di port 8080

<<<<<<< HEAD
# Terminal 2 – frontend
cd frontend && trunk serve
# Open http://localhost:3000
=======
# Terminal 2 – frontend web
make dev-web           # build Tailwind + dx serve di port 8080 (proxy ke backend)
# Buka http://localhost:8080

# Terminal 3 – mobile Android (opsional, butuh Android SDK)
make dev-android       # dx serve --platform android
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3
```

### 4. Full Docker stack

```bash
docker compose up -d --build
# Open http://localhost
```

### 5. Seed demo data

```bash
make seed
```

---

## Environment Variables

| Variabel | Keterangan |
|---|---|
| `DATABASE_URL` | Connection string PostgreSQL |
| `REDIS_URL` | Connection string Redis |
| `PORT` | Port backend (default `8080`) |
| `VAPID_PUBLIC_KEY` | Base64url public key untuk Web Push |
| `VAPID_PRIVATE_KEY_PEM` | PEM private key untuk Web Push |
| `SMTP_HOST` / `SMTP_PORT` | Server SMTP (default: `smtp.gmail.com:587`) |
| `SMTP_USER` / `SMTP_PASS` | Kredensial SMTP (Gmail: gunakan App Password) |
| `SMTP_FROM` | Alamat pengirim email SOS |

> **Tidak perlu API key peta** — peta, routing, dan geocoding menggunakan Leaflet + OpenStreetMap + OSRM (semua gratis).

---

## REST API

```
GET  /health
<<<<<<< HEAD
GET  /api/config                          → returns Maps API key for the frontend

POST /api/reports                         → create a new report
GET  /api/reports?lat=&lng=&radius=       → reports within radius (meters)
POST /api/reports/:id/upvote
POST /api/reports/:id/downvote

POST /api/route-score                     → compute route safety score
     body: { waypoints: [{lat, lng}] }
     returns the best-scoring route plus up to 2 alternatives, each with a score

POST /api/sos/trigger                     → trigger SOS
     body: { device_hash, lat, lng, timestamp }
     returns: { sos_id, maps_link, email_contacts_notified: [...], whatsapp_contacts: [...] }
POST /api/sos/resolve                     → mark an SOS as resolved ("I'm safe")
     body: { sos_id }

POST /api/contacts                        → add an emergency contact
     body: { device_hash, name, channel: "whatsapp" | "email", value }
GET  /api/contacts?device_hash=           → list emergency contacts for a device
DELETE /api/contacts/:id                  → remove an emergency contact
=======

# Laporan keamanan
POST /api/reports                       Buat laporan baru
GET  /api/reports?lat=&lng=&radius=     Laporan dalam radius (meter)
POST /api/reports/:id/upvote
POST /api/reports/:id/downvote

# Skor rute
POST /api/route-score                   Hitung skor keamanan rute
     body: { waypoints: [{lat, lng}] }
     → { level: "Aman"|"Waspada"|"Hindari", score: f64, ... }

# SOS & kontak darurat
POST   /api/sos/trigger                 Kirim SOS (alarm + push + email)
GET    /api/sos/contacts?device_hash=   Daftar kontak darurat
POST   /api/sos/contacts                Tambah kontak darurat
DELETE /api/sos/contacts/:id            Hapus kontak darurat
GET    /api/sos/invite/:token           Info undangan (publik)
POST   /api/sos/subscribe               Daftarkan push subscription kontak
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3
```

There is intentionally **no** invite/subscribe flow — earlier drafts of this project used Web Push (VAPID) subscriptions, which required the emergency contact to open the PWA and grant notification permission ahead of time. That was dropped in favor of the simpler email/WhatsApp model above, since it needs zero setup on the contact's side.

---

## Anti-Spam / Anti-Abuse Design

- **Report cooldown**: 1 report per device per 10 minutes at the same location (Redis TTL)
- **Auto-hide**: reports with ≥3 downvotes are automatically hidden
- **Report decay**: report weight decreases linearly over 30 days
- **SOS rate limit**: max 1 trigger per minute per device (Redis TTL, plus client-side debounce)
- **Contact ownership**: emergency contacts can only be added through the app by the owning `device_hash` — there is no public endpoint that lets anyone register a WhatsApp number or email as someone else's contact
- **Privacy**: no real names are stored beyond what the user types for a contact's display name; no login means no centrally stored personal profile; SOS location is only ever sent to contacts the user explicitly added themselves

---

## Design System (summary)

Two color modes, dark as default (primary use case is walking at night), light as an optional toggle. Full token table and screen-by-screen mockups live in the project design doc — key points for implementation:

- One color = one meaning everywhere: red = danger/SOS/avoid, amber = caution, green = safe. Never reused for anything else.
- Bottom navigation has **3 tabs only**: Map, Route, Contacts. No Profile tab — there's no login to have a profile of.
- The SOS button is **not** a tab. It's a persistent floating button, visible on every screen, bottom-center, so it's always in the same place during an emergency.
- Category and score labels use plain everyday words ("Gelap", "Rawan", "Kecelakaan" / "Aman", "Waspada", "Hindari") — no percentages or jargon in the UI.

---

## Project Structure

```
JalanAman/
├── backend/                   Rust/Axum REST API
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── routes.rs
<<<<<<< HEAD
│   │   ├── models/            # report.rs, emergency_contact.rs
│   │   ├── services/          # email.rs (Resend/SendGrid client), whatsapp.rs (wa.me link builder)
│   │   └── handlers/          # reports.rs, route_score.rs, sos.rs, contacts.rs, health.rs
│   └── migrations/
│       └── 001_init.sql
├── frontend/                  # Dioxus WASM PWA
│   ├── src/
│   │   ├── main.rs
│   │   ├── api.rs
│   │   ├── js.rs               # wasm_bindgen bindings to JS helpers
│   │   ├── types.rs
│   │   ├── components/         # map.rs, report_form.rs, sos_button.rs, sos_countdown.rs, route_score.rs
│   │   └── pages/              # map_page.rs, route_page.rs, contacts_page.rs, dashboard.rs
│   ├── public/
│   │   ├── sw.js                # Service Worker (offline caching / PWA installability only — no push)
│   │   └── manifest.json
│   └── index.html               # Entry point + Google Maps script + JS helpers (alarm, vibration)
=======
│   │   ├── models/            report.rs · emergency_contact.rs
│   │   └── handlers/          reports · route_score · sos · health
│   └── migrations/
│       └── 001_init.sql
├── frontend/
│   ├── shared/                Types & komponen bersama (Dioxus)
│   │   └── src/
│   │       ├── components/    report_form · route_score · sos_button
│   │       └── utils/types.rs
│   ├── web/                   PWA Web (dx serve → WASM)
│   │   ├── src/
│   │   │   ├── main.rs        Entry point dx
│   │   │   ├── app.rs         Router & routes
│   │   │   ├── pages/         home · dashboard · contacts · invite
│   │   │   ├── components/    map
│   │   │   ├── hooks/         use_geolocation
│   │   │   ├── services/      api · push
│   │   │   └── utils/         js · device
│   │   ├── public/
│   │   │   ├── sw.js          Service Worker (PWA + push)
│   │   │   └── manifest.json
│   │   ├── assets/
│   │   │   └── tailwind.css   Input Tailwind (→ tw.css saat build)
│   │   ├── index.html         Shell HTML + Leaflet CDN + JS helpers
│   │   └── Dioxus.toml
│   └── mobile/                Android/iOS (dx serve --platform android/ios)
│       ├── src/main.rs
│       └── Dioxus.toml
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3
├── nginx/nginx.conf
├── docker-compose.yml
├── Makefile
└── scripts/
    └── seed_demo.sql
```

---

## Database Schema (summary)

```sql
CREATE TYPE report_category AS ENUM ('lighting', 'crime', 'accident', 'other');
CREATE TYPE contact_channel AS ENUM ('whatsapp', 'email');

CREATE TABLE reports (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  category report_category NOT NULL,
  lat DOUBLE PRECISION NOT NULL,
  lng DOUBLE PRECISION NOT NULL,
  note VARCHAR(100),
  device_hash TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  upvote_count INT NOT NULL DEFAULT 0,
  downvote_count INT NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'active'
);

CREATE TABLE emergency_contacts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  device_hash TEXT NOT NULL,
  name TEXT NOT NULL,
  channel contact_channel NOT NULL,
  value TEXT NOT NULL, -- WhatsApp number (62xxx format) or email address
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Just two tables by design — no `users`/`roles`/`push_subscriptions` tables. Radius queries (map view, route scoring) use the `earthdistance`+`cube` extension rather than full PostGIS, since simple radius lookups are all this MVP needs.

---

## AI Usage Disclosure (Garuda Hacks)

As required by Garuda Hacks 7.0 rules, here is what AI (Claude) assisted with:

<<<<<<< HEAD
- **Initial project planning**: theme/track selection, problem scoping, feature prioritization for a 30-hour build window
- **Infrastructure scaffolding** (project structure, `Cargo.toml`, `docker-compose.yml`, initial migration SQL)
- **Axum handler boilerplate** (CRUD structure and error handling patterns)
- **Dioxus component boilerplate** (RSX structure, prop types)
- **Design system and screen mockups** (color tokens, navigation layout, SOS flow)
- **Service Worker and JS helper scaffolding** (Web Audio API, Vibration API, Geolocation)
=======
- **Scaffold infrastruktur awal** (struktur proyek, Cargo.toml, docker-compose.yml, migration SQL)
- **Template handler Axum** (boilerplate CRUD dan error handling)
- **Template komponen Dioxus** (struktur RSX, prop types)
- **Service Worker dan JS helper** (Web Push, Web Audio API, Geolocation, Leaflet integration)
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3

All core business logic — the route scoring algorithm, the anti-spam/anti-abuse mechanisms, and the SOS trigger flow — was designed by the team and implemented/verified by hand.

---

## Team

1. Farrel Nayaka
2. Felix Octaniel T
3. Aryabell Boston T
4. Ingatius Rayden I.R

---

## License

<<<<<<< HEAD
MIT License – see [LICENSE](LICENSE)
=======
MIT License — lihat [LICENSE](LICENSE)
>>>>>>> c45b07d9326ecbd6afc6a5268a8aed005ee968c3
